const assert = require("assert");
const anchor = require("@project-serum/anchor");
const chai = require("chai");
const expect = chai.expect;
chai.use(require("chai-as-promised"));
const { SystemProgram } = anchor.web3;

describe("increment-once", () => {
  /* create and set a Provider */
  const provider = anchor.Provider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.IncrementOnce;

  let counter, counterBump;
  before(async () => {
    [counter, counterBump] =
      await anchor.web3.PublicKey.findProgramAddress(
        [Buffer.from("counter")],
        program.programId
      );
  });

  async function incrementCounter(pubKey, signers, expectedCount) {
    const [hasIncremented, hasIncrementedBump] =
      await anchor.web3.PublicKey.findProgramAddress(
        [pubKey.toBuffer()],
        program.programId
      );

    await program.rpc.increment(new anchor.BN(hasIncrementedBump), {
      accounts: {
        counter,
        hasIncremented,
        user: pubKey,
        systemProgram: SystemProgram.programId,
      },
      // This is a bit tricky. It's not needed when using provider.wallet.publicKey.
      //
      // From Discord:
      // "You didn't have to explicitly sign for the wallet because it happened behind the scenes since the wallet pays for the tx"
      signers,
    });

    const account = await program.account.counter.fetch(counter);
    assert.ok(Number(account.count.toString()) == expectedCount);
  }

  it("Creates a counter", async () => {
    /* Call the create function via RPC */
    await program.rpc.create(new anchor.BN(counterBump), {
      accounts: {
        counter,
        user: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
      },
    });

    /* Fetch the account and check the value of count */
    const account = await program.account.counter.fetch(counter);
    assert.ok(account.count.toString() == 0);
  });

  it("Increments the counter", async () => {
    await incrementCounter(provider.wallet.publicKey, [], 1);

    // Incrementing with a different user (different public key) should succeed
    //
    // Need to request airdrop for secondary account, so that it has enough SOL
    // for the tx to succeed.
    const dummy = anchor.web3.Keypair.generate();
    const signature = await program.provider.connection.requestAirdrop(
      dummy.publicKey,
      anchor.web3.LAMPORTS_PER_SOL
    );
    await program.provider.connection.confirmTransaction(signature);
    await incrementCounter(dummy.publicKey, [dummy], 2);

    // Incrementing again should fail. Don't put this right after the first increment
    // for the wallet, because then it will fail with:
    // "This transaction has already been processed"
    // (because you're not supposed to execute the exact same tx twice in a row)
    await expect(
      incrementCounter(provider.wallet.publicKey, [], 0)
    ).to.be.rejectedWith(Error);
  });
});

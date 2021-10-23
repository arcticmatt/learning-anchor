const assert = require("assert");
const anchor = require("@project-serum/anchor");
const { SystemProgram } = anchor.web3;

describe("increment", () => {
  const provider = anchor.Provider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.Increment;

  let counter, counterBump;
  before(async () => {
    [counter, counterBump] =
      await anchor.web3.PublicKey.findProgramAddress(
        [Buffer.from("counter")],
        program.programId
      );
  });

  it("Creates a counter)", async () => {
    /* Call the create function via RPC */
    await program.rpc.create(new anchor.BN(counterBump), {
      accounts: {
        counter,
        user: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
      },
    });

    const account = await program.account.counter.fetch(counter);
    assert.ok(account.count.toString() == 0);

  });

  it("Increment once", async () => {
    await program.rpc.increment({
      accounts: {
        counter,
      },
    });

    const account = await program.account.counter.fetch(counter);
    assert.ok(account.count.toString() == 1);
    
    // TODO: test increment twice. Not sure how to get around "This transaction has already been processed"
  });
});
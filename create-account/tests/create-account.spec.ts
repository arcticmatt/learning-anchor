import {setProvider, Provider, workspace, web3} from "@project-serum/anchor";
import invariant from "invariant";

const {Keypair, SystemProgram, Transaction} = web3;

describe('create-account', () => {

  const provider = Provider.env();
  setProvider(provider);
  const program = workspace.CreateAccount;

  it("createAccount", async () => {
    const keypair1 = Keypair.generate();
    const keypair2 = Keypair.generate();
    const airdrop1 = await provider.connection.requestAirdrop(keypair1.publicKey, 100000000000);
    await provider.connection.confirmTransaction(airdrop1, 'finalized');
    console.log("keypair1", keypair1.publicKey.toString())
    console.log("keypair2", keypair2.publicKey.toString())

    const instruction = await SystemProgram.createAccount({
      fromPubkey: keypair1.publicKey,
      lamports: 10000000,
      programId: program.programId,
      space: 2000,
      newAccountPubkey: keypair2.publicKey,
    });

    let tx = new Transaction().add(instruction);
    tx.recentBlockhash = (await provider.connection.getRecentBlockhash()).blockhash;
    tx.feePayer = keypair1.publicKey;
    const sig = await provider.connection.sendTransaction(tx, [keypair1, keypair2], { skipPreflight: false, preflightCommitment: "confirmed" });
    await provider.connection.confirmTransaction(sig, 'finalized');

    const accountInfo = await provider.connection.getAccountInfo(keypair2.publicKey);
    console.log("accountInfo", accountInfo);
  });
});
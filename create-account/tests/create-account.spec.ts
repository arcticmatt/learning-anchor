import {setProvider, Provider, workspace, web3} from "@project-serum/anchor";
import { PublicKey } from "@solana/web3.js";

const {Keypair, SystemProgram, Transaction} = web3;

describe('create-account', () => {

  const provider = Provider.env();
  setProvider(provider);
  const program = workspace.CreateAccount;

  /**
   * Creates a "normal" account (e.g. an account associated with a keypair).
   * 
   * Note that SystemProgram.createAccount CANNOT be used to create an account with a PDA,
   * because the account has no private key! Which makes it impossible to properly sign the transaction.
   * 
   * Instead, a program must invoke system_instruction::create_account with invoke_signed. 
   */
  it("createAccount", async () => {
    const keypair1 = Keypair.generate();
    const keypair2 = Keypair.generate();
    const airdrop1 = await provider.connection.requestAirdrop(keypair1.publicKey, 100000000000);
    await provider.connection.confirmTransaction(airdrop1, 'confirmed');
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
    await provider.connection.confirmTransaction(sig, 'confirmed');

    const accountInfo = await provider.connection.getAccountInfo(keypair2.publicKey);
    console.log("accountInfo", accountInfo);
  });

  /**
   * Creates an account with SystemProgram.createAccountWithSeed.
   * 
   * Note that this is different from creating an account associated with a PDA... not sure why you would
   * want to do this.
   */
  it("createAccountWithSeed", async () => {
    const keypair1 = Keypair.generate();
    const airdrop1 = await provider.connection.requestAirdrop(keypair1.publicKey, 100000000000);
    await provider.connection.confirmTransaction(airdrop1, 'confirmed');
    console.log("keypair1", keypair1.publicKey.toString())
    console.log("keypair1 on curve?", PublicKey.isOnCurve(keypair1.publicKey.toBuffer()));

    const pda = 
      await PublicKey.createWithSeed(
        keypair1.publicKey,
        "hi",
        program.programId
      );

    console.log("programId", program.programId.toString());
    console.log("pda", pda.toString());
    console.log("pda on curve?", PublicKey.isOnCurve(pda.toBuffer()));

    // PublicKey.createWithSeed generates a different address than PublicKey.findProgramAddress!
    const [pda2] = await PublicKey.findProgramAddress([Buffer.from("hi"), keypair1.publicKey.toBytes()], program.programId);
    console.log("pda2", pda2.toString());
    console.log("pda2 on curve?", PublicKey.isOnCurve(pda2.toBuffer()));

    const [pda3] = await PublicKey.findProgramAddress([keypair1.publicKey.toBuffer(), Buffer.from("hi")], program.programId);
    console.log("pda3", pda3.toString());
    console.log("pda3 on curve?", PublicKey.isOnCurve(pda3.toBuffer()));

    const instruction = await SystemProgram.createAccountWithSeed({
      basePubkey: keypair1.publicKey,
      fromPubkey: keypair1.publicKey,
      lamports: 10000000,
      newAccountPubkey: pda,
      programId: program.programId,
      seed: "hi",
      space: 2000,
    });

    let tx = new Transaction().add(instruction);
    tx.recentBlockhash = (await provider.connection.getRecentBlockhash()).blockhash;
    tx.feePayer = keypair1.publicKey;
    const sig = await provider.connection.sendTransaction(tx, [keypair1,], { skipPreflight: false, preflightCommitment: "confirmed" });
    await provider.connection.confirmTransaction(sig, 'confirmed');

    const accountInfo = await provider.connection.getAccountInfo(pda);
    console.log("accountInfo owner", accountInfo?.owner.toString());

    // The PDA accounts are not created.
    const accountInfo2 = await provider.connection.getAccountInfo(pda2);
    console.log("accountInfo2", accountInfo2);

    const accountInfo3 = await provider.connection.getAccountInfo(pda3);
    console.log("accountInfo3", accountInfo3);
  });
});
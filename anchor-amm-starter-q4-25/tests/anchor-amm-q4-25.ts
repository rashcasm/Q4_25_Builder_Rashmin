import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AnchorAmmQ425 } from "../target/types/anchor_amm_q4_25";
import { assert, expect } from "chai";
import {
  ASSOCIATED_TOKEN_PROGRAM_ID,
  createAssociatedTokenAccountInstruction,
  createMint,
  getAccount,
  getAssociatedTokenAddressSync,
  getOrCreateAssociatedTokenAccount,
  mintTo,
  TOKEN_PROGRAM_ID,
} from "@solana/spl-token";

describe("anchor-amm-q4-25", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.anchorAmmQ425 as Program<AnchorAmmQ425>;

  let user = provider.wallet.publicKey;
  // let user=anchor.web3.Keypair.generate();
  let mintX: anchor.web3.PublicKey;
  let mintY: anchor.web3.PublicKey;
  let userAtaX: anchor.web3.PublicKey;
  let userAtaY: anchor.web3.PublicKey;
  let userLp: anchor.web3.PublicKey;

  let mintLp: anchor.web3.PublicKey;
  let vaultX: anchor.web3.PublicKey;
  let vaultY: anchor.web3.PublicKey;
  let configPda: anchor.web3.PublicKey;
  let lpBump: number;
  let configBump: number;

  let seed = new anchor.BN(1234);
  let fee = 10;
  let decimals = 6;

  before(async () => {
    await provider.connection.requestAirdrop(
      user,
      20 * anchor.web3.LAMPORTS_PER_SOL
    );

    await new Promise(resolve => setTimeout(resolve, 1000));

    //creating mint for x and y --------
    mintX = await createMint(
      provider.connection,
      provider.wallet.payer,
      user,
      null,
      decimals
    );
    mintY = await createMint(
      provider.connection,
      provider.wallet.payer,
      user,
      null,
      decimals
    );


    // create ats and mint tokens ---
    // for x  ----------
    // userAtaX =  getAssociatedTokenAddressSync(mintX, user);
    // const userAtaXTx = new anchor.web3.Transaction().add(
    //   createAssociatedTokenAccountInstruction(
    //     provider.wallet.publicKey,
    //     userAtaX,
    //     user,
    //     mintX
    //   )
    // );
    // await provider.sendAndConfirm(userAtaXTx);

    const userXInfo = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      provider.wallet.payer,
      mintX,
      user
    );
    userAtaX = userXInfo.address;

    await mintTo(
      provider.connection,
      provider.wallet.payer,
      mintX,
      userAtaX,
      user,
      2 * anchor.web3.LAMPORTS_PER_SOL
    );

    // for y-----------
    // userAtaY =  getAssociatedTokenAddressSync(mintY, user);
    // const userAtaYTx = new anchor.web3.Transaction().add(
    //   createAssociatedTokenAccountInstruction(
    //     provider.wallet.publicKey,
    //     userAtaY,
    //     user,
    //     mintY
    //   )
    // );
    // await provider.sendAndConfirm(userAtaYTx);
     const userYInfo = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      provider.wallet.payer,
      mintY,
      user
    );
     userAtaY = userYInfo.address;
    await mintTo(
      provider.connection,
      provider.wallet.payer,
      mintY,
      userAtaY,
      user,
      2 * anchor.web3.LAMPORTS_PER_SOL
    );


   

    [configPda, configBump] =
       anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("config"), seed.toArrayLike(Buffer, "le", 8)],
        program.programId
      );
    [mintLp, lpBump] = await anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("lp"), configPda.toBuffer()],
      program.programId
    );



   
    console.log('149...success...');
  });

  it("configuration is initialized!", async () => {
    // Add your test here.

        //for vault x and y -------------
    vaultX = getAssociatedTokenAddressSync(mintX, configPda,true);
    vaultY = getAssociatedTokenAddressSync(mintY, configPda,true);
    
    const tx=await program.methods
      .initialize(seed, fee, user)
      .accountsStrict({
        initializer: user,
        mintX: mintX,
        mintY: mintY,
        mintLp: mintLp,
        vaultX: vaultX,
        vaultY: vaultY,
        config: configPda,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();
          console.log("AMM pool initialised, tx is: ", tx);

    const configAccount = await program.account.config.fetch(configPda);
    expect(configAccount.mintX.toBase58()).to.equal(mintX.toBase58());
    expect(configAccount.mintY.toBase58()).to.equal(mintY.toBase58());
    expect(configAccount.fee).to.equal(fee);
    expect(configAccount.configBump).to.equal(configBump);
    expect(configAccount.lpBump).to.equal(lpBump);
    expect(configAccount.locked).to.equal(false);
    console.log("Your transaction signature", tx);
  });

  it("adding liquidity at initial phase", async () => {
    const maxXAmt = new anchor.BN(100_000_000);
    const maxYAmt = new anchor.BN(200_000_000);
    const lpTokens = new anchor.BN(100_000_000);

      //for user lp-----
     [userLp] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        provider.wallet.publicKey.toBuffer(),
        TOKEN_PROGRAM_ID.toBuffer(),
        mintLp.toBuffer(),
      ],
      ASSOCIATED_TOKEN_PROGRAM_ID
    );

    const depositAccounts = {
      user,
      mintX,
      mintY,
      config: configPda,
      mintLp,
      vaultX,
      vaultY,
      userX: userAtaX,
      userY: userAtaY,
      userLp,
      tokenProgram: TOKEN_PROGRAM_ID,
      associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      systemProgram: anchor.web3.SystemProgram.programId,
    };

    await program.methods.deposit(lpTokens, maxXAmt, maxYAmt).accountsStrict(depositAccounts).rpc();

    const vaultXAccount=await getAccount(provider.connection,vaultX);
    const vaultYAccount=await getAccount(provider.connection,vaultY);
    const userLpTokens=await getAccount(provider.connection,userLp);


    expect(vaultXAccount.amount?.toString()).to.not.be.null;
    expect(vaultXAccount.amount).to.lessThanOrEqual(maxXAmt.toNumber(),"Slippage exceeded");
    expect(vaultYAccount.amount).to.lessThanOrEqual(maxYAmt.toNumber(),"Slippage exceeded");
    expect(userLpTokens.amount?.toString()).to.be.equal(lpTokens?.toString());
  });

  //swapping -----------

  it("swapping 10 X tokens for Y", async () => {
    const xTokenAmt = new anchor.BN(10_000_000);
    const minYToken = new anchor.BN(3_000_000);

     let beforeUserAtaXAccount=await getAccount(provider.connection,userAtaX);
    let beforeUserAtaYAccount=await getAccount(provider.connection,userAtaY);

    console.log(`Before Swap user's x balance: ${beforeUserAtaXAccount} and user's y balance: ${beforeUserAtaYAccount}`);
    console.log(`swapping 10 X tokens for min 3 Y tokens..`);

    const swapAccounts = {
      user,
      mintX,
      mintY,
      config: configPda,
      mintLp,
      vaultX,
      vaultY,
      userX: userAtaX,
      userY: userAtaY,
      tokenProgram: TOKEN_PROGRAM_ID,
      associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      systemProgram: anchor.web3.SystemProgram.programId,
    };

    await program.methods.swap(true, xTokenAmt, minYToken).accountsStrict(swapAccounts).rpc();

    let afterUserAtaXAccount=await getAccount(provider.connection,userAtaX);
    let afterUserAtaYAccount=await getAccount(provider.connection,userAtaY);
    const userLpTokens=await getAccount(provider.connection,userLp);

    console.log(`after Swap user's x balance: ${afterUserAtaXAccount} and user's y balance: ${afterUserAtaYAccount}`);

    
    assert(beforeUserAtaXAccount.amount.toString()>afterUserAtaXAccount.amount.toString(),"user's x balance should decrease.")
    
    assert(beforeUserAtaYAccount.amount.toString()<afterUserAtaYAccount.amount.toString(),"user's y balance should increase.")
  
  });

  it("swapping 20 Y tokens for X", async () => {
    const yTokenAmt = new anchor.BN(20_000_000);
    const minXToken = new anchor.BN(10_000_000);

     let beforeUserAtaXAccount=await getAccount(provider.connection,userAtaX);
    let beforeUserAtaYAccount=await getAccount(provider.connection,userAtaY);

    console.log(`Before Swap user's x balance: ${beforeUserAtaXAccount} and user's y balance: ${beforeUserAtaYAccount}`);

    console.log(`swapping 20 Y tokens for min 10 Y tokens..`);

    const swapAccounts = {
      user,
      mintX,
      mintY,
      config: configPda,
      mintLp,
      vaultX,
      vaultY,
      userX: userAtaX,
      userY: userAtaY,
      tokenProgram: TOKEN_PROGRAM_ID,
      associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      systemProgram: anchor.web3.SystemProgram.programId,
    };

    await program.methods.swap(false, yTokenAmt, minXToken).accountsStrict(swapAccounts).rpc();

    let afterUserAtaXAccount=await getAccount(provider.connection,userAtaX);
    let afterUserAtaYAccount=await getAccount(provider.connection,userAtaY);
    const userLpTokens=await getAccount(provider.connection,userLp);

    console.log(`after Swap user's x balance: ${afterUserAtaXAccount} and user's y balance: ${afterUserAtaYAccount}`);

    
    assert(beforeUserAtaXAccount.amount.toString()>afterUserAtaXAccount.amount.toString(),"user's x balance should  decrease.")
    
    assert(beforeUserAtaYAccount.amount.toString()<afterUserAtaYAccount.amount.toString(),"user's y balance should increase.")
  
  });

  //withdraw---------
  it("withdrawing half liquidity from pool", async () => {
   
    const beforeUserAtaXAccount=await getAccount(provider.connection,userAtaX);
    const beforeUserAtaYAccount=await getAccount(provider.connection,userAtaY);
    const beforeUserLpTokens=await getAccount(provider.connection,userLp);
    const halfAmount=new anchor.BN(beforeUserLpTokens.amount).div(new anchor.BN(2));

    console.log(`Before withdraw user's x balance: ${beforeUserAtaXAccount} , user's y balance: ${beforeUserAtaYAccount} and user LpTokens: ${beforeUserLpTokens.amount}`);
    

    const withdrawAccounts = {
      user,
      mintX,
      mintY,
      config: configPda,
      mintLp,
      userLp,
      vaultX,
      vaultY,
      userX: userAtaX,
      userY: userAtaY,
      tokenProgram: TOKEN_PROGRAM_ID,
      associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      systemProgram: anchor.web3.SystemProgram.programId,
    };

    await program.methods.deposit(halfAmount,new anchor.BN(0),new anchor.BN(0)).accountsStrict(withdrawAccounts).rpc();


    const afterUserAtaXAccount=await getAccount(provider.connection,userAtaX);
    const afterUserAtaYAccount=await getAccount(provider.connection,userAtaY);
    const afterUserLpTokens=await getAccount(provider.connection,userLp);


    console.log(`After withdraw user's x balance: ${afterUserAtaXAccount} , user's y balance: ${afterUserAtaYAccount} and user LpToken: ${afterUserLpTokens.amount}`);

    assert(afterUserAtaXAccount.amount > beforeUserAtaXAccount.amount, "user's x balance should increase");
    assert(afterUserAtaYAccount.amount > beforeUserAtaYAccount.amount, "user's y balance should increase");
    assert(afterUserLpTokens.amount < beforeUserLpTokens.amount, "user's lp token should decrease");

  });
});
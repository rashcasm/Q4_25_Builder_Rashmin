import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Ekonos } from "../target/types/ekonos";
import { expect } from "chai";
import { PublicKey, Keypair, SystemProgram } from "@solana/web3.js";
import {
  createMint,
  getOrCreateAssociatedTokenAccount,
  mintTo,
  getAccount,
  getAssociatedTokenAddress,
} from "@solana/spl-token";

const TOKEN_PROGRAM_ID = new PublicKey("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");
const ASSOCIATED_TOKEN_PROGRAM_ID = new PublicKey("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL");

describe("ekonos", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.ekonos as Program<Ekonos>;

  describe("initialize_partnership", () => {
    it("initializes partnership", async () => {
      const provider = anchor.getProvider() as anchor.AnchorProvider;
      const authority = provider.wallet.publicKey;

      const partnershipId = new anchor.BN(1);
      const totalShares = new anchor.BN(1000);

      const [partnershipPda] = await PublicKey.findProgramAddress(
        [Buffer.from("partnership"), authority.toBuffer(), partnershipId.toArrayLike(Buffer, "le", 8)],
        program.programId
      );

      await program.methods
        .initializePartnership(partnershipId, totalShares)
        .accounts({
          authority: authority,
          partnership: partnershipPda,
          systemProgram: SystemProgram.programId,
        } as any)
        .rpc();

      const acct = await program.account.partnership.fetch(partnershipPda);
      expect(acct.authority.toString()).to.equal(authority.toString());
      expect(acct.totalShares.toNumber()).to.equal(1000);
    });

    it("fails when initializing same partnership twice", async () => {
      const provider = anchor.getProvider() as anchor.AnchorProvider;
      const authority = provider.wallet.publicKey;

      const partnershipId = new anchor.BN(2);
      const totalShares = new anchor.BN(10);

      const [partnershipPda] = await PublicKey.findProgramAddress(
        [Buffer.from("partnership"), authority.toBuffer(), partnershipId.toArrayLike(Buffer, "le", 8)],
        program.programId
      );

      await program.methods
        .initializePartnership(partnershipId, totalShares)
        .accounts({
          authority: authority,
          partnership: partnershipPda,
          systemProgram: SystemProgram.programId,
        } as any)
        .rpc();

      // Second call should fail because account already exists
      try {
        await program.methods
          .initializePartnership(partnershipId, totalShares)
          .accounts({
            authority: authority,
            partnership: partnershipPda,
            systemProgram: SystemProgram.programId,
          } as any)
          .rpc();
        throw new Error("duplicate init unexpectedly succeeded");
      } catch (err: any) {
        expect(err.toString()).to.include("already in use");
      }
    });
  });

  describe("deposit_nft", () => {
    it("fails when called by non-owner (unauthorized)", async () => {
      const provider = anchor.getProvider() as anchor.AnchorProvider;
      const authority = provider.wallet.publicKey;

      const partnershipId = new anchor.BN(3);
      const totalShares = new anchor.BN(1);

      const [partnershipPda] = await PublicKey.findProgramAddress(
        [Buffer.from("partnership"), authority.toBuffer(), partnershipId.toArrayLike(Buffer, "le", 8)],
        program.programId
      );

      await program.methods
        .initializePartnership(partnershipId, totalShares)
        .accounts({ authority: authority, partnership: partnershipPda, systemProgram: SystemProgram.programId } as any)
        .rpc();

      // Create a different owner (bob) who actually holds the NFT
      const bob = Keypair.generate();

      // Create an NFT mint and mint 1 token to bob's ATA (so owner != authority)
      const connection = provider.connection;
      const mint = await createMint(connection, provider.wallet.payer as any, authority, null, 0);
      const bobAta = await getOrCreateAssociatedTokenAccount(connection, provider.wallet.payer as any, mint, bob.publicKey);
      await mintTo(connection, provider.wallet.payer as any, mint, bobAta.address, authority, 1);

      // Attempt deposit with correct authority but nft_from owned by bob -> should fail with Unauthorized
      try {
        const vaultAta = await getAssociatedTokenAddress(mint, partnershipPda, true);

        await program.methods
          .depositNft()
          .accounts({
            authority: authority,
            partnership: partnershipPda,
            nftMint: mint,
            nftFrom: bobAta.address,
            vaultAta: vaultAta,
            tokenProgram: TOKEN_PROGRAM_ID,
            associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
            systemProgram: SystemProgram.programId,
            rent: anchor.web3.SYSVAR_RENT_PUBKEY,
          } as any)
          .rpc();
        throw new Error("unauthorized deposit unexpectedly succeeded");
      } catch (err: any) {
        expect(err.toString()).to.include("Unauthorized");
      }
    });

    it("deposits an NFT successfully", async () => {
      const provider = anchor.getProvider() as anchor.AnchorProvider;
      const authority = provider.wallet.publicKey;
      const connection = provider.connection;

      const partnershipId = new anchor.BN(4);
      const totalShares = new anchor.BN(100);
      const [partnershipPda] = await PublicKey.findProgramAddress(
        [Buffer.from("partnership"), authority.toBuffer(), partnershipId.toArrayLike(Buffer, "le", 8)],
        program.programId
      );

      await program.methods
        .initializePartnership(partnershipId, totalShares)
        .accounts({ authority: authority, partnership: partnershipPda, systemProgram: SystemProgram.programId } as any)
        .rpc();

      // Create NFT mint, mint 1 to authority
      const mint = await createMint(connection, provider.wallet.payer as any, authority, null, 0);
      const authorityAta = await getOrCreateAssociatedTokenAccount(connection, provider.wallet.payer as any, mint, authority);
      await mintTo(connection, provider.wallet.payer as any, mint, authorityAta.address, authority, 1);

      // Derive vault ATA for partnership
      const vaultAta = await getAssociatedTokenAddress(mint, partnershipPda, true);

      await program.methods
        .depositNft()
        .accounts({
          authority: authority,
          partnership: partnershipPda,
          nftMint: mint,
          nftFrom: authorityAta.address,
          vaultAta: vaultAta,
          tokenProgram: TOKEN_PROGRAM_ID,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
          systemProgram: SystemProgram.programId,
          rent: anchor.web3.SYSVAR_RENT_PUBKEY,
        } as any)
        .rpc();

      const acct = await program.account.partnership.fetch(partnershipPda);
      expect(acct.isNftDeposited).to.be.true;
    });
  });

  describe("mint_shares", () => {
    it("fails if NFT not deposited", async () => {
      const provider = anchor.getProvider() as anchor.AnchorProvider;
      const authority = provider.wallet.publicKey;

      const partnershipId = new anchor.BN(5);
      const totalShares = new anchor.BN(50);
      const [partnershipPda] = await PublicKey.findProgramAddress(
        [Buffer.from("partnership"), authority.toBuffer(), partnershipId.toArrayLike(Buffer, "le", 8)],
        program.programId
      );

      await program.methods
        .initializePartnership(partnershipId, totalShares)
        .accounts({ authority: authority, partnership: partnershipPda, systemProgram: SystemProgram.programId } as any)
        .rpc();

      const [shareMintPda] = await PublicKey.findProgramAddress([Buffer.from("share_mint"), partnershipPda.toBuffer()], program.programId);
      const distribution = [{ wallet: authority, amount: new anchor.BN(50) }];

      try {
        const authorityAta = await getAssociatedTokenAddress(shareMintPda, authority, true);

        await program.methods
          .mintShares(distribution)
          .accounts({
            authority: authority,
            partnership: partnershipPda,
            shareMint: shareMintPda,
            tokenProgram: TOKEN_PROGRAM_ID,
            associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
            systemProgram: SystemProgram.programId,
            rent: anchor.web3.SYSVAR_RENT_PUBKEY,
            authorityAta: authorityAta,
          } as any)
          .rpc();
        throw new Error("mintShares unexpectedly succeeded");
      } catch (err: any) {
        expect(err.toString()).to.include("NftNotDeposited");
      }
    });

    it("mints shares after NFT deposit", async () => {
      const provider = anchor.getProvider() as anchor.AnchorProvider;
      const authority = provider.wallet.publicKey;
      const connection = provider.connection;

      const partnershipId = new anchor.BN(6);
      const totalShares = new anchor.BN(10);
      const [partnershipPda] = await PublicKey.findProgramAddress(
        [Buffer.from("partnership"), authority.toBuffer(), partnershipId.toArrayLike(Buffer, "le", 8)],
        program.programId
      );

      await program.methods
        .initializePartnership(partnershipId, totalShares)
        .accounts({ authority: authority, partnership: partnershipPda, systemProgram: SystemProgram.programId } as any)
        .rpc();

      // Create NFT and deposit
      const mint = await createMint(connection, provider.wallet.payer as any, authority, null, 0);
      const authorityAta = await getOrCreateAssociatedTokenAccount(connection, provider.wallet.payer as any, mint, authority);
      await mintTo(connection, provider.wallet.payer as any, mint, authorityAta.address, authority, 1);
      const vaultAta = await getAssociatedTokenAddress(mint, partnershipPda, true);

      await program.methods
        .depositNft()
        .accounts({
          authority: authority,
          partnership: partnershipPda,
          nftMint: mint,
          nftFrom: authorityAta.address,
          vaultAta: vaultAta,
          tokenProgram: TOKEN_PROGRAM_ID,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
          systemProgram: SystemProgram.programId,
          rent: anchor.web3.SYSVAR_RENT_PUBKEY,
        } as any)
        .rpc();

      // Now mint shares
      const [shareMintPda] = await PublicKey.findProgramAddress([Buffer.from("share_mint"), partnershipPda.toBuffer()], program.programId);
      const authorityAtaSharesPub = await getAssociatedTokenAddress(shareMintPda, authority);

      const distribution = [{ wallet: authority, amount: new anchor.BN(10) }];

      await program.methods
        .mintShares(distribution)
        .accounts({
          authority: authority,
          partnership: partnershipPda,
          shareMint: shareMintPda,
          tokenProgram: TOKEN_PROGRAM_ID,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
          systemProgram: SystemProgram.programId,
          rent: anchor.web3.SYSVAR_RENT_PUBKEY,
          authorityAta: authorityAtaSharesPub,
        } as any)
        .rpc();

      // verify balance
      const acct = await getAccount(connection, authorityAtaSharesPub);
      expect(Number(acct.amount)).to.equal(10);
    });
  });
});

# airdrop2

Turbin3 pre-requisite task 2!

## Directory Structure
- `src/lib.rs`: Main library file containing core logic and functions.
- `dev-wallet.json`, `Turbin3-wallet.json`: wallet files for development and testing [git ignored]
- `Cargo.toml`, `Cargo.lock`: Rust package manifest and lock file.

## Brief outline of what it does
1. Generate wallets
2. Encode/decode private keys
3. Claim airdrops
4. Transfer SOL
5. Empty wallets
6. Submit transactions to a custom Solana program (PDA-based). PDA account was created with TS as a part of task 1

- `keygen()` - Create a new Solana keypair (wallet) and display it.
- `wallet_to_base58()` - Convert a JSON byte array private key into Base58 string.
- `claim_airdrop()`- Request Devnet SOL for a wallet.
- `transfer_sol()` - Send SOL from your wallet to turbin3 wallet address.
- `empty_wallet()` - Transfer entire wallet balance to turbin3 wallet address while accounting for transaction fees.
- `submit_rs()` - Submit a custom instruction to a Solana program.

> For details, see the comments in `src/lib.rs`.

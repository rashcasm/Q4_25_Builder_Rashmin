# Ekonos

### *Own Assets in Partnership. On-Chain, With Real-World Simplicity*

Ekonos is a Solana smart-contract system built using Anchor that enables co-ownership of digital assets through fractional tokenized shares.

Think of it as a way to buy and own assets together. Just like you can co-own land, property, or IP in the real world.

This MVP demonstrates the core mechanism for **locking an asset, assigning ownership shares, and maintaining neutral custody**.


At a high level, the program enables:

### Create a Partnership

* A PDA-derived on-chain account tied to an authority.
* Stores metadata about the partnership & fractional share setup.
* Only one NFT (for MVP) can be associated with a partnership.

### Deposit & Lock an NFT

* NFT is transferred from the authority into a **vault ATA owned by a PDA**, not a person.
* This ensures no one can individually move/sell the asset without rules.

### Mint Fractional Ownership Shares

* The program creates an SPL mint representing “shares” of the NFT.
* Total supply (e.g. 10,000 shares) represents **100% ownership**.
* Shares are distributed to chosen wallets according to fixed distribution rules.

*In short: the NFT goes into escrow, and ownership becomes fungible & transferrable as SPL tokens.*


## **Repository Layout**

```
programs/ekonos/  
  src/instructions/     # initialize_partnership, deposit_nft, mint_shares
  src/state/            # PDA account / state structs
  Cargo.toml            # dependencies
tests/             # TypeScript tests (`anchor test`)
Anchor.toml
Cargo.toml
package.json
```

## **Running Locally / Tests**

Install JS dependencies (tests use @coral-xyz/anchor + spl-token helpers):

```bash
npm install
```

Compile the program:

```bash
anchor build
```

Run the full test suite (automatically starts a local validator):

```bash
anchor test
```

---

## **Why tf This MVP?**

This version is intentionally small and focused.

It **does not** aim to solve legal frameworks, real-world compliance, or dynamic governance.
It **only proves the core mechanism**:

> *Neutral escrow of an asset + programmatic fractional ownership.*

Think of it as the **engine block**, not the whole car.


## Future Scope

This MVP is step 1 of a much larger system for asset-backed co-ownership on Solana. Future phases include:

### Transferring Ownership Shares

* Free market trading or internal marketplace for fractional shares.
* NFT ownership will follow whoever holds majority/control tokens.

### Redeeming / Buyout to Withdraw the NFT

* A process for reclaiming the NFT by:

  * burning enough share tokens (full buyout),
  * or hitting a threshold (e.g., >95% consensus),
  * or full unanimous settlement.

### Automated Revenue Distribution

* Routing royalties, rental income, IP revenue, or dividends.
* Partners receive prorated payments based on share tokens.
* Support for SOL & any SPL token.

### Partnership Governance

* Adding delegated authority, multi-sig control, or weighted voting by shares.
* Control who can withdraw, liquidate, or redeploy assets.

### Compliance & Trust Layers

* Integration with custodians, oracles, KYC layers, and regulated vaults.
* Optional audit proofs for real-world backing.

## 💬 **Vision**

Ekonos aims to reinvent joint ownership. Instead of lawyers, middlemen, or trust, we rely on transparent, enforceable, cryptographic agreements.

Whether fractionalizing a digital IP or tokenizing a warehouse in New York, ownership becomes programmable.


### 👨‍💻 Contribute / Feedback

Want to collaborate, audit, or expand features?
DM or open an issue, the goal is to eventually take Ekonos beyond NFTs and into real-world programmable equity.

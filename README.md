
# 🌱 PlantGO – Solana NFT Program

PlantGO is a Solana-based NFT program that rewards users with collectible **Plant Cards** when they:
- Discover plants
- Identify new species
- Participate in quizzes

Each card is a **1/1 NFT**, minted using SPL Token, with **on-chain ownership records** and **plant-wise supply tracking**.

This README is written for **frontend developers** so they know:
- What instruction to send
- What accounts are required
- How PDAs are derived
- How rarity is decided

---

## 🧠 Core Concepts

- Every NFT = SPL Token with supply `1`
- Ownership is stored in a **PDA**
- Plant-wise mint counts are stored in a **Plant Counter PDA**
- Rarity is determined on-chain

---

## 🛠 Tech Stack

- Solana Program (Rust)
- Borsh Serialization
- SPL Token Program
- SPL Associated Token Account Program

---

## 🔧 Instruction Encoding

All instructions are serialized using **Borsh**.

---

## 📜 Program Instructions

### 1️⃣ CreateMint

Creates and initializes an SPL Token mint used for NFTs.

#### Instruction
```ts
CreateMint {
  args: {
    nft_title: string
    nft_symbol: string
    nft_uri: string
  }
}

Required Accounts (in order)
Index	Account	Writable	Signer
0	Mint Account	✅	❌
1	Mint Authority	❌	❌
2	Payer	✅	✅
3	Rent Sysvar	❌	❌
4	System Program	❌	❌
5	Token Program	❌	❌

Notes:

Mint decimals = 0

Mint authority == freeze authority

Metadata (title, symbol, URI) is currently frontend-handled

2️⃣ MintNFT

Mints an NFT based on plant discovery or quiz logic.

Instruction
MintNFT {
  card_type: CardRarityInstruction
  plant_name: string
  is_new_species?: boolean
  quiz_winner?: boolean
}

🎴 Card Rarity Types
enum CardRarityInstruction {
  GenesisFragment,   // Common
  AstralShard,       // Rare
  MythicCrest,       // Epic
  AscendantSeal,     // Quiz Winner
  CodexOfInsight,    // Quiz Participation
  PrimordialRelic,   // First known discovery
  AuroraSeed         // First ever species discovery
}

🌿 MintNFT – Plant Discovery Flow

Used when minting:
GenesisFragment, AstralShard, MythicCrest, PrimordialRelic, AuroraSeed

Required Accounts (in order)
Index	Account
0	User Wallet
1	Common Mint
2	Rare Mint
3	Epic Mint
4	Aurora Mint
5	Primordial Mint
6	Mint Authority
7	User Associated Token Account
8	Payer
9	Rent Sysvar
10	System Program
11	Token Program
12	Ownership PDA
13	Plant Counter PDA
🧪 MintNFT – Quiz Flow

Used when minting:
CodexOfInsight, AscendantSeal

Required Accounts (in order)
Index	Account
0	User Wallet
1	Codex Mint
2	Ascendant Mint
3	Mint Authority
4	User Associated Token Account
5	Payer
6	Rent Sysvar
7	System Program
8	Token Program
9	Ownership PDA
10	Plant Counter PDA
🧾 Program Derived Addresses (PDAs)
1️⃣ Ownership PDA

Ensures a user cannot mint the same plant card twice.

seeds = [
  plant_name (bytes),
  user_wallet (pubkey),
  card_type (u8)
]


Stores ownership metadata

One per (user + plant + rarity)

2️⃣ Plant Counter PDA

Tracks mint counts and first discovery.

seeds = [
  "plant_counter",
  plant_name
]

📊 Rarity Distribution Rules
Condition	Card Minted
New species + first on-chain	AuroraSeed
Known species + first on-chain	PrimordialRelic
Epic < 20	MythicCrest
Rare < 50	AstralShard
Otherwise	GenesisFragment
🧾 Ownership Record (Stored On-Chain)
{
  owner: Pubkey
  plant_name: string
  rarity: CardRarityInstruction
  minted_at: UnixTimestamp
  nft_mint: Pubkey
}

🌱 Plant Counter Data
{
  plant_name: string
  seed_count: number
  relic_count: number
  epic_count: number
  rare_count: number
  common_count: number
  mastery_count: number
  codex_count: number
  first_minter?: Pubkey
}

🪙 NFT Behavior

SPL Token

Supply = 1

Minted to user's ATA

ATA auto-created if missing

❌ Common Errors
Error	Meaning
InvalidInstructionData	Wrong Borsh encoding
InvalidArgument	PDA mismatch
Custom(999)	User already owns this card

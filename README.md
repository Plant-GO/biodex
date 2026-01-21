🌱 PlantGO
PlantGO is a Solana-based mobile application that promotes biodiversity awareness and citizen engagement through gamification, artificial intelligence, and blockchain technology. By turning plant discovery into an interactive game, PlantGO encourages users to explore nature, learn about local flora, and contribute to scientific data collection while earning collectible NFT cards.

📑 Table of Contents

Overview
Features
How It Works
Gamification & Rewards
Blockchain Integration
Technology Stack
NFT Program Documentation

Core Concepts
Program Instructions
PDA Derivation
Card Rarity System


Getting Started
Contributing
Contact


🌍 Overview
Biodiversity conservation is at the heart of the United Nations Sustainable Development Goals (SDG 15: Life on Land). Engaging citizens effectively in biodiversity monitoring, however, remains a challenge. PlantGO leverages gamification to motivate learning, sustain engagement, and encourage pro-environmental behaviors while building a crowdsourced database of plant sightings.
PlantGO integrates treasure-hunt style gameplay, where users receive riddles and challenges guiding them to discover and identify plants in their local environment. Plant identification is performed via a camera-based interface powered by a machine learning model (VGG16 architecture), which provides confidence levels to inform users about prediction reliability.

✨ Features

🔍 Plant Discovery: Explore your local environment to discover and identify plants
🤖 AI-based Identification: Capture plant images and classify them using a VGG16 machine learning model
🎮 Gamified Challenges: Solve riddles and complete quests to earn points and rewards
✅ Crowdsourced Validation: Community-driven verification ensures data accuracy and reliability
🗺️ Digital Botanical Map: Verified sightings contribute to a collaborative map of global plant biodiversity
💎 Blockchain Rewards: Earn collectible NFT cards for plant discoveries, quiz participation, and rare finds


🎯 How It Works

Receive Challenges: Users receive riddles and quests directing them to local plants
Capture & Identify: Take photos of plants using the in-app camera. AI classifies the plant and displays prediction confidence
Community Validation: The blockchain-backed voting system allows users to confirm or correct identifications
Earn Rewards: Gamified incentives, including points, levels, and collectible NFT cards, motivate continued participation
Track Contributions: Verified plant sightings are added to the digital botanical map, helping researchers and policymakers


🎁 Gamification & Rewards
PlantGO turns plant exploration into a fun and competitive experience with NFT cards and points:
Plant Discovery Cards

Common Cards: GenesisFragment (for common plant discoveries)
Rare Cards: AstralShard (for rare plant finds)
Epic Cards: MythicCrest (for epic discoveries)
Mastery Cards: AscendantSeal (for plant mastery achievements)

Quiz Participation Cards

All quiz participants earn a CodexOfInsight card
Quiz winners receive an AscendantSeal card

Special Achievement Cards

AuroraSeed: First to identify an invasive plant species
PrimordialRelic: First person in the world to discover a new plant species

These rewards are securely minted and tracked on the Solana blockchain as 1/1 NFTs, ensuring fairness, transparency, and data integrity.

⛓️ Blockchain Integration
PlantGO uses the Solana blockchain to ensure secure storage, transparent rewards, and community trust:

NFT Card Minting: Each card is a unique 1/1 SPL Token
On-chain Ownership: Ownership records are stored in PDAs (Program Derived Addresses)
Supply Tracking: Plant-wise mint counts are tracked on-chain
Blockchain Voting: Community-driven validation of plant identifications
Transparent Rewards: All card distributions are publicly verifiable


🛠 Technology Stack

Mobile App: React Native / Flutter (cross-platform)
AI/ML: PyTorch, TensorFlow (VGG16-based plant classification)
Blockchain: Solana (for NFT minting and validation)
Smart Contract: Rust (Solana Program)
Backend: Node.js / Rust (Quinn or Actix for P2P and API services)
Database: PostgreSQL / MongoDB (for user and plant data)
Serialization: Borsh
Token Standard: SPL Token Program


📜 NFT Program Documentation
This section is written for frontend developers to understand how to interact with the Solana NFT program.
🧠 Core Concepts

Every NFT = SPL Token with supply 1
Ownership is stored in a PDA (Program Derived Address)
Plant-wise mint counts are stored in a Plant Counter PDA
Rarity is determined on-chain based on card type
All instructions are serialized using Borsh


🔧 Program Instructions
1️⃣ CreateMint
Creates and initializes an SPL Token mint used for NFTs.
Instruction Structure:

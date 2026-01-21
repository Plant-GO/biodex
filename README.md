
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

# PlantGO 🌱

**PlantGO** is a mobile application designed to promote biodiversity awareness and citizen engagement through gamification, artificial intelligence, and blockchain technologies. By turning plant discovery into an interactive game, PlantGO encourages users to explore nature, learn about local flora, and contribute to scientific data collection.

---

## Table of Contents

- [Overview](#overview)
- [Features](#features)
- [How It Works](#how-it-works)
- [Gamification & Rewards](#gamification--rewards)
- [Blockchain Integration](#blockchain-integration)
- [Technology Stack](#technology-stack)
- [Getting Started](#getting-started)
- [Contributing](#contributing)
- [Contact](#contact)

---

## Overview

Biodiversity conservation is at the heart of the United Nations Sustainable Development Goals (SDG 15: Life on Land). Engaging citizens effectively in biodiversity monitoring, however, remains a challenge. PlantGO leverages gamification to motivate learning, sustain engagement, and encourage pro-environmental behaviors while building a crowdsourced database of plant sightings.

PlantGO integrates a treasure-hunt style gameplay, where users receive riddles and challenges guiding them to discover and identify plants in their local environment. Plant identification is performed via a camera-based interface powered by a machine learning model (VGG16 architecture), which provides confidence levels to inform users about prediction reliability.

---

## Features

- **Plant Discovery:** Explore your local environment to discover and identify plants.  
- **AI-based Identification:** Capture plant images and classify them using a VGG16 machine learning model.  
- **Gamified Challenges:** Solve riddles and complete quests to earn points and rewards.  
- **Crowdsourced Validation:** Community-driven verification ensures data accuracy and reliability.  
- **Digital Botanical Map:** Verified sightings contribute to a collaborative map of global plant biodiversity.  
- **Blockchain Rewards:** Earn collectible cards for plant discoveries, quiz participation, and rare finds.  

---

## How It Works

1. **Receive Challenges:** Users receive riddles and quests directing them to local plants.  
2. **Capture & Identify:** Take photos of plants using the in-app camera. AI classifies the plant and displays prediction confidence.  
3. **Community Validation:** The blockchain-backed voting system allows users to confirm or correct identifications.  
4. **Earn Rewards:** Gamified incentives, including points, levels, and collectible cards, motivate continued participation.  
5. **Track Contributions:** Verified plant sightings are added to the digital botanical map, helping researchers and policymakers.  

---

## Gamification & Rewards

PlantGO turns plant exploration into a fun and competitive experience with **cards and points**:  

- **Plant Discovery Cards:**  
  - Common, Rare, Epic, and Mastery cards are minted depending on the plants discovered.  
- **Quiz Participation Cards:**  
  - All quiz participants earn a **Codex of Insight** card.  
  - Winners receive an **Ascendant Seal** card.  
- **Special Achievement Cards:**  
  - First to identify an invasive plant: **Aurora Seed** card.  
  - First person in the world to discover a new plant species: **Primordial Relic** card.  

These rewards are securely minted and tracked on the blockchain, ensuring fairness, transparency, and data integrity.

---

## Blockchain Integration

PlantGO uses blockchain to ensure secure storage, transparent rewards, and community trust:  

```rust
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub enum CardRarityInstruction {
    // Common
    GenesisFragment,

    // Rare
    AstralShard,

    // Epic
    MythicCrest,

    // Mastery
    AscendantSeal,

    // Knowledge
    CodexOfInsight,

    // First to discover a plant
    PrimordialRelic,

    // Only first invasive plant
    AuroraSeed,
}'''

## Blockchain Rewards & Participation

- Cards are issued based on **rarity**, **achievements**, and **first discoveries**.  
- Blockchain-backed voting validates plant identifications.  
- Tokens and card ownership encourage sustained participation.  

---

## Technology Stack

- **Mobile App:** React Native / Flutter (cross-platform)  
- **AI:** PyTorch, TensorFlow (VGG16-based plant classification)  
- **Blockchain:** Solana / NEAR Protocol (for card minting and validation)  
- **Backend:** Node.js / Rust (Quinn or Actix for P2P and API services)  
- **Database:** PostgreSQL / MongoDB (for user and plant data)  

---

## Getting Started

1. Clone the repository:  
   ```bash
   git clone https://github.com/yourusername/PlantGO.git
2. Install dependencies for the mobile app and backend.

3. Configure blockchain network settings.

4. Run the backend server and launch the mobile app.

5. Start discovering plants and collecting cards

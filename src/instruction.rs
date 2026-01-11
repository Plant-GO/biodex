use std::collections::HashMap;

use crate::mint::CreateTokenArgs;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug)]
pub struct OwnershipRecord {
    pub owner: Pubkey,

    pub plant_name: String,

    pub rarity: CardRarityInstruction,


    pub nft_mint: Pubkey,
}

impl OwnershipRecord {
    pub const MAX_SIZE: usize = 32 + 4 + 50 + 1 + 8 + 32;

    pub fn new(
        owner: Pubkey,
        plant_name: String,
        rarity: CardRarityInstruction,
        nft_mint: Pubkey,
    ) -> OwnershipRecord {
        Self {
            owner,
            plant_name,
            rarity,
            nft_mint,
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub enum ProgramInstruction {
    CreateMint {
        args: CreateTokenArgs,
    },

    MintNFT {
        card_type: CardRarityInstruction,
        plant_name: String,
        is_new_species: Option<bool>,
        quiz_winner: Option<bool>,
    },
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct PlantInfo {
    pub name: String,
    pub rarity: CardRarityInstruction,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct PlantRegistry {
    pub plants: Vec<PlantInfo>,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct PlantCounter {
    pub plant_name: String,
    pub seed_count: u64,
    pub relic_count: u64,
    pub epic_count: u64,
    pub rare_count: u64,
    pub common_count: u64,
    pub mastery_count: u64,
    pub codex_count: u64,
    pub first_minter: Option<Pubkey>,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub enum CardRarityInstruction {
    // Common
    GenesisFragment,

    // Rare
    AstralShard,

    // Epic
    MythicCrest,

    //Mastery
    AscendantSeal,

    // Knowledge
    CodexOfInsight,

    //First
    PrimordialRelic,

    // Only first
    AuroraSeed,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct CardAccount {
    pub card_id: u64,
    pub owner: Pubkey,
    pub rarity: CardRarityInstruction,
    pub ipfs_cid: Vec<u64>,
    pub discovered_at: i64,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct CardSupply {
    pub plant_id: u64,
    pub epic_minted: u64,
    pub rare_minted: u64,
    pub common_minted: u64,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct UserCardOwnership {
    pub user: Pubkey,
    pub owned_plants: HashMap<u64, CardRarityInstruction>,
}

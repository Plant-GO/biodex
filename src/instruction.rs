use std::collections::HashMap;

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

use crate::mint::CreateTokenArgs;

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub enum ProgramInstruction {
    CreateMint {
        args: CreateTokenArgs,
    },

    MintNFT {
        card_type: CardRarityInstruction,
        plant_name: String,
    },
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

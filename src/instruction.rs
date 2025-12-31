use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

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
    PromordialRelic,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct CardAccount {
    pub card_id: u64,
    pub owner: Pubkey,
    pub rarity: CardRarityInstruction,
    pub ipfs_cid: Vec<u64>,
    pub discovered_at: i64,
}

use borsh::BorshDeserialize;
use solana_program::{account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey};

use crate::instruction::{CardRarityInstruction, OwnershipRecord};

pub fn get_user_cards_for_plant(
    program_id: &Pubkey,
    user_wallet: &Pubkey,
    plant_name: &str,
    accounts: &[AccountInfo],
) -> Result<Vec<OwnershipRecord>, ProgramError> {
    let mut owned_cards = Vec::new();

    let card_types = [
        CardRarityInstruction::GenesisFragment,
        CardRarityInstruction::AstralShard,
        CardRarityInstruction::MythicCrest,
        CardRarityInstruction::AscendantSeal,
        CardRarityInstruction::PrimordialRelic,
        CardRarityInstruction::AuroraSeed,
        CardRarityInstruction::CodexOfInsight,
    ];

    for card_type in card_types.iter() {
        let (ownership_pda, _) = Pubkey::find_program_address(
            &[
                plant_name.as_bytes(),
                user_wallet.as_ref(),
                &[card_type.clone() as u8],
            ],
            program_id,
        );

        if let Some(account) = accounts.iter().find(|a| a.key == &ownership_pda) {
            if !account.data_is_empty() {
                if let Ok(record) = OwnershipRecord::try_from_slice(&account.data.borrow()) {
                    owned_cards.push(record);
                }
            }
        }
    }

    Ok(owned_cards)
}

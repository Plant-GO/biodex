use crate::CardRarityInstruction;
use borsh::BorshDeserialize;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::{self, ProgramResult},
    example_mocks::solana_sdk::system_program,
    msg,
    program::invoke,
    program_error::ProgramError,
    pubkey::Pubkey,
    system_instruction,
    sysvar::{rent::Rent, Sysvar},
};
use solana_sdk::signer::Signer;
use spl_associated_token_account_interface::instruction as associated_token_account_instruction;
use spl_token::instruction as token_instruction;
use mpl_token_metadata::instruction as mpl_instruction;

pub struct Processor {}

impl Processor {
    fn process_genesis_fragment() -> ProgramResult {
        Ok(())
    }

    fn process_astral_shard() -> ProgramResult {
        Ok(())
    }

    fn process_mythic_crest() -> ProgramResult {
        Ok(())
    }

    fn process_ascendant_seal() -> ProgramResult {
        Ok(())
    }

    fn process_code_of_insight() -> ProgramResult {
        Ok(())
    }

    fn process_primordial_relic() -> ProgramResult {
        Ok(())
    }

    pub fn process(
        _program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction = CardRarityInstruction::try_from_slice(instruction_data)
            .map_err(|_| ProgramError::InvalidInstructionData)?;

        match instruction {
            CardRarityInstruction::GenesisFragment => Self::process_genesis_fragment()?,

            CardRarityInstruction::AstralShard => Self::process_astral_shard()?,

            CardRarityInstruction::MythicCrest => Self::process_mythic_crest()?,

            CardRarityInstruction::AscendantSeal => Self::process_ascendant_seal()?,

            CardRarityInstruction::CodexOfInsight => Self::process_code_of_insight()?,

            CardRarityInstruction::PromordialRelic => Self::process_primordial_relic()?,
        }

        Ok(())
    }

    fn process_minting(
        _program_id: &Pubkey,
        accounts: &[AccountInfo],
        _instruction_data: &[u8],
    ) -> ProgramResult {
        let accounts_iter = &mut accounts.iter();

        let mint_account = next_account_info(accounts_iter)?;
        let metadata_account = next_account_info(accounts_iter)?;
        let edition_account = next_account_info(accounts_iter)?;
        let mint_authority = next_account_info(accounts_iter)?;
        let associated_token_account = next_account_info(accounts_iter)?;
        let payer = next_account_info(accounts_iter)?;
        let rent = next_account_info(accounts_iter)?;
        let _system_program = next_account_info(accounts_iter)?;
        let token_program = next_account_info(accounts_iter)?;
        let associated_token_program = next_account_info(accounts_iter)?;
        let token_metadata_program = next_account_info(accounts_iter)?;

        if associated_token_account.lamports() == 0 {
            msg!("Creating associated token account...");

            invoke(
                &associated_token_account_instruction::create_associated_token_account(
                    payer.key,
                    mint_account.key,
                    token_program.key,
                ),
                &[
                    mint_account.clone(),
                    associated_token_account.clone(),
                    payer.clone(),
                    token_program.clone(),
                    associated_token_program.clone(),
                ],
            )?;
        } else {
            msg!("Associated token account exists.");
        }

        msg!("Associated Token Address: {}", associated_token_account.key);

        msg!("Minting NFT to associated token account...");
        invoke(
            &token_instruction::mint_to(
                token_program.key,
                mint_account.key,
                associated_token_account.key,
                mint_authority.key,
                &[mint_authority.key],
                1,
            )?,
            &[
                mint_account.clone(),
                mint_authority.clone(),
                associated_token_account.clone(),
                token_program.clone(),
            ],
        );

        msg!("Creating edition account...");
        msg!("Edition account address: {}", edition_account.key);
        invoke(&mpl_instruction::create_master_edition_v3(
                *token_metadata_program.key,
                *edition_account.key
        ), account_infos)

        Ok(())
    }
}

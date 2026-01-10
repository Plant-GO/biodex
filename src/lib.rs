use borsh::{BorshDeserialize, BorshSerialize};
use instruction::ProgramInstruction;
use mint::create_token;
use processor::Processor;
use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, program_error::ProgramError,
    pubkey::Pubkey,
};

pub mod instruction;
pub mod mint;
pub mod processor;
pub mod types;

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct CounterAccount {
    pub count: u64,
}

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = ProgramInstruction::try_from_slice(instruction_data)
        .map_err(|_| ProgramError::InvalidInstructionData)?;

    match instruction {
        ProgramInstruction::CreateMint { args } => create_token(accounts, args)?,
        ProgramInstruction::MintNFT {
            card_type,
            plant_name,
            is_new_species,
        } => Processor::process(
            program_id,
            accounts,
            card_type,
            plant_name.as_str(),
            is_new_species,
        )?,
    };

    Ok(())
}

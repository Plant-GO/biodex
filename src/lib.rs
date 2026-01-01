use borsh::{BorshDeserialize, BorshSerialize};
use instruction::CardRarityInstruction;
use processor::Processor;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program::invoke,
    program_error::ProgramError,
    pubkey::Pubkey,
    system_instruction,
    sysvar::{rent::Rent, Sysvar},
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
    let instruction = CardRarityInstruction::try_from_slice(instruction_data)
        .map_err(|_| ProgramError::InvalidInstructionData)?;
    Processor::process(program_id, accounts, instruction_data)?;

    Ok(())
}

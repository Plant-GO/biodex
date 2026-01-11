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

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct CounterAccount {
    pub count: u64,
}

// Aba chai jun Ownership data cha tyo chai provide garna milne aeuta kei banaune

entrypoint!(process_instruction);

pub fn process_instruction<'a, 'b, 'c, 'd>(
    program_id: &'a Pubkey,
    accounts: &'b [AccountInfo<'c>],
    instruction_data: &'d [u8],
) -> ProgramResult {
    let accounts: &'b [AccountInfo<'b>] =
        unsafe { std::mem::transmute::<&'b [AccountInfo<'c>], &'b [AccountInfo<'b>]>(accounts) };

    let instruction = ProgramInstruction::try_from_slice(instruction_data)
        .map_err(|_| ProgramError::InvalidInstructionData)?;
    match instruction {
        ProgramInstruction::CreateMint { args } => create_token(accounts, args)?,
        ProgramInstruction::MintNFT {
            card_type,
            plant_name,
            is_new_species,
            quiz_winner,
        } => Processor::process(
            program_id,
            accounts,
            card_type,
            plant_name.as_str(),
            is_new_species,
            quiz_winner,
        )?,
    };
    Ok(())
}

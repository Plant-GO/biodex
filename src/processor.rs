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
use spl_associated_token_account::instruction as associated_token_account_instruction;
use spl_token::instruction as token_instruction;

use crate::instruction::CardRarityInstruction;

pub struct Processor {}

impl Processor {
    pub fn process(
        _program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction = CardRarityInstruction::try_from_slice(instruction_data)
            .map_err(|_| ProgramError::InvalidInstructionData)?;

        match instruction {
            CardRarityInstruction::GenesisFragment => {
                Self::process_minting(accounts, CardRarityInstruction::GenesisFragment)?
            }

            CardRarityInstruction::AstralShard => {
                Self::process_minting(accounts, CardRarityInstruction::AstralShard)?
            }

            CardRarityInstruction::MythicCrest => {
                Self::process_minting(accounts, CardRarityInstruction::MythicCrest)?
            }

            CardRarityInstruction::AscendantSeal => {
                Self::process_minting(accounts, CardRarityInstruction::AscendantSeal)?
            }

            CardRarityInstruction::CodexOfInsight => {
                Self::process_minting(accounts, CardRarityInstruction::CodexOfInsight)?
            }

            CardRarityInstruction::PrimordialRelic => {
                Self::process_minting(accounts, CardRarityInstruction::PrimordialRelic)?
            }
        }

        Ok(())
    }

    fn process_minting(
        accounts: &[AccountInfo],
        card_type: CardRarityInstruction,
    ) -> ProgramResult {
        let accounts_iter = &mut accounts.iter();

        let user_wallet_account = next_account_info(accounts_iter)?;
        let card_supply_account = next_account_info(accounts_iter)?;
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
                    user_wallet_account.key,
                    mint_account.key,
                    token_program.key,
                ),
                &[
                    mint_account.clone(),
                    associated_token_account.clone(),
                    payer.clone(),
                    user_wallet_account.clone(),
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
                token_program.key,            // token program id
                mint_account.key,             // mint pubkey
                associated_token_account.key, // account
                mint_authority.key,           // owner
                &[mint_authority.key],        // signers
                1,
            )?,
            &[
                mint_account.clone(),
                mint_authority.clone(),
                associated_token_account.clone(),
                token_program.clone(),
            ],
        )?;

        msg!("NFT minted successfully");

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{mint, CardRarityInstruction, Processor};
    use borsh::BorshSerialize;
    use solana_program_test::*;
    use solana_sdk::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
        signature::{Keypair, Signer},
        system_program,
        transaction::Transaction,
    };
    use spl_associated_token_account::get_associated_token_address;

    async fn setup_program() -> (ProgramTestContext, Pubkey) {
        let program_id = Pubkey::new_unique();
        let mut program_test =
            ProgramTest::new("program", program_id, processor!(Processor::process));

        let context = program_test.start_with_context().await;
        (context, program_id)
    }

    async fn create_mint_and_metadata(
        context: &mut ProgramTestContext,
        program_id: Pubkey,
        payer: &Keypair,
        user: &Keypair,
        mint: &Keypair,
        mint_authority: &Keypair,
        metadata: &Keypair,
        rarity_title: &str,
    ) {
        let args = CreateTokenArgs {
            nft_title: rarity_title.to_string(),
            nft_symbol: format!("{}", rarity_title.split("-").last().unwrap()),
            nft_uri: "https://example.com".to_string(),
        };

        let accounts = vec![
            AccountMeta::new(mint.pubkey(), true),
            AccountMeta::new(mint_authority.pubkey(), true),
            AccountMeta::new(metadata.pubkey(), false),
            AccountMeta::new(payer.pubkey(), true),
            AccountMeta::new_readonly(solana_sdk::sysvar::rent::id(), false),
            AccountMeta::new_readonly(system_program::id(), false),
            AccountMeta::new_readonly(spl_token::id(), false),
            AccountMeta::new_readonly(spl_token_metadata::id(), false),
        ];

        let instruction_data = borsh::to_vec(&args).unwrap();
        let instruction = Instruction::new_with_bytes(program_id, &instruction_data, accounts);

        let transaction = Transaction::new_with_payer(&[instruction], Some(&payer.pubkey()));
        context
            .banks_client
            .process_transaction(transaction)
            .await
            .unwrap();
    }

    async fn mint_to_user(
        context: &mut ProgramTestContext,
        mint: &Keypair,
        mint_authority: &Keypair,
        user: &Keypair,
        amount: u64,
    ) {
        let ata = get_associated_token_address(&user.pubkey(), &mint.pubkey());
        let instruction = spl_token::instruction::mint_to(
            spl_token::id(),
            &mint.pubkey(),
            &ata,
            &mint_authority.pubkey(),
            &[],
            amount,
        )
        .unwrap();
        let mut transaction =
            Transaction::new_with_payer(&[instruction], Some(&mint_authority.pubkey()));
        transaction.sign(&[mint_authority], context.last_blockhash);
        context
            .banks_client
            .process_transaction(transaction)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_nft_minting() {
        // Create program test with your program
        let program_id = Pubkey::new_unique();
        let mut program_test = ProgramTest::new(
            "program", // Your program name
            program_id,
            processor!(Processor::process),
        );

        // Start test context
        let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

        // Create keypairs
        let user_wallet = Keypair::new();
        let card_supply_account = Keypair::new();

        let common_mint = Keypair::new();
        let rare_mint = Keypair::new();
        let epic_mint = Keypair::new();

        let metadata_common = Keypair::new();
        let edition_common = Keypair::new();

        let metadata_rare = Keypair::new();
        let edition_rare = Keypair::new();

        let metadata_epic = Keypair::new();
        let edition_epic = Keypair::new();

        let mint_authority = Keypair::new();

        let ata_common = Keypair::new();
        let ata_rare = Keypair::new();
        let ata_epic = Keypair::new();

        // Common card instruction
        let common_instruction_data = borsh::to_vec(&CardRarityInstruction::GenesisFragment)
            .expect("Failed to serialize instruction!");
        let common_instruction = Instruction::new_with_bytes(
            program_id,
            &common_instruction_data,
            vec![
                AccountMeta::new(user_wallet.pubkey(), true),
                AccountMeta::new(card_supply_account.pubkey(), false),
                AccountMeta::new(common_mint.pubkey(), true),
                AccountMeta::new(metadata_common.pubkey(), false),
                AccountMeta::new(edition_common.pubkey(), false),
                AccountMeta::new(mint_authority.pubkey(), true),
                AccountMeta::new(ata_common.pubkey(), false),
                AccountMeta::new(payer.pubkey(), true),
                AccountMeta::new_readonly(system_program::id(), false),
                AccountMeta::new_readonly(system_program::id(), false),
                AccountMeta::new_readonly(system_program::id(), false),
                AccountMeta::new_readonly(system_program::id(), false),
                AccountMeta::new_readonly(system_program::id(), false),
            ],
        );

        let mut transaction =
            Transaction::new_with_payer(&[common_instruction], Some(&payer.pubkey()));
        transaction.sign(
            &[&payer, &user_wallet, &mint_authority, &common_mint],
            recent_blockhash,
        );

        let result = banks_client.process_transaction(transaction).await;
        assert!(result.is_ok(), "Common card mint should succeed");
        println!("Common card minted successfully");

        // Rare card instruction
        let rare_instruction_data = borsh::to_vec(&CardRarityInstruction::AstralShard)
            .expect("Failed to serialize instruction!");
        let rare_instruction = Instruction::new_with_bytes(
            program_id,
            &rare_instruction_data,
            vec![
                AccountMeta::new(user_wallet.pubkey(), true),
                AccountMeta::new(card_supply_account.pubkey(), false),
                AccountMeta::new(rare_mint.pubkey(), true),
                AccountMeta::new(metadata_rare.pubkey(), false),
                AccountMeta::new(edition_rare.pubkey(), false),
                AccountMeta::new(mint_authority.pubkey(), true),
                AccountMeta::new(ata_rare.pubkey(), false),
                AccountMeta::new(payer.pubkey(), true),
                AccountMeta::new_readonly(system_program::id(), false),
                AccountMeta::new_readonly(system_program::id(), false),
                AccountMeta::new_readonly(system_program::id(), false),
                AccountMeta::new_readonly(system_program::id(), false),
                AccountMeta::new_readonly(system_program::id(), false),
            ],
        );

        let mut transaction =
            Transaction::new_with_payer(&[rare_instruction], Some(&payer.pubkey()));
        transaction.sign(
            &[&payer, &user_wallet, &mint_authority, &rare_mint],
            recent_blockhash,
        );

        let result = banks_client.process_transaction(transaction).await;
        assert!(result.is_ok(), "Rare card mint should succeed");
        println!("Rare card minted successfully");

        // Epic card instruction
        let epic_instruction_data = borsh::to_vec(&CardRarityInstruction::MythicCrest)
            .expect("Failed to serialize instruction!");
        let epic_instruction = Instruction::new_with_bytes(
            program_id,
            &epic_instruction_data,
            vec![
                AccountMeta::new(user_wallet.pubkey(), true),
                AccountMeta::new(card_supply_account.pubkey(), false),
                AccountMeta::new(epic_mint.pubkey(), true),
                AccountMeta::new(metadata_epic.pubkey(), false),
                AccountMeta::new(edition_epic.pubkey(), false),
                AccountMeta::new(mint_authority.pubkey(), true),
                AccountMeta::new(ata_epic.pubkey(), false),
                AccountMeta::new(payer.pubkey(), true),
                AccountMeta::new_readonly(system_program::id(), false),
                AccountMeta::new_readonly(system_program::id(), false),
                AccountMeta::new_readonly(system_program::id(), false),
                AccountMeta::new_readonly(system_program::id(), false),
                AccountMeta::new_readonly(system_program::id(), false),
            ],
        );

        let mut transaction =
            Transaction::new_with_payer(&[epic_instruction], Some(&payer.pubkey()));
        transaction.sign(
            &[&payer, &user_wallet, &mint_authority, &epic_mint],
            recent_blockhash,
        );

        let result = banks_client.process_transaction(transaction).await;
        assert!(result.is_ok(), "Epic card mint should succeed");
        println!("Epic card minted successfully");
    }
}

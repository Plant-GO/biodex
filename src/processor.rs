use crate::instruction::{CardRarityInstruction, PlantCounter};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::{self, ProgramResult},
    msg,
    program::invoke,
    program_error::ProgramError,
    pubkey::Pubkey,
    system_instruction,
    sysvar::{rent::Rent, Sysvar},
};
use solana_sdk::program::invoke_signed;
use spl_associated_token_account::instruction as associated_token_account_instruction;
use spl_token::instruction as token_instruction;

const MAX_EPIC: u64 = 20;
const MAX_RARE: u64 = 50;

pub struct Processor {}

impl Processor {
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction: CardRarityInstruction,
        plant_name: &str,
    ) -> ProgramResult {
        match instruction {
            CardRarityInstruction::GenesisFragment => Self::process_minting(
                program_id,
                accounts,
                CardRarityInstruction::GenesisFragment,
                plant_name,
            )?,

            CardRarityInstruction::AstralShard => Self::process_minting(
                program_id,
                accounts,
                CardRarityInstruction::AstralShard,
                plant_name,
            )?,

            CardRarityInstruction::MythicCrest => Self::process_minting(
                program_id,
                accounts,
                CardRarityInstruction::MythicCrest,
                plant_name,
            )?,

            CardRarityInstruction::AscendantSeal => Self::process_minting(
                program_id,
                accounts,
                CardRarityInstruction::AscendantSeal,
                plant_name,
            )?,

            CardRarityInstruction::CodexOfInsight => Self::process_minting(
                program_id,
                accounts,
                CardRarityInstruction::CodexOfInsight,
                plant_name,
            )?,

            CardRarityInstruction::PrimordialRelic => Self::process_minting(
                program_id,
                accounts,
                CardRarityInstruction::PrimordialRelic,
                plant_name,
            )?,
        }

        Ok(())
    }

    fn process_minting(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        card_type: CardRarityInstruction,
        requested_rarity: CardRarityInstruction,
        plant_name: &str,
    ) -> ProgramResult {
        let accounts_iter = &mut accounts.iter();

        let user_wallet_account = next_account_info(accounts_iter)?;
        let common_mint_account = next_account_info(accounts_iter)?;
        let rare_mint_account = next_account_info(accounts_iter)?;
        let epic_mint_account = next_account_info(accounts_iter)?;
        let metadata_account = next_account_info(accounts_iter)?;
        let edition_account = next_account_info(accounts_iter)?;
        let mint_authority = next_account_info(accounts_iter)?;
        let associated_token_account = next_account_info(accounts_iter)?;
        let payer = next_account_info(accounts_iter)?;
        let rent = next_account_info(accounts_iter)?;
        let system_program = next_account_info(accounts_iter)?;
        let token_program = next_account_info(accounts_iter)?;
        let associated_token_program = next_account_info(accounts_iter)?;
        let token_metadata_program = next_account_info(accounts_iter)?;
        let ownership_account = next_account_info(accounts_iter)?;
        let plant_counter_account = next_account_info(accounts_iter)?;

        let (ownership_pda, bump) = Pubkey::find_program_address(
            &[plant_name.as_bytes(), user_wallet_account.key.as_ref()],
            program_id,
        );

        if ownership_pda != *ownership_account.key {
            msg!("Ownership account does not match derived PDA");
            return Err(ProgramError::InvalidArgument);
        }

        if ownership_account.lamports() > 0 {
            msg!("User already owns this card for plant: {}", plant_name);
            return Err(ProgramError::Custom(999));
        }

        let (plant_counter_pda, bump) =
            Pubkey::find_program_address(&[b"plant_counter", plant_name.as_bytes()], program_id);

        if plant_counter_pda != *plant_counter_account.key {
            return Err(ProgramError::InvalidArgument);
        }

        let mut counter = if plant_counter_account.data_is_empty() {
            PlantCounter {
                plant_name: plant_name.to_string(),
                epic_count: 0,
                rare_count: 0,
                common_count: 0,
                first_minter: None,
            }
        } else {
            PlantCounter::try_from_slice(&plant_counter_account.data.borrow())?
        };

        let (final_rarity, mint_account) = match requested_rarity {
            CardRarityInstruction::MythicCrest if counter.epic_count < MAX_EPIC => {
                counter.epic_count += 1;
                (CardRarityInstruction::MythicCrest, epic_mint_account)
            }
            CardRarityInstruction::MythicCrest | CardRarityInstruction::AstralShard
                if counter.rare < MAX_RARE =>
            {
                counter.rare_count += 1;
                (CardRarityInstruction::AstralShard, rare_mint_account)
            }
            _ => {
                counter.common_count += 1;
                (CardRarityInstruction::GenesisFragment, common_mint_account)
            }
        };

        msg!("Minting {:?} card for plant {}", final_rarity, plant_name);

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
                    payer.clone(),                    // 0
                    associated_token_account.clone(), // 1
                    user_wallet_account.clone(),      // 2
                    mint_account.clone(),             // 3
                    system_program.clone(),           // 4
                    token_program.clone(),            // 5
                    rent.clone(),                     // 6
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

        invoke_signed(
            &system_instruction::create_account(
                &payer.key,
                &ownership_account.key,
                Rent::get()?.minimum_balance(0),
                0, // space
                program_id,
            ),
            &[
                payer.clone(),
                system_program.clone(),
                ownership_account.clone(),
            ],
            &[&[
                plant_name.as_bytes(),
                user_wallet_account.key.as_ref(),
                &[bump],
            ]],
        )?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::{
        instruction::CardRarityInstruction, instruction::ProgramInstruction, mint::CreateTokenArgs,
        process_instruction,
    };
    use borsh::BorshSerialize;
    use solana_program_test::*;
    use solana_sdk::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
        signature::{Keypair, Signer},
        system_instruction, system_program,
        transaction::Transaction,
    };
    use spl_associated_token_account::get_associated_token_address;
    use spl_token::id as token_program_id;

    #[tokio::test]
    async fn test_nft_creation_and_minting() {
        let program_id = Pubkey::new_unique();
        let program_test = ProgramTest::new("program", program_id, processor!(process_instruction));

        let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

        let user_wallet = Keypair::new();
        let mint_authority = Keypair::new();

        let fund_user_ix =
            system_instruction::transfer(&payer.pubkey(), &user_wallet.pubkey(), 1_000_000_000);
        let mut fund_tx = Transaction::new_with_payer(&[fund_user_ix], Some(&payer.pubkey()));
        fund_tx.sign(&[&payer], recent_blockhash);
        banks_client.process_transaction(fund_tx).await.unwrap();

        let fund_authority_ix =
            system_instruction::transfer(&payer.pubkey(), &mint_authority.pubkey(), 1_000_000_000);
        let mut fund_tx = Transaction::new_with_payer(&[fund_authority_ix], Some(&payer.pubkey()));
        fund_tx.sign(&[&payer], recent_blockhash);
        banks_client.process_transaction(fund_tx).await.unwrap();

        let common_mint = Keypair::new();
        let rare_mint = Keypair::new();
        let epic_mint = Keypair::new();

        let metadata_common = Keypair::new();
        let metadata_rare = Keypair::new();
        let metadata_epic = Keypair::new();

        let edition_common = Keypair::new();
        let edition_rare = Keypair::new();
        let edition_epic = Keypair::new();

        for (mint, metadata, title, symbol) in &[
            (&common_mint, &metadata_common, "Orange", "GenesisFragment"),
            (&rare_mint, &metadata_rare, "Marigold", "AstralShard"),
            (&epic_mint, &metadata_epic, "Sunflower", "MythicCrest"),
        ] {
            let create_args = CreateTokenArgs {
                nft_title: title.to_string(),
                nft_symbol: symbol.to_string(),
                nft_uri: "https://example.com/nft.json".to_string(),
            };

            let data = ProgramInstruction::CreateMint { args: create_args }
                .try_to_vec()
                .expect("Failed to serialize CreateMint instruction");

            let instruction = Instruction::new_with_bytes(
                program_id,
                &data,
                vec![
                    AccountMeta::new(mint.pubkey(), true),
                    AccountMeta::new(mint_authority.pubkey(), true),
                    AccountMeta::new(metadata.pubkey(), false),
                    AccountMeta::new(payer.pubkey(), true),
                    AccountMeta::new_readonly(solana_sdk::sysvar::rent::id(), false),
                    AccountMeta::new_readonly(system_program::id(), false),
                    AccountMeta::new_readonly(token_program_id(), false),
                    AccountMeta::new_readonly(system_program::id(), false),
                ],
            );

            let recent_blockhash = banks_client.get_latest_blockhash().await.unwrap();
            let mut tx = Transaction::new_with_payer(&[instruction], Some(&payer.pubkey()));
            tx.sign(&[&payer, &mint_authority, mint], recent_blockhash);

            let result = banks_client.process_transaction(tx).await;
            assert!(
                result.is_ok(),
                "Failed to create mint: {} - {:?}",
                title,
                result.err()
            );
            println!("Mint created: {}", title);
        }

        let plant_name = "Sunflower";
        let ownership_common = Pubkey::find_program_address(
            &[plant_name.as_bytes(), user_wallet.pubkey().as_ref()],
            &program_id,
        )
        .0;

        for (mint, edition, card_type, metadata, name) in &[
            (
                &common_mint,
                &edition_common,
                CardRarityInstruction::GenesisFragment,
                &metadata_common,
                "GenesisFragment",
            ),
            (
                &rare_mint,
                &edition_rare,
                CardRarityInstruction::AstralShard,
                &metadata_rare,
                "AstralShard",
            ),
            (
                &epic_mint,
                &edition_epic,
                CardRarityInstruction::MythicCrest,
                &metadata_epic,
                "MythicCrest",
            ),
        ] {
            let data = ProgramInstruction::MintNFT {
                card_type: card_type.clone(),
                plant_name: "Sunflower".to_string(),
            }
            .try_to_vec()
            .expect("Failed to serialize MintNFT instruction");

            let ata = get_associated_token_address(&user_wallet.pubkey(), &mint.pubkey());

            let instruction = Instruction::new_with_bytes(
                program_id,
                &data,
                vec![
                    AccountMeta::new(user_wallet.pubkey(), true),
                    AccountMeta::new(mint.pubkey(), false),
                    AccountMeta::new(metadata.pubkey(), false),
                    AccountMeta::new(edition.pubkey(), false),
                    AccountMeta::new(mint_authority.pubkey(), true),
                    AccountMeta::new(ata, false),
                    AccountMeta::new(payer.pubkey(), true),
                    AccountMeta::new_readonly(solana_sdk::sysvar::rent::id(), false),
                    AccountMeta::new_readonly(system_program::id(), false),
                    AccountMeta::new_readonly(token_program_id(), false),
                    AccountMeta::new_readonly(spl_associated_token_account::id(), false),
                    AccountMeta::new_readonly(system_program::id(), false),
                    AccountMeta::new(ownership_common, false),
                ],
            );

            let recent_blockhash = banks_client.get_latest_blockhash().await.unwrap();
            let mut tx = Transaction::new_with_payer(&[instruction], Some(&payer.pubkey()));
            tx.sign(&[&payer, &user_wallet, &mint_authority], recent_blockhash);

            let result = banks_client.process_transaction(tx).await;
            assert!(
                result.is_ok(),
                "Failed to mint NFT {}: {:?}",
                name,
                result.err()
            );
            println!("NFT minted successfully: {}", name);
        }

        for (mint, edition, card_type, metadata, name) in &[
            (
                &common_mint,
                &edition_common,
                CardRarityInstruction::GenesisFragment,
                &metadata_common,
                "GenesisFragment",
            ),
            (
                &rare_mint,
                &edition_rare,
                CardRarityInstruction::AstralShard,
                &metadata_rare,
                "AstralShard",
            ),
            (
                &epic_mint,
                &edition_epic,
                CardRarityInstruction::MythicCrest,
                &metadata_epic,
                "MythicCrest",
            ),
        ] {
            let data = ProgramInstruction::MintNFT {
                card_type: card_type.clone(),
                plant_name: "Sunflower".to_string(),
            }
            .try_to_vec()
            .expect("Failed to serialize MintNFT instruction");

            let ata = get_associated_token_address(&user_wallet.pubkey(), &mint.pubkey());

            let instruction = Instruction::new_with_bytes(
                program_id,
                &data,
                vec![
                    AccountMeta::new(user_wallet.pubkey(), true),
                    AccountMeta::new(mint.pubkey(), false),
                    AccountMeta::new(metadata.pubkey(), false),
                    AccountMeta::new(edition.pubkey(), false),
                    AccountMeta::new(mint_authority.pubkey(), true),
                    AccountMeta::new(ata, false),
                    AccountMeta::new(payer.pubkey(), true),
                    AccountMeta::new_readonly(solana_sdk::sysvar::rent::id(), false),
                    AccountMeta::new_readonly(system_program::id(), false),
                    AccountMeta::new_readonly(token_program_id(), false),
                    AccountMeta::new_readonly(spl_associated_token_account::id(), false),
                    AccountMeta::new_readonly(system_program::id(), false),
                    AccountMeta::new(ownership_common, false),
                ],
            );

            let recent_blockhash = banks_client.get_latest_blockhash().await.unwrap();
            let mut tx = Transaction::new_with_payer(&[instruction], Some(&payer.pubkey()));
            tx.sign(&[&payer, &user_wallet, &mint_authority], recent_blockhash);

            let result = banks_client.process_transaction(tx).await;
            if result.is_ok() {
                println!("Duplicate mint allowed for GenesisFragment");
            } else {
                println!(
                    "Duplicate mint rejected for GenesisFragment: {:?}",
                    result.err()
                );
            }
        }

        println!("\nAll tests passed!");
    }
}

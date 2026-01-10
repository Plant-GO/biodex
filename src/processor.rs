use crate::instruction::{CardRarityInstruction, PlantCounter};
use borsh::BorshDeserialize;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
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
use tokio::time::Sleep;

const MAX_EPIC: u64 = 20;
const MAX_RARE: u64 = 50;

pub struct Processor {}

impl Processor {
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction: CardRarityInstruction,
        plant_name: &str,
        is_new_species: bool,
    ) -> ProgramResult {
        match instruction {
            CardRarityInstruction::GenesisFragment => Self::process_minting(
                program_id,
                accounts,
                CardRarityInstruction::GenesisFragment,
                plant_name,
                is_new_species,
            )?,

            CardRarityInstruction::AstralShard => Self::process_minting(
                program_id,
                accounts,
                CardRarityInstruction::AstralShard,
                plant_name,
                is_new_species,
            )?,

            CardRarityInstruction::MythicCrest => Self::process_minting(
                program_id,
                accounts,
                CardRarityInstruction::MythicCrest,
                plant_name,
                is_new_species,
            )?,

            CardRarityInstruction::AscendantSeal => Self::process_minting(
                program_id,
                accounts,
                CardRarityInstruction::AscendantSeal,
                plant_name,
                is_new_species,
            )?,

            CardRarityInstruction::CodexOfInsight => Self::process_minting(
                program_id,
                accounts,
                CardRarityInstruction::CodexOfInsight,
                plant_name,
                is_new_species,
            )?,

            CardRarityInstruction::PrimordialRelic => Self::process_minting(
                program_id,
                accounts,
                CardRarityInstruction::PrimordialRelic,
                plant_name,
                is_new_species,
            )?,

            CardRarityInstruction::AuroraSeed => Self::process_minting(
                program_id,
                accounts,
                CardRarityInstruction::PrimordialRelic,
                plant_name,
                is_new_species,
            )?,
        }

        Ok(())
    }

    fn process_ownership_account(
        ownership_account: &AccountInfo,
        program_id: &Pubkey,
        plant_name: &str,
        card_type: CardRarityInstruction,
        user_wallet_account: &AccountInfo,
    ) -> Result<(Pubkey, u8), ProgramError> {
        let (ownership_pda, ownership_bump) = Pubkey::find_program_address(
            &[
                plant_name.as_bytes(),
                user_wallet_account.key.as_ref(),
                &[card_type.clone() as u8],
            ],
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

        Ok((ownership_pda, ownership_bump))
    }

    fn process_plant_counter_pda(
        plant_name: &str,
        program_id: &Pubkey,
        plant_counter_account: &AccountInfo,
    ) -> Result<(Pubkey, u8), ProgramError> {
        let (plant_counter_pda, _plant_counter_bump) =
            Pubkey::find_program_address(&[b"plant_counter", plant_name.as_bytes()], program_id);

        if plant_counter_pda != *plant_counter_account.key {
            return Err(ProgramError::InvalidArgument);
        }

        Ok((plant_counter_pda, _plant_counter_bump))
    }

    fn process_minting(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        card_type: CardRarityInstruction,
        plant_name: &str,
        is_new_species: bool,
    ) -> ProgramResult {
        let accounts_iter = &mut accounts.iter();

        let user_wallet_account = next_account_info(accounts_iter)?;
        let common_mint_account = next_account_info(accounts_iter)?;
        let rare_mint_account = next_account_info(accounts_iter)?;
        let epic_mint_account = next_account_info(accounts_iter)?;
        let aurora_mint_account = next_account_info(accounts_iter)?;
        let primordial_mint_account = next_account_info(accounts_iter)?;
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

        let (_ownership_pda, ownership_bump) = Self::process_ownership_account(
            ownership_account,
            program_id,
            plant_name,
            card_type.clone(),
            user_wallet_account,
        )
        .unwrap();

        let (_plant_counter_pda, _) =
            Self::process_plant_counter_pda(plant_name, program_id, plant_counter_account).unwrap();

        let is_first_on_chain = plant_counter_account.data_is_empty();

        let mut counter = if is_first_on_chain {
            PlantCounter {
                plant_name: plant_name.to_string(),
                seed_count: 0,
                relic_count: 0,
                epic_count: 0,
                rare_count: 0,
                common_count: 0,
                mastery_count: 0,
                codex_count: 0,
                first_minter: None,
            }
        } else {
            PlantCounter::try_from_slice(&plant_counter_account.data.borrow())?
        };

        let (final_rarity, mint_account) =
            if is_new_species && is_first_on_chain && counter.seed_count == 0 {
                msg!("AuroraSeed AWARDED!");
                msg!("This is a brand new species discovery!");
                counter.first_minter = Some(*user_wallet_account.key);
                counter.epic_count += 1;
                (CardRarityInstruction::PrimordialRelic, aurora_mint_account)
            } else if !is_new_species && is_first_on_chain && counter.relic_count == 0 {
                msg!("PrimordialRelic AWARDED!");
                msg!("First person to photograph this known plant!");
                counter.epic_count += 1;
                counter.first_minter = Some(*user_wallet_account.key);
                (
                    CardRarityInstruction::AscendantSeal,
                    primordial_mint_account,
                )
            } else {
                msg!("Regular rarity distribution");

                if counter.epic_count < MAX_EPIC {
                    counter.epic_count += 1;
                    (CardRarityInstruction::MythicCrest, epic_mint_account)
                } else if counter.rare_count < MAX_RARE {
                    counter.rare_count += 1;
                    (CardRarityInstruction::AstralShard, rare_mint_account)
                } else {
                    counter.common_count += 1;
                    (CardRarityInstruction::GenesisFragment, common_mint_account)
                }
            };

        msg!("Final rarity: {:?}", final_rarity);

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
                    payer.clone(),
                    associated_token_account.clone(),
                    user_wallet_account.clone(),
                    mint_account.clone(),
                    system_program.clone(),
                    token_program.clone(),
                    rent.clone(),
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
        )?;

        msg!("NFT minted successfully");

        invoke_signed(
            &system_instruction::create_account(
                &payer.key,
                &ownership_account.key,
                Rent::get()?.minimum_balance(0),
                0,
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
                &[card_type.clone() as u8],
                &[ownership_bump],
            ]],
        )?;

        Ok(())
    }

    fn process_quiz(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        card_type: CardRarityInstruction,
        plant_name: &str,
        is_winner: bool,
    ) -> ProgramResult {
        let accounts_iter = &mut accounts.iter();

        let user_wallet_account = next_account_info(accounts_iter)?;
        let common_mint_account = next_account_info(accounts_iter)?;
        let rare_mint_account = next_account_info(accounts_iter)?;
        let epic_mint_account = next_account_info(accounts_iter)?;
        let ascendant_mint_account = next_account_info(accounts_iter)?;
        let primordial_mint_account = next_account_info(accounts_iter)?;
        let codex_mint_account = next_account_info(accounts_iter)?; // Knowledge card mint
        let mastery_mint_account = next_account_info(accounts_iter)?; // Mastery card mint
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

        let (_, ownership_bump) = Self::process_ownership_account(
            ownership_account,
            program_id,
            plant_name,
            card_type,
            user_wallet_account,
        )
        .unwrap();

        let (_plant_counter_pda, _) =
            Self::process_plant_counter_pda(plant_name, program_id, plant_counter_account).unwrap();

        let mut counter = if plant_counter_account.data_is_empty() {
            PlantCounter {
                plant_name: plant_name.to_string(),
                epic_count: 0,
                rare_count: 0,
                seed_count: 0,
                relic_count: 0,
                common_count: 0,
                first_minter: None,
                codex_count: 0,
                mastery_count: 0,
            }
        } else {
            PlantCounter::try_from_slice(&plant_counter_account.data.borrow())?
        };

        let (final_card, mint_account) = if is_winner {
            msg!("MythicCrest AWARDED - Quiz Winner!");
            counter.mastery_count += 1;
            (CardRarityInstruction::MythicCrest, mastery_mint_account)
        } else {
            msg!("CodexOfInsight AWARDED - Quiz Participation!");
            counter.codex_count += 1;
            (CardRarityInstruction::CodexOfInsight, codex_mint_account)
        };

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
                    payer.clone(),
                    associated_token_account.clone(),
                    user_wallet_account.clone(),
                    mint_account.clone(),
                    system_program.clone(),
                    token_program.clone(),
                    rent.clone(),
                ],
            )?;
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
        )?;

        msg!("NFT minted successfully");

        invoke_signed(
            &system_instruction::create_account(
                &payer.key,
                &ownership_account.key,
                Rent::get()?.minimum_balance(0),
                0,
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
                &[card_type.clone() as u8],
                &[ownership_bump],
            ]],
        )?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        instruction::{CardRarityInstruction, ProgramInstruction},
        mint::CreateTokenArgs,
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
    async fn test_nft_creation_and_minting_with_rarity() {
        let program_id = Pubkey::new_unique();

        let program_test = ProgramTest::new("program", program_id, processor!(process_instruction));

        let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

        let user_wallet = Keypair::new();
        let mint_authority = Keypair::new();

        // Fund user & authority
        for kp in [&user_wallet, &mint_authority] {
            let ix = system_instruction::transfer(&payer.pubkey(), &kp.pubkey(), 1_000_000_000);
            let mut tx = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));
            tx.sign(&[&payer], recent_blockhash);
            banks_client.process_transaction(tx).await.unwrap();
        }

        // Mints
        let common_mint = Keypair::new();
        let rare_mint = Keypair::new();
        let epic_mint = Keypair::new();

        let metadata_common = Keypair::new();
        let metadata_rare = Keypair::new();
        let metadata_epic = Keypair::new();

        let edition_common = Keypair::new();
        let edition_rare = Keypair::new();
        let edition_epic = Keypair::new();

        let mint_data = [
            (&common_mint, &metadata_common, "Orange", "GenesisFragment"),
            (&rare_mint, &metadata_rare, "Marigold", "AstralShard"),
            (&epic_mint, &metadata_epic, "Sunflower", "MythicCrest"),
        ];

        for (mint, metadata, title, symbol) in mint_data {
            let args = CreateTokenArgs {
                nft_title: title.to_string(),
                nft_symbol: symbol.to_string(),
                nft_uri: "https://example.com/nft.json".to_string(),
            };

            let data = ProgramInstruction::CreateMint { args }
                .try_to_vec()
                .unwrap();

            let ix = Instruction::new_with_bytes(
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
                ],
            );

            let blockhash = banks_client.get_latest_blockhash().await.unwrap();
            let mut tx = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));
            tx.sign(&[&payer, &mint_authority, mint], blockhash);
            banks_client.process_transaction(tx).await.unwrap();
        }

        let plant_name = "Sunflower";

        let nft_sets = [
            (
                &common_mint,
                &metadata_common,
                &edition_common,
                CardRarityInstruction::GenesisFragment,
            ),
            (
                &rare_mint,
                &metadata_rare,
                &edition_rare,
                CardRarityInstruction::AstralShard,
            ),
            (
                &epic_mint,
                &metadata_epic,
                &edition_epic,
                CardRarityInstruction::MythicCrest,
            ),
        ];

        for (mint, metadata, edition, rarity) in nft_sets {
            let ata = get_associated_token_address(&user_wallet.pubkey(), &mint.pubkey());

            // Correct PDA derivation according to your program
            let ownership_pda = Pubkey::find_program_address(
                &[
                    plant_name.as_bytes(),
                    user_wallet.pubkey().as_ref(),
                    &[rarity.clone() as u8],
                ],
                &program_id,
            )
            .0;

            let plant_counter_pda = Pubkey::find_program_address(
                &[b"plant_counter", plant_name.as_bytes()],
                &program_id,
            )
            .0;

            let ix = Instruction::new_with_bytes(
                program_id,
                &ProgramInstruction::MintNFT {
                    card_type: rarity.clone(),
                    plant_name: plant_name.to_string(),
                    is_new_species: false,
                }
                .try_to_vec()
                .unwrap(),
                vec![
                    AccountMeta::new(user_wallet.pubkey(), true),
                    AccountMeta::new(common_mint.pubkey(), false),
                    AccountMeta::new(rare_mint.pubkey(), false),
                    AccountMeta::new(epic_mint.pubkey(), false),
                    AccountMeta::new(metadata.pubkey(), false),
                    AccountMeta::new(edition.pubkey(), false),
                    AccountMeta::new(mint_authority.pubkey(), true),
                    AccountMeta::new(
                        get_associated_token_address(&user_wallet.pubkey(), &mint.pubkey()),
                        false,
                    ),
                    AccountMeta::new(payer.pubkey(), true),
                    AccountMeta::new_readonly(solana_sdk::sysvar::rent::id(), false),
                    AccountMeta::new_readonly(system_program::id(), false),
                    AccountMeta::new_readonly(token_program_id(), false),
                    AccountMeta::new_readonly(spl_associated_token_account::id(), false),
                    AccountMeta::new_readonly(spl_associated_token_account::id(), false),
                    AccountMeta::new(ownership_pda, false),
                    AccountMeta::new(plant_counter_pda, false),
                ],
            );

            let blockhash = banks_client.get_latest_blockhash().await.unwrap();
            let mut tx = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));
            tx.sign(&[&payer, &user_wallet, &mint_authority], blockhash);

            assert!(
                banks_client.process_transaction(tx).await.is_ok(),
                "Mint failed for {:?}",
                rarity
            );
        }

        let duplicate_data = ProgramInstruction::MintNFT {
            card_type: CardRarityInstruction::GenesisFragment,
            plant_name: plant_name.to_string(),
            is_new_species: false,
        }
        .try_to_vec()
        .unwrap();

        let ownership_pda = Pubkey::find_program_address(
            &[
                plant_name.as_bytes(),
                user_wallet.pubkey().as_ref(),
                &[CardRarityInstruction::GenesisFragment as u8],
            ],
            &program_id,
        )
        .0;

        let plant_counter_pda =
            Pubkey::find_program_address(&[b"plant_counter", plant_name.as_bytes()], &program_id).0;

        let duplicate_ix = Instruction::new_with_bytes(
            program_id,
            &duplicate_data,
            vec![
                AccountMeta::new(user_wallet.pubkey(), true),
                AccountMeta::new(common_mint.pubkey(), false),
                AccountMeta::new(rare_mint.pubkey(), false),
                AccountMeta::new(epic_mint.pubkey(), false),
                AccountMeta::new(metadata_common.pubkey(), false),
                AccountMeta::new(edition_common.pubkey(), false),
                AccountMeta::new(mint_authority.pubkey(), true),
                AccountMeta::new(
                    get_associated_token_address(&user_wallet.pubkey(), &common_mint.pubkey()),
                    false,
                ),
                AccountMeta::new(payer.pubkey(), true),
                AccountMeta::new_readonly(solana_sdk::sysvar::rent::id(), false),
                AccountMeta::new_readonly(system_program::id(), false),
                AccountMeta::new_readonly(token_program_id(), false),
                AccountMeta::new_readonly(spl_associated_token_account::id(), false),
                AccountMeta::new_readonly(spl_associated_token_account::id(), false),
                AccountMeta::new(ownership_pda, false),
                AccountMeta::new(plant_counter_pda, false),
            ],
        );

        let blockhash = banks_client.get_latest_blockhash().await.unwrap();
        let mut tx = Transaction::new_with_payer(&[duplicate_ix], Some(&payer.pubkey()));
        tx.sign(&[&payer, &user_wallet, &mint_authority], blockhash);

        assert!(
            banks_client.process_transaction(tx).await.is_err(),
            "Duplicate mint should fail"
        );

        println!("All NFT minting tests passed!");
    }
}

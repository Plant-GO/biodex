#[cfg(test)]
mod tests {
    use biodex::{
        instruction::{CardRarityInstruction, OwnershipRecord, PlantCounter, ProgramInstruction},
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
    // //
    // // async fn setup_test_environment() -> (BanksClient, Keypair, Pubkey, solana_sdk::hash::Hash) {
    // //     let program_id = Pubkey::new_unique();
    // //
    // //     let program_test = ProgramTest::new("program", program_id, processor!(process_instruction));
    // //
    // //     let (mut banks_client, payer, recent_blockhash) = program_test.start().await;
    // //
    // //     (banks_client, payer, program_id, recent_blockhash)
    // }

    async fn create_funded_keypair(
        banks_client: &mut BanksClient,
        payer: &Keypair,
        amount: u64,
    ) -> Keypair {
        let keypair = Keypair::new();
        let blockhash = banks_client.get_latest_blockhash().await.unwrap();
        let ix = system_instruction::transfer(&payer.pubkey(), &keypair.pubkey(), amount);
        let mut tx = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));
        tx.sign(&[&payer], blockhash);
        banks_client.process_transaction(tx).await.unwrap();

        keypair
    }

    async fn create_single_mint(
        banks_client: &mut BanksClient,
        payer: &Keypair,
        program_id: Pubkey,
        mint_authority: &Keypair,
        title: &str,
        symbol: &str,
    ) -> Keypair {
        let mint = Keypair::new();
        let args = CreateTokenArgs {
            nft_title: title.to_string(),
            nft_symbol: symbol.to_string(),
            nft_uri: "https://example.com/nft.json".to_string(),
        };

        let data = ProgramInstruction::CreateMint { args }
            .try_to_vec()
            .unwrap();

        let instruction = Instruction::new_with_bytes(
            program_id,
            &data,
            vec![
                AccountMeta::new(mint.pubkey(), true),
                AccountMeta::new(mint_authority.pubkey(), true),
                AccountMeta::new(payer.pubkey(), true),
                AccountMeta::new_readonly(solana_sdk::sysvar::rent::id(), false),
                AccountMeta::new_readonly(system_program::id(), false),
                AccountMeta::new_readonly(token_program_id(), false),
            ],
        );

        let blockhash = banks_client.get_latest_blockhash().await.unwrap();
        let mut tx = Transaction::new_with_payer(&[instruction], Some(&payer.pubkey()));
        tx.sign(&[&payer, &mint_authority, &&mint], blockhash);
        banks_client.process_transaction(tx).await.unwrap();

        mint
    }

    // async fn create_all_mints(banks_client: &mut BanksClient,
    //     payer: &Keypair,
    //     program_id: Pubkey,
    //     mint_authority: &Keypair,) -> (Keypair, Keypair, Keypair, Keypair, Keypair, Keypair, Keypair) {
    //     let common = create_single_mint(banks_client, payer, program_id, mint_authority, "", symbol)
    // }

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

        let common_mint = Keypair::new();
        let rare_mint = Keypair::new();
        let epic_mint = Keypair::new();
        let aurora_mint = Keypair::new();
        let primordial_mint = Keypair::new();
        // let metadata_common = Keypair::new();
        // let metadata_rare = Keypair::new();
        // let metadata_epic = Keypair::new();
        //
        // let edition_common = Keypair::new();
        // let edition_rare = Keypair::new();
        // let edition_epic = Keypair::new();

        let mint_data = [
            (&common_mint, "Orange", "GenesisFragment"),
            (&rare_mint, "Marigold", "AstralShard"),
            (&epic_mint, "Sunflower", "MythicCrest"),
            (&aurora_mint, "Kaggle", "AuroraSeed"),
            (&primordial_mint, "Rose", "PromordialRelic"),
        ];

        for (mint, title, symbol) in mint_data {
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

        //         let plant_name = "Sunflower";
        //
        //         let nft_sets = [
        //             (
        //                 &common_mint,
        //                 &metadata_common,
        //                 &edition_common,
        //                 CardRarityInstruction::GenesisFragment,
        //             ),
        //             (
        //                 &rare_mint,
        //                 &metadata_rare,
        //                 &edition_rare,
        //                 CardRarityInstruction::AstralShard,
        //             ),
        //             (
        //                 &epic_mint,
        //                 &metadata_epic,
        //                 &edition_epic,
        //                 CardRarityInstruction::MythicCrest,
        //             ),
        //         ];
        //
        //         for (mint, metadata, edition, rarity) in nft_sets {
        //             let ata = get_associated_token_address(&user_wallet.pubkey(), &mint.pubkey());
        //
        //             // Correct PDA derivation according to your program
        //             let ownership_pda = Pubkey::find_program_address(
        //                 &[
        //                     plant_name.as_bytes(),
        //                     user_wallet.pubkey().as_ref(),
        //                     &[rarity.clone() as u8],
        //                 ],
        //                 &program_id,
        //             )
        //             .0;
        //
        //             let plant_counter_pda = Pubkey::find_program_address(
        //                 &[b"plant_counter", plant_name.as_bytes()],
        //                 &program_id,
        //             )
        //             .0;
        //
        //             let ix = Instruction::new_with_bytes(
        //                 program_id,
        //                 &ProgramInstruction::MintNFT {
        //                     card_type: rarity.clone(),
        //                     plant_name: plant_name.to_string(),
        //                     is_new_species: false,
        //                 }
        //                 .try_to_vec()
        //                 .unwrap(),
        //                 vec![
        //                     AccountMeta::new(user_wallet.pubkey(), true),
        //                     AccountMeta::new(common_mint.pubkey(), false),
        //                     AccountMeta::new(rare_mint.pubkey(), false),
        //                     AccountMeta::new(epic_mint.pubkey(), false),
        //                     AccountMeta::new(metadata.pubkey(), false),
        //                     AccountMeta::new(edition.pubkey(), false),
        //                     AccountMeta::new(mint_authority.pubkey(), true),
        //                     AccountMeta::new(
        //                         get_associated_token_address(&user_wallet.pubkey(), &mint.pubkey()),
        //                         false,
        //                     ),
        //                     AccountMeta::new(payer.pubkey(), true),
        //                     AccountMeta::new_readonly(solana_sdk::sysvar::rent::id(), false),
        //                     AccountMeta::new_readonly(system_program::id(), false),
        //                     AccountMeta::new_readonly(token_program_id(), false),
        //                     AccountMeta::new_readonly(spl_associated_token_account::id(), false),
        //                     AccountMeta::new_readonly(spl_associated_token_account::id(), false),
        //                     AccountMeta::new(ownership_pda, false),
        //                     AccountMeta::new(plant_counter_pda, false),
        //                 ],
        //             );
        //
        //             let blockhash = banks_client.get_latest_blockhash().await.unwrap();
        //             let mut tx = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));
        //             tx.sign(&[&payer, &user_wallet, &mint_authority], blockhash);
        //
        //             assert!(
        //                 banks_client.process_transaction(tx).await.is_ok(),
        //                 "Mint failed for {:?}",
        //                 rarity
        //             );
        //         }
        //
        //         let duplicate_data = ProgramInstruction::MintNFT {
        //             card_type: CardRarityInstruction::GenesisFragment,
        //             plant_name: plant_name.to_string(),
        //             is_new_species: false,
        //         }
        //         .try_to_vec()
        //         .unwrap();
        //
        //         let ownership_pda = Pubkey::find_program_address(
        //             &[
        //                 plant_name.as_bytes(),
        //                 user_wallet.pubkey().as_ref(),
        //                 &[CardRarityInstruction::GenesisFragment as u8],
        //             ],
        //             &program_id,
        //         )
        //         .0;
        //
        //         let plant_counter_pda =
        //             Pubkey::find_program_address(&[b"plant_counter", plant_name.as_bytes()], &program_id).0;
        //
        //         let duplicate_ix = Instruction::new_with_bytes(
        //             program_id,
        //             &duplicate_data,
        //             vec![
        //                 AccountMeta::new(user_wallet.pubkey(), true),
        //                 AccountMeta::new(common_mint.pubkey(), false),
        //                 AccountMeta::new(rare_mint.pubkey(), false),
        //                 AccountMeta::new(epic_mint.pubkey(), false),
        //                 AccountMeta::new(metadata_common.pubkey(), false),
        //                 AccountMeta::new(edition_common.pubkey(), false),
        //                 AccountMeta::new(mint_authority.pubkey(), true),
        //                 AccountMeta::new(
        //                     get_associated_token_address(&user_wallet.pubkey(), &common_mint.pubkey()),
        //                     false,
        //                 ),
        //                 AccountMeta::new(payer.pubkey(), true),
        //                 AccountMeta::new_readonly(solana_sdk::sysvar::rent::id(), false),
        //                 AccountMeta::new_readonly(system_program::id(), false),
        //                 AccountMeta::new_readonly(token_program_id(), false),
        //                 AccountMeta::new_readonly(spl_associated_token_account::id(), false),
        //                 AccountMeta::new_readonly(spl_associated_token_account::id(), false),
        //                 AccountMeta::new(ownership_pda, false),
        //                 AccountMeta::new(plant_counter_pda, false),
        //             ],
        //         );
        //
        //         let blockhash = banks_client.get_latest_blockhash().await.unwrap();
        //         let mut tx = Transaction::new_with_payer(&[duplicate_ix], Some(&payer.pubkey()));
        //         tx.sign(&[&payer, &user_wallet, &mint_authority], blockhash);
        //
        //         assert!(
        //             banks_client.process_transaction(tx).await.is_err(),
        //             "Duplicate mint should fail"
        //         );
        //
        //         println!("All NFT minting tests passed!");
    }
}

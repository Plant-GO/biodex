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
    use spl_associated_token_account::{
        get_associated_token_address, instruction::create_associated_token_account,
    };
    use spl_token::id as token_program_id;

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

    async fn setup_mints(
        banks_client: &mut BanksClient,
        payer: &Keypair,
        program_id: Pubkey,
        mint_authority: &Keypair,
    ) -> (Keypair, Keypair, Keypair, Keypair, Keypair) {
        let common_mint = Keypair::new();
        let rare_mint = Keypair::new();
        let epic_mint = Keypair::new();
        let aurora_mint = Keypair::new();
        let primordial_mint = Keypair::new();

        let mint_data = [
            (&common_mint, "Orange", "GenesisFragment"),
            (&rare_mint, "Marigold", "AstralShard"),
            (&epic_mint, "Sunflower", "MythicCrest"),
            (&aurora_mint, "Kaggle", "AuroraSeed"),
            (&primordial_mint, "Rose", "PrimordialRelic"),
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
            tx.sign(&[&payer, mint_authority, mint], blockhash);
            banks_client.process_transaction(tx).await.unwrap();
        }

        (
            common_mint,
            rare_mint,
            epic_mint,
            aurora_mint,
            primordial_mint,
        )
    }

    async fn setup_quiz_mints(
        banks_client: &mut BanksClient,
        payer: &Keypair,
        program_id: Pubkey,
        mint_authority: &Keypair,
    ) -> (Keypair, Keypair) {
        let codex_mint = Keypair::new();
        let ascendent_mint = Keypair::new();

        let mint_data = [
            (&codex_mint, "Orange", "CodexOfInsight"),
            (&ascendent_mint, "Orange", "AstralShard"),
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
            tx.sign(&[&payer, mint_authority, mint], blockhash);
            banks_client.process_transaction(tx).await.unwrap();
        }

        (codex_mint, ascendent_mint)
    }

    async fn mint_card(
        banks_client: &mut BanksClient,
        payer: &Keypair,
        program_id: Pubkey,
        user: &Keypair,
        mint_authority: &Keypair,
        plant_name: &str,
        is_new_species: bool,
        expected_rarity: CardRarityInstruction,
        expected_mint: &Keypair,
        common_mint: &Keypair,
        rare_mint: &Keypair,
        epic_mint: &Keypair,
        aurora_mint: &Keypair,
        primordial_mint: &Keypair,
    ) {
        let ata = get_associated_token_address(&user.pubkey(), &expected_mint.pubkey());
        let ownership_pda = Pubkey::find_program_address(
            &[
                plant_name.as_bytes(),
                user.pubkey().as_ref(),
                &[expected_rarity.clone() as u8],
            ],
            &program_id,
        )
        .0;

        let plant_counter_pda =
            Pubkey::find_program_address(&[b"plant_counter", plant_name.as_bytes()], &program_id).0;

        let ix = Instruction::new_with_bytes(
            program_id,
            &ProgramInstruction::MintNFT {
                card_type: CardRarityInstruction::GenesisFragment,
                plant_name: plant_name.to_string(),
                is_new_species: Some(is_new_species),
                quiz_winner: Some(false),
            }
            .try_to_vec()
            .unwrap(),
            vec![
                AccountMeta::new(user.pubkey(), true),
                AccountMeta::new(common_mint.pubkey(), false),
                AccountMeta::new(rare_mint.pubkey(), false),
                AccountMeta::new(epic_mint.pubkey(), false),
                AccountMeta::new(aurora_mint.pubkey(), false),
                AccountMeta::new(primordial_mint.pubkey(), false),
                AccountMeta::new(mint_authority.pubkey(), true),
                AccountMeta::new(ata, false),
                AccountMeta::new(payer.pubkey(), true),
                AccountMeta::new_readonly(solana_sdk::sysvar::rent::id(), false),
                AccountMeta::new_readonly(system_program::id(), false),
                AccountMeta::new_readonly(token_program_id(), false),
                AccountMeta::new(ownership_pda, false),
                AccountMeta::new(plant_counter_pda, false),
                AccountMeta::new_readonly(spl_associated_token_account::id(), false),
            ],
        );

        let blockhash = banks_client.get_latest_blockhash().await.unwrap();
        let mut tx = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));
        tx.sign(&[&payer, user, mint_authority], blockhash);
        banks_client.process_transaction(tx).await.unwrap();
    }

    async fn mint_quiz_card(
        banks_client: &mut BanksClient,
        payer: &Keypair,
        program_id: Pubkey,
        user: &Keypair,
        mint_authority: &Keypair,
        plant_name: &str,
        quiz: bool,
        expected_rarity: CardRarityInstruction,
        expected_mint: &Keypair,
        codex_mint: &Keypair,
        ascendent_mint: &Keypair,
    ) {
        let ata = get_associated_token_address(&user.pubkey(), &expected_mint.pubkey());
        let ownership_pda = Pubkey::find_program_address(
            &[
                plant_name.as_bytes(),
                user.pubkey().as_ref(),
                &[expected_rarity.clone() as u8],
            ],
            &program_id,
        )
        .0;

        let card_type = if quiz {
            CardRarityInstruction::AscendantSeal
        } else {
            CardRarityInstruction::CodexOfInsight
        };

        let plant_counter_pda =
            Pubkey::find_program_address(&[b"plant_counter", plant_name.as_bytes()], &program_id).0;

        let ix = Instruction::new_with_bytes(
            program_id,
            &ProgramInstruction::MintNFT {
                card_type,
                plant_name: plant_name.to_string(),
                is_new_species: Some(false),
                quiz_winner: Some(quiz),
            }
            .try_to_vec()
            .unwrap(),
            vec![
                AccountMeta::new(user.pubkey(), true),
                AccountMeta::new(codex_mint.pubkey(), false),
                AccountMeta::new(ascendent_mint.pubkey(), false),
                AccountMeta::new(mint_authority.pubkey(), true),
                AccountMeta::new(ata, false),
                AccountMeta::new(payer.pubkey(), true),
                AccountMeta::new_readonly(solana_sdk::sysvar::rent::id(), false),
                AccountMeta::new_readonly(system_program::id(), false),
                AccountMeta::new_readonly(token_program_id(), false),
                AccountMeta::new(ownership_pda, false),
                AccountMeta::new(plant_counter_pda, false),
                AccountMeta::new_readonly(spl_associated_token_account::id(), false),
            ],
        );

        let blockhash = banks_client.get_latest_blockhash().await.unwrap();
        let mut tx = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));
        tx.sign(&[&payer, user, mint_authority], blockhash);
        banks_client.process_transaction(tx).await.unwrap();
    }

    #[tokio::test]
    async fn test_primordial_relic_first_known_plant() {
        println!("\n🧪 TEST: PrimordialRelic - First mint of known plant");

        let program_id = Pubkey::new_unique();
        let program_test = ProgramTest::new("program", program_id, processor!(process_instruction));
        let (mut banks_client, payer, _) = program_test.start().await;

        let mint_authority = create_funded_keypair(&mut banks_client, &payer, 1_000_000_000).await;
        let (common_mint, rare_mint, epic_mint, aurora_mint, primordial_mint) =
            setup_mints(&mut banks_client, &payer, program_id, &mint_authority).await;

        let user = create_funded_keypair(&mut banks_client, &payer, 1_000_000_000).await;

        mint_card(
            &mut banks_client,
            &payer,
            program_id,
            &user,
            &mint_authority,
            "Rose",
            false,
            CardRarityInstruction::PrimordialRelic,
            &primordial_mint,
            &common_mint,
            &rare_mint,
            &epic_mint,
            &aurora_mint,
            &primordial_mint,
        )
        .await;

        println!("✅ PrimordialRelic minted for first known plant mint");
    }

    #[tokio::test]
    async fn test_aurora_seed_new_species() {
        println!("\n🧪 TEST: AuroraSeed - First mint of NEW species");

        let program_id = Pubkey::new_unique();
        let program_test = ProgramTest::new("program", program_id, processor!(process_instruction));
        let (mut banks_client, payer, _) = program_test.start().await;

        let mint_authority = create_funded_keypair(&mut banks_client, &payer, 1_000_000_000).await;
        let (common_mint, rare_mint, epic_mint, aurora_mint, primordial_mint) =
            setup_mints(&mut banks_client, &payer, program_id, &mint_authority).await;

        let user = create_funded_keypair(&mut banks_client, &payer, 1_000_000_000).await;

        mint_card(
            &mut banks_client,
            &payer,
            program_id,
            &user,
            &mint_authority,
            "NewSpecies",
            true,
            CardRarityInstruction::AuroraSeed,
            &aurora_mint,
            &common_mint,
            &rare_mint,
            &epic_mint,
            &aurora_mint,
            &primordial_mint,
        )
        .await;

        println!("✅ AuroraSeed minted for new species discovery");
    }

    #[tokio::test]
    async fn test_mythic_crest_regular_distribution() {
        println!("\n🧪 TEST: MythicCrest - Regular distribution (2nd mint)");

        let program_id = Pubkey::new_unique();
        let program_test = ProgramTest::new("program", program_id, processor!(process_instruction));
        let (mut banks_client, payer, _) = program_test.start().await;

        let mint_authority = create_funded_keypair(&mut banks_client, &payer, 1_000_000_000).await;
        let (common_mint, rare_mint, epic_mint, aurora_mint, primordial_mint) =
            setup_mints(&mut banks_client, &payer, program_id, &mint_authority).await;

        // First user gets PrimordialRelic
        let user1 = create_funded_keypair(&mut banks_client, &payer, 1_000_000_000).await;
        mint_card(
            &mut banks_client,
            &payer,
            program_id,
            &user1,
            &mint_authority,
            "Sunflower",
            false,
            CardRarityInstruction::PrimordialRelic,
            &primordial_mint,
            &common_mint,
            &rare_mint,
            &epic_mint,
            &aurora_mint,
            &primordial_mint,
        )
        .await;
        println!("  → User 1 got PrimordialRelic");

        // Second user should get MythicCrest
        let user2 = create_funded_keypair(&mut banks_client, &payer, 1_000_000_000).await;
        mint_card(
            &mut banks_client,
            &payer,
            program_id,
            &user2,
            &mint_authority,
            "Sunflower",
            false,
            CardRarityInstruction::MythicCrest,
            &epic_mint,
            &common_mint,
            &rare_mint,
            &epic_mint,
            &aurora_mint,
            &primordial_mint,
        )
        .await;

        println!("✅ MythicCrest minted in regular distribution");
    }

    #[tokio::test]
    async fn test_mastery_card() {
        println!("\nTesting Ascendental Seed");
        let program_id = Pubkey::new_unique();
        let program_test = ProgramTest::new("program", program_id, processor!(process_instruction));
        let (mut bank_client, payer, _) = program_test.start().await;

        let mint_authority = create_funded_keypair(&mut bank_client, &payer, 1_000_000_000).await;
        let (codex_mint, ascendent_mint) =
            setup_quiz_mints(&mut bank_client, &payer, program_id, &mint_authority).await;

        let user = create_funded_keypair(&mut bank_client, &payer, 1_000_000_000).await;

        let plant_name = "Orange";

        println!("\nMinting AscendantSeal\n");
        mint_quiz_card(
            &mut bank_client,
            &payer,
            program_id,
            &user,
            &mint_authority,
            plant_name,
            true,
            CardRarityInstruction::AscendantSeal,
            &ascendent_mint,
            &codex_mint,
            &ascendent_mint,
        )
        .await;
    }

    #[tokio::test]
    async fn test_codex_card() {
        println!("\nTesting Ascendental Seed");
        let program_id = Pubkey::new_unique();
        let program_test = ProgramTest::new("program", program_id, processor!(process_instruction));
        let (mut bank_client, payer, _) = program_test.start().await;

        let mint_authority = create_funded_keypair(&mut bank_client, &payer, 1_000_000_000).await;
        let (codex_mint, ascendent_mint) =
            setup_quiz_mints(&mut bank_client, &payer, program_id, &mint_authority).await;

        let user = create_funded_keypair(&mut bank_client, &payer, 1_000_000_000).await;

        let plant_name = "Orange";

        println!("\nMinting CodexOfInsight\n");
        mint_quiz_card(
            &mut bank_client,
            &payer,
            program_id,
            &user,
            &mint_authority,
            plant_name,
            false,
            CardRarityInstruction::CodexOfInsight,
            &codex_mint,
            &codex_mint,
            &ascendent_mint,
        )
        .await;
    }

    #[tokio::test]
    async fn test_astral_shard_after_epic_exhausted() {
        println!("\n🧪 TEST: AstralShard - After MythicCrest slots exhausted");

        let program_id = Pubkey::new_unique();
        let program_test = ProgramTest::new("program", program_id, processor!(process_instruction));
        let (mut banks_client, payer, _) = program_test.start().await;

        let mint_authority = create_funded_keypair(&mut banks_client, &payer, 1_000_000_000).await;
        let (common_mint, rare_mint, epic_mint, aurora_mint, primordial_mint) =
            setup_mints(&mut banks_client, &payer, program_id, &mint_authority).await;

        let plant_name = "Tulip";

        // First user gets PrimordialRelic
        let user1 = create_funded_keypair(&mut banks_client, &payer, 1_000_000_000).await;
        mint_card(
            &mut banks_client,
            &payer,
            program_id,
            &user1,
            &mint_authority,
            plant_name,
            false,
            CardRarityInstruction::PrimordialRelic,
            &primordial_mint,
            &common_mint,
            &rare_mint,
            &epic_mint,
            &aurora_mint,
            &primordial_mint,
        )
        .await;
        println!("  → Mint 1: PrimordialRelic");

        // Next 5 users get MythicCrest (exhausting epic slots)
        for i in 0..5 {
            let user = create_funded_keypair(&mut banks_client, &payer, 1_000_000_000).await;
            mint_card(
                &mut banks_client,
                &payer,
                program_id,
                &user,
                &mint_authority,
                plant_name,
                false,
                CardRarityInstruction::MythicCrest,
                &epic_mint,
                &common_mint,
                &rare_mint,
                &epic_mint,
                &aurora_mint,
                &primordial_mint,
            )
            .await;
            println!("  → Mint {}: MythicCrest ({}/5)", i + 2, i + 1);
        }

        // 7th user should get AstralShard
        let user7 = create_funded_keypair(&mut banks_client, &payer, 1_000_000_000).await;
        mint_card(
            &mut banks_client,
            &payer,
            program_id,
            &user7,
            &mint_authority,
            plant_name,
            false,
            CardRarityInstruction::AstralShard,
            &rare_mint,
            &common_mint,
            &rare_mint,
            &epic_mint,
            &aurora_mint,
            &primordial_mint,
        )
        .await;

        println!("✅ AstralShard minted after MythicCrest exhausted");
    }

    #[tokio::test]
    async fn test_genesis_fragment_after_all_exhausted() {
        println!("\n🧪 TEST: GenesisFragment - After all rare slots exhausted");

        let program_id = Pubkey::new_unique();
        let program_test = ProgramTest::new("program", program_id, processor!(process_instruction));
        let (mut banks_client, payer, _) = program_test.start().await;

        let mint_authority = create_funded_keypair(&mut banks_client, &payer, 1_000_000_000).await;
        let (common_mint, rare_mint, epic_mint, aurora_mint, primordial_mint) =
            setup_mints(&mut banks_client, &payer, program_id, &mint_authority).await;

        let plant_name = "Daisy";

        // First: PrimordialRelic
        let user = create_funded_keypair(&mut banks_client, &payer, 1_000_000_000).await;
        mint_card(
            &mut banks_client,
            &payer,
            program_id,
            &user,
            &mint_authority,
            plant_name,
            false,
            CardRarityInstruction::PrimordialRelic,
            &primordial_mint,
            &common_mint,
            &rare_mint,
            &epic_mint,
            &aurora_mint,
            &primordial_mint,
        )
        .await;
        println!("  → Mint 1: PrimordialRelic");

        // Next 5: MythicCrest
        for i in 0..5 {
            let user = create_funded_keypair(&mut banks_client, &payer, 1_000_000_000).await;
            mint_card(
                &mut banks_client,
                &payer,
                program_id,
                &user,
                &mint_authority,
                plant_name,
                false,
                CardRarityInstruction::MythicCrest,
                &epic_mint,
                &common_mint,
                &rare_mint,
                &epic_mint,
                &aurora_mint,
                &primordial_mint,
            )
            .await;
            println!("  → Mint {}: MythicCrest", i + 2);
        }

        // Next 10: AstralShard
        for i in 0..10 {
            let user = create_funded_keypair(&mut banks_client, &payer, 1_000_000_000).await;
            mint_card(
                &mut banks_client,
                &payer,
                program_id,
                &user,
                &mint_authority,
                plant_name,
                false,
                CardRarityInstruction::AstralShard,
                &rare_mint,
                &common_mint,
                &rare_mint,
                &epic_mint,
                &aurora_mint,
                &primordial_mint,
            )
            .await;
            println!("  → Mint {}: AstralShard", i + 7);
        }

        // 17th mint: GenesisFragment
        let user17 = create_funded_keypair(&mut banks_client, &payer, 1_000_000_000).await;
        mint_card(
            &mut banks_client,
            &payer,
            program_id,
            &user17,
            &mint_authority,
            plant_name,
            false,
            CardRarityInstruction::GenesisFragment,
            &common_mint,
            &common_mint,
            &rare_mint,
            &epic_mint,
            &aurora_mint,
            &primordial_mint,
        )
        .await;

        println!("✅ GenesisFragment minted after all rare slots exhausted");
    }

    #[tokio::test]
    async fn test_duplicate_card_prevention() {
        println!("\n🧪 TEST: Duplicate prevention - Same rarity for same plant");

        let program_id = Pubkey::new_unique();
        let program_test = ProgramTest::new("program", program_id, processor!(process_instruction));
        let (mut banks_client, payer, _) = program_test.start().await;

        let mint_authority = create_funded_keypair(&mut banks_client, &payer, 1_000_000_000).await;
        let (common_mint, rare_mint, epic_mint, aurora_mint, primordial_mint) =
            setup_mints(&mut banks_client, &payer, program_id, &mint_authority).await;

        let user = create_funded_keypair(&mut banks_client, &payer, 1_000_000_000).await;

        // First mint succeeds
        mint_card(
            &mut banks_client,
            &payer,
            program_id,
            &user,
            &mint_authority,
            "Orchid",
            false,
            CardRarityInstruction::PrimordialRelic,
            &primordial_mint,
            &common_mint,
            &rare_mint,
            &epic_mint,
            &aurora_mint,
            &primordial_mint,
        )
        .await;
        println!("  → First mint successful: PrimordialRelic");

        // Second mint with same user should fail
        let ata = get_associated_token_address(&user.pubkey(), &primordial_mint.pubkey());
        let ownership_pda = Pubkey::find_program_address(
            &[
                "Orchid".as_bytes(),
                user.pubkey().as_ref(),
                &[CardRarityInstruction::PrimordialRelic as u8],
            ],
            &program_id,
        )
        .0;
        let plant_counter_pda =
            Pubkey::find_program_address(&[b"plant_counter", "Orchid".as_bytes()], &program_id).0;

        let ix = Instruction::new_with_bytes(
            program_id,
            &ProgramInstruction::MintNFT {
                card_type: CardRarityInstruction::GenesisFragment,
                plant_name: "Orchid".to_string(),
                is_new_species: Some(false),
                quiz_winner: Some(false),
            }
            .try_to_vec()
            .unwrap(),
            vec![
                AccountMeta::new(user.pubkey(), true),
                AccountMeta::new(common_mint.pubkey(), false),
                AccountMeta::new(rare_mint.pubkey(), false),
                AccountMeta::new(epic_mint.pubkey(), false),
                AccountMeta::new(aurora_mint.pubkey(), false),
                AccountMeta::new(primordial_mint.pubkey(), false),
                AccountMeta::new(mint_authority.pubkey(), true),
                AccountMeta::new(ata, false),
                AccountMeta::new(payer.pubkey(), true),
                AccountMeta::new_readonly(solana_sdk::sysvar::rent::id(), false),
                AccountMeta::new_readonly(system_program::id(), false),
                AccountMeta::new_readonly(token_program_id(), false),
                AccountMeta::new(ownership_pda, false),
                AccountMeta::new(plant_counter_pda, false),
                AccountMeta::new_readonly(spl_associated_token_account::id(), false),
            ],
        );

        let blockhash = banks_client.get_latest_blockhash().await.unwrap();
        let mut tx = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));
        tx.sign(&[&payer, &user, &mint_authority], blockhash);

        let result = banks_client.process_transaction(tx).await;
        assert!(result.is_err(), "Duplicate mint should fail");

        println!("✅ Duplicate card correctly prevented");
    }

    #[tokio::test]
    async fn test_user_collects_multiple_rarities_same_plant() {
        println!("\n🧪 TEST: User collects MULTIPLE different rarities for SAME plant");

        let program_id = Pubkey::new_unique();
        let program_test = ProgramTest::new("program", program_id, processor!(process_instruction));
        let (mut banks_client, payer, _) = program_test.start().await;

        let mint_authority = create_funded_keypair(&mut banks_client, &payer, 1_000_000_000).await;
        let (common_mint, rare_mint, epic_mint, aurora_mint, primordial_mint) =
            setup_mints(&mut banks_client, &payer, program_id, &mint_authority).await;

        let plant_name = "Lily";
        let collector = create_funded_keypair(&mut banks_client, &payer, 10_000_000_000).await;

        // Collector gets PrimordialRelic (first mint)
        mint_card(
            &mut banks_client,
            &payer,
            program_id,
            &collector,
            &mint_authority,
            plant_name,
            false,
            CardRarityInstruction::PrimordialRelic,
            &primordial_mint,
            &common_mint,
            &rare_mint,
            &epic_mint,
            &aurora_mint,
            &primordial_mint,
        )
        .await;
        println!("  ✓ Collector has: PrimordialRelic");

        // Fill up 5 MythicCrest slots with other users
        for i in 0..5 {
            let other = create_funded_keypair(&mut banks_client, &payer, 1_000_000_000).await;
            mint_card(
                &mut banks_client,
                &payer,
                program_id,
                &other,
                &mint_authority,
                plant_name,
                false,
                CardRarityInstruction::MythicCrest,
                &epic_mint,
                &common_mint,
                &rare_mint,
                &epic_mint,
                &aurora_mint,
                &primordial_mint,
            )
            .await;
            println!("  → Other user {} got MythicCrest", i + 1);
        }

        // Collector gets AstralShard
        mint_card(
            &mut banks_client,
            &payer,
            program_id,
            &collector,
            &mint_authority,
            plant_name,
            false,
            CardRarityInstruction::AstralShard,
            &rare_mint,
            &common_mint,
            &rare_mint,
            &epic_mint,
            &aurora_mint,
            &primordial_mint,
        )
        .await;
        println!("  ✓ Collector has: PrimordialRelic + AstralShard");

        // Fill up remaining 9 AstralShard slots
        for i in 0..9 {
            let other = create_funded_keypair(&mut banks_client, &payer, 1_000_000_000).await;
            mint_card(
                &mut banks_client,
                &payer,
                program_id,
                &other,
                &mint_authority,
                plant_name,
                false,
                CardRarityInstruction::AstralShard,
                &rare_mint,
                &common_mint,
                &rare_mint,
                &epic_mint,
                &aurora_mint,
                &primordial_mint,
            )
            .await;
            println!("  → Other user {} got AstralShard", i + 1);
        }

        // Collector gets GenesisFragment
        mint_card(
            &mut banks_client,
            &payer,
            program_id,
            &collector,
            &mint_authority,
            plant_name,
            false,
            CardRarityInstruction::GenesisFragment,
            &common_mint,
            &common_mint,
            &rare_mint,
            &epic_mint,
            &aurora_mint,
            &primordial_mint,
        )
        .await;

        println!("  ✓ Collector has: PrimordialRelic + AstralShard + GenesisFragment");
        println!("✅ Same user successfully collected 3 different rarities for same plant!");
    }
}

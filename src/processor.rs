use crate::instruction::{CardRarityInstruction, OwnershipRecord, PlantCounter};
use borsh::BorshDeserialize;
use solana_program::program::invoke_signed;
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
use spl_associated_token_account::instruction as associated_token_account_instruction;
use spl_token::instruction as token_instruction;

const MAX_EPIC: u64 = 5;
const MAX_RARE: u64 = 10;

pub struct Processor {}

impl Processor {
    pub fn process<'a>(
        program_id: &Pubkey,
        accounts: &'a [AccountInfo<'a>],
        instruction: CardRarityInstruction,
        plant_name: &str,
        is_new_species: Option<bool>,
        quiz_winner: Option<bool>,
    ) -> ProgramResult {
        match instruction {
            CardRarityInstruction::GenesisFragment
            | CardRarityInstruction::AstralShard
            | CardRarityInstruction::MythicCrest
            | CardRarityInstruction::PrimordialRelic
            | CardRarityInstruction::AuroraSeed => Self::process_minting(
                program_id,
                accounts,
                instruction,
                plant_name,
                is_new_species,
            ),
            CardRarityInstruction::CodexOfInsight => Self::process_quiz(
                program_id,
                accounts,
                instruction,
                plant_name,
                quiz_winner.unwrap(),
            ),

            CardRarityInstruction::AscendantSeal => Self::process_quiz(
                program_id,
                accounts,
                instruction,
                plant_name,
                quiz_winner.unwrap(),
            ),
        }?;

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

    fn load_or_init_counter(
        plant_counter_account: &AccountInfo,
        plant_name: &str,
    ) -> Result<PlantCounter, ProgramError> {
        if plant_counter_account.data_is_empty() {
            Ok(PlantCounter {
                plant_name: plant_name.to_string(),
                seed_count: 0,
                relic_count: 0,
                epic_count: 0,
                rare_count: 0,
                common_count: 0,
                mastery_count: 0,
                codex_count: 0,
                first_minter: None,
            })
        } else {
            Ok(PlantCounter::try_from_slice(
                &plant_counter_account.data.borrow(),
            )?)
        }
    }

    fn determine_rarity<'a>(
        is_first_on_chain: bool,
        is_new_species: bool,
        counter: &mut PlantCounter,
        user_wallet: &Pubkey,
        common_mint: &'a AccountInfo<'a>,
        rare_mint: &'a AccountInfo<'a>,
        epic_mint: &'a AccountInfo<'a>,
        primordial_mint: &'a AccountInfo<'a>,
        aurora_mint_account: &'a AccountInfo<'a>,
    ) -> (CardRarityInstruction, &'a AccountInfo<'a>) {
        if is_new_species && is_first_on_chain && counter.seed_count == 0 {
            msg!("AuroraSeed AWARDED!");
            msg!("This is a brand new species discovery!");
            counter.first_minter = Some(*user_wallet);
            counter.seed_count += 1;
            (CardRarityInstruction::AuroraSeed, aurora_mint_account)
        } else if !is_new_species && is_first_on_chain && counter.relic_count == 0 {
            msg!("PrimordialRelic AWARDED!");
            msg!("First person to photograph this known plant!");
            counter.relic_count += 1;
            counter.first_minter = Some(*user_wallet);
            (CardRarityInstruction::PrimordialRelic, primordial_mint)
        } else {
            msg!("Regular rarity distribution");

            if counter.epic_count < MAX_EPIC {
                counter.epic_count += 1;
                (CardRarityInstruction::MythicCrest, epic_mint)
            } else if counter.rare_count < MAX_RARE {
                counter.rare_count += 1;
                (CardRarityInstruction::AstralShard, rare_mint)
            } else {
                counter.common_count += 1;
                (CardRarityInstruction::GenesisFragment, common_mint)
            }
        }
    }

    fn ensure_associated_token_account<'a>(
        associated_token_account: &AccountInfo<'a>,
        payer: &AccountInfo<'a>,
        user_wallet: &AccountInfo<'a>,
        mint_account: &AccountInfo<'a>,
        system_program: &AccountInfo<'a>,
        token_program: &AccountInfo<'a>,
        rent: &AccountInfo<'a>,
    ) -> ProgramResult {
        if associated_token_account.lamports() == 0 {
            msg!("Creating associated token account...");

            invoke(
                &associated_token_account_instruction::create_associated_token_account(
                    payer.key,
                    user_wallet.key,
                    mint_account.key,
                    token_program.key,
                ),
                &[
                    payer.clone(),
                    associated_token_account.clone(),
                    user_wallet.clone(),
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

        Ok(())
    }

    fn nft_mint<'a>(
        token_program: &AccountInfo<'a>,
        mint_account: &AccountInfo<'a>,
        associated_token_account: &AccountInfo<'a>,
        mint_authority: &AccountInfo<'a>,
    ) -> ProgramResult {
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

        Ok(())
    }

    fn create_ownership_record<'a>(
        payer: &AccountInfo<'a>,
        ownership_account: &AccountInfo<'a>,
        system_program: &AccountInfo<'a>,
        program_id: &Pubkey,
        user_wallet: &Pubkey,
        plant_name: &str,
        card_type: CardRarityInstruction,
        mint_account: &Pubkey,
        ownership_bump: u8,
    ) -> ProgramResult {
        let ownership_record = borsh::to_vec(&OwnershipRecord::new(
            *user_wallet,
            plant_name.to_string(),
            card_type.clone(),
            *mint_account,
        ))?;
        let ownership_space = ownership_record.len() as u64;
        let rent_lamports = Rent::get()?.minimum_balance(ownership_space as usize) as usize;

        invoke_signed(
            &system_instruction::create_account(
                &payer.key,
                &ownership_account.key,
                Rent::get()?.minimum_balance(rent_lamports),
                ownership_space,
                program_id,
            ),
            &[
                payer.clone(),
                system_program.clone(),
                ownership_account.clone(),
            ],
            &[&[
                plant_name.as_bytes(),
                user_wallet.as_ref(),
                &[card_type.clone() as u8],
                &[ownership_bump],
            ]],
        )?;

        let mut ownership_acc_mut = ownership_account.try_borrow_mut_data()?;
        ownership_acc_mut[..ownership_record.len()].copy_from_slice(&ownership_record);

        Ok(())
    }

    fn save_plant_counter<'a>(
        plant_counter_account: &AccountInfo<'a>,
        counter: &PlantCounter,
        payer: &AccountInfo<'a>,
        system_program: &AccountInfo<'a>,
        program_id: &Pubkey,
        plant_name: &str,
        plant_counter_bump: u8,
    ) -> ProgramResult {
        let serialized_counter = borsh::to_vec(&counter)?;
        if plant_counter_account.data_is_empty() {
            let required_space = serialized_counter.len();
            let rent_lamports = Rent::get()?.minimum_balance(required_space);

            msg!("Inside the first timer");

            invoke_signed(
                &system_instruction::create_account(
                    payer.key,
                    plant_counter_account.key,
                    rent_lamports,
                    required_space as u64,
                    program_id,
                ),
                &[
                    payer.clone(),
                    plant_counter_account.clone(),
                    system_program.clone(),
                ],
                &[&[
                    b"plant_counter",
                    plant_name.as_bytes(),
                    &[plant_counter_bump],
                ]],
            )?;
        }
        msg!("Outside becasue saved");

        let mut data = plant_counter_account.try_borrow_mut_data()?;
        data[..serialized_counter.len()].copy_from_slice(&serialized_counter);

        Ok(())
    }

    fn process_minting<'a>(
        program_id: &Pubkey,
        accounts: &'a [AccountInfo<'a>],
        _card_type: CardRarityInstruction,
        plant_name: &str,
        is_new_species: Option<bool>,
    ) -> ProgramResult {
        let accounts_iter = &mut accounts.iter();

        let user_wallet_account = next_account_info(accounts_iter)?;
        let common_mint_account = next_account_info(accounts_iter)?;
        let rare_mint_account = next_account_info(accounts_iter)?;
        let epic_mint_account = next_account_info(accounts_iter)?;
        let aurora_mint_account = next_account_info(accounts_iter)?;
        let primordial_mint_account = next_account_info(accounts_iter)?;
        let mint_authority = next_account_info(accounts_iter)?;
        let associated_token_account = next_account_info(accounts_iter)?;
        let payer = next_account_info(accounts_iter)?;
        let rent = next_account_info(accounts_iter)?;
        let system_program = next_account_info(accounts_iter)?;
        let token_program = next_account_info(accounts_iter)?;
        let ownership_account = next_account_info(accounts_iter)?;
        let plant_counter_account = next_account_info(accounts_iter)?;

        let (_plant_counter_pda, _plant_counter_bump) =
            Self::process_plant_counter_pda(plant_name, program_id, plant_counter_account)?;

        let is_first_on_chain = plant_counter_account.data_is_empty();

        let mut counter = Self::load_or_init_counter(plant_counter_account, plant_name)?;

        // FIRST: Determine what rarity will actually be minted
        let (final_rarity, mint_account) = Self::determine_rarity(
            is_first_on_chain,
            is_new_species.unwrap(),
            &mut counter,
            user_wallet_account.key,
            common_mint_account,
            rare_mint_account,
            epic_mint_account,
            primordial_mint_account,
            aurora_mint_account,
        );

        msg!("Final rarity: {:?}", final_rarity);

        // THEN: Check ownership using the FINAL rarity, not the input card_type
        let (_ownership_pda, ownership_bump) = Self::process_ownership_account(
            ownership_account,
            program_id,
            plant_name,
            final_rarity.clone(),
            user_wallet_account,
        )
        .unwrap();

        msg!("Minting {:?} card for plant {}", final_rarity, plant_name);

        Self::ensure_associated_token_account(
            associated_token_account,
            payer,
            user_wallet_account,
            mint_account,
            system_program,
            token_program,
            rent,
        )?;

        msg!("Minting NFT to associated token account...");
        Self::nft_mint(
            token_program,
            mint_account,
            associated_token_account,
            mint_authority,
        )?;
        msg!("NFT minted successfully");

        Self::create_ownership_record(
            payer,
            ownership_account,
            system_program,
            program_id,
            user_wallet_account.key,
            plant_name,
            final_rarity.clone(),
            mint_account.key,
            ownership_bump,
        )?;

        Self::save_plant_counter(
            plant_counter_account,
            &counter,
            payer,
            system_program,
            program_id,
            plant_name,
            _plant_counter_bump,
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
        let codex_mint_account = next_account_info(accounts_iter)?;
        let ascendant_account = next_account_info(accounts_iter)?;
        let mint_authority = next_account_info(accounts_iter)?;
        let associated_token_account = next_account_info(accounts_iter)?;
        let payer = next_account_info(accounts_iter)?;
        let rent = next_account_info(accounts_iter)?;
        let system_program = next_account_info(accounts_iter)?;
        let token_program = next_account_info(accounts_iter)?;
        let ownership_account = next_account_info(accounts_iter)?;
        let plant_counter_account = next_account_info(accounts_iter)?;

        let (_, ownership_bump) = Self::process_ownership_account(
            ownership_account,
            program_id,
            plant_name,
            card_type.clone(),
            user_wallet_account,
        )
        .unwrap();

        let (_plant_counter_pda, _plant_counter_bump) =
            Self::process_plant_counter_pda(plant_name, program_id, plant_counter_account).unwrap();

        let mut counter = Self::load_or_init_counter(plant_counter_account, plant_name)?;

        let (final_card, mint_account) = if is_winner {
            msg!("AscendantSeal AWARDED - Quiz Winner!");
            counter.mastery_count += 1;
            (CardRarityInstruction::AscendantSeal, ascendant_account)
        } else {
            msg!("CodexOfInsight AWARDED - Quiz Participation!");
            counter.codex_count += 1;
            (CardRarityInstruction::CodexOfInsight, codex_mint_account)
        };

        Self::ensure_associated_token_account(
            associated_token_account,
            payer,
            user_wallet_account,
            mint_account,
            system_program,
            token_program,
            rent,
        )?;

        msg!("Associated Token Address: {}", associated_token_account.key);

        msg!("Minting NFT to associated token account...");
        Self::nft_mint(
            token_program,
            mint_account,
            associated_token_account,
            mint_authority,
        )?;
        msg!("NFT minted successfully");

        Self::create_ownership_record(
            payer,
            ownership_account,
            system_program,
            program_id,
            user_wallet_account.key,
            plant_name,
            final_card,
            mint_account.key,
            ownership_bump,
        )?;

        Self::process_plant_counter_pda(plant_name, program_id, plant_counter_account)?;

        Ok(())
    }
}

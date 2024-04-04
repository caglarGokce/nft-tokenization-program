
use crate::check::check_mint_and_owner;

use crate::service::create_token_transfer_instruction;
use crate::state::{ InitPDA, NFTState, Terms, TokenBenefit};
use borsh::BorshDeserialize;


use solana_program::system_instruction::{self};
use solana_program::{
  account_info::{next_account_info, AccountInfo},
  entrypoint::ProgramResult,
  pubkey::Pubkey,
  program::{invoke_signed,invoke},
  program_pack::Pack
};


use spl_associated_token_account::instruction::create_associated_token_account;
use spl_token::state::Account;

pub fn get_tokenized_benefit(
    accounts: &[AccountInfo],
    program_id: &Pubkey,
    data:TokenBenefit) -> ProgramResult {

    let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();

    let registered_nft_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let terms_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let benefit_ata: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let benefit_mint: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let benefitor: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let benefitor_tokenization_ata: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let benefitor_benefit_ata: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let benefit_received: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let tokenization_mint: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let token_program_id: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let sysvar: &AccountInfo<'_> = next_account_info(accounts_iter)?;


    let terms: Terms = Terms::try_from_slice(&terms_account.data.borrow())?;

    if terms.is_init != 1 {panic!()}

    if !benefitor.is_signer{panic!()}
    if terms_account.owner != program_id{panic!()}
    if terms_account.is_writable {panic!()}

    let nft_data: NFTState = NFTState::try_from_slice(&registered_nft_account.data.borrow())?;
    let token_mint_from_bytes = Pubkey::new_from_array(nft_data.tokenization_mint);
    if &token_mint_from_bytes != tokenization_mint.key {panic!()}

    check_mint_and_owner(tokenization_mint.key, benefitor.key, benefitor_tokenization_ata);
    check_mint_and_owner(benefit_mint.key, registered_nft_account.key, benefit_ata);

    let result: Result<Account, solana_program::program_error::ProgramError>  = Account::unpack_from_slice(&benefitor_tokenization_ata.data.borrow());

    let ata_unpacked: spl_token::state::Account = match result {
        Ok(account) => account,
        Err(error) => {panic!("{}",error);}};

    if tokenization_mint.key != &ata_unpacked.mint {panic!()}
    if benefitor.key != &ata_unpacked.owner {panic!()}

    let result: Result<Account, solana_program::program_error::ProgramError>  = Account::unpack_from_slice(&benefit_ata.data.borrow());

    let benefit_ata_unpacked: spl_token::state::Account = match result {
        Ok(account) => account,
        Err(error) => {panic!("{}",error);}};

    if benefit_mint.key != &benefit_ata_unpacked.mint {panic!()}
    if registered_nft_account.key != &benefit_ata_unpacked.owner {panic!()}

    let benefit = benefit_ata_unpacked.amount/nft_data.number_of_tokens * ata_unpacked.amount;

    if benefitor_benefit_ata.owner != &spl_token_2022::id() && benefitor_benefit_ata.owner != &spl_token::id() {
      let create_benefitor_benefit_ata: solana_program::instruction::Instruction = create_associated_token_account(
        benefitor.key,
        benefitor.key,
        tokenization_mint.key,
        token_program_id.key);

      invoke(&create_benefitor_benefit_ata,
          &[benefitor.clone(),benefitor_benefit_ata.clone(),tokenization_mint.clone(),token_program_id.clone(),sysvar.clone()])?;
    }

    let ix = create_token_transfer_instruction(
      token_program_id.key,
      benefit_ata.key,
      benefit_mint.key,
      benefitor_benefit_ata.key,
      registered_nft_account.key,
      benefit,
      data.decimals);

    let seed = nft_data.nft_mint;

    invoke_signed(&ix,
       &[benefit_mint.clone(),benefit_ata.clone(),benefitor_benefit_ata.clone(),token_program_id.clone(),registered_nft_account.clone()],
       &[&[&seed, &[data.registered_nft_account_bump]]],
      )?;

    let seed1 = benefit_mint.key.to_bytes();
    let seed2 = benefitor.key.to_bytes();

      invoke_signed(&system_instruction::create_account(
        &benefitor.key,
        &benefit_received.key,
        terms.small_account,
        0,
        program_id),
        &[benefitor.clone(),benefit_received.clone()],
        &[&[b"bnft",&seed1,&seed2, &[data.benefit_received_pda_bump]]],
       )?;

    Ok(())
  }

pub fn get_sol_benefit(
    accounts: &[AccountInfo],
    program_id: &Pubkey,
    data:InitPDA) -> ProgramResult {

    let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();

    let registered_nft_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let terms_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let benefitor: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let benefitor_tokenization_ata: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let benefit_received: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let tokenization_mint: &AccountInfo<'_> = next_account_info(accounts_iter)?;


    let terms: Terms = Terms::try_from_slice(&terms_account.data.borrow())?;

    if terms.is_init != 1 {panic!()}

    if !benefitor.is_signer{panic!()}
    if terms_account.owner != program_id{panic!()}
    if terms_account.is_writable {panic!()}

    let nft_data: NFTState = NFTState::try_from_slice(&registered_nft_account.data.borrow())?;
    let token_mint_from_bytes = Pubkey::new_from_array(nft_data.tokenization_mint);
    if &token_mint_from_bytes != tokenization_mint.key {panic!()}

    check_mint_and_owner(tokenization_mint.key, benefitor.key, benefitor_tokenization_ata);

    let result: Result<Account, solana_program::program_error::ProgramError>  = Account::unpack_from_slice(&benefitor_tokenization_ata.data.borrow());

    let ata_unpacked: spl_token::state::Account = match result {
        Ok(account) => account,
        Err(error) => {panic!("{}",error);}};

    if tokenization_mint.key != &ata_unpacked.mint {panic!()}
    if benefitor.key != &ata_unpacked.owner {panic!()}

    let total_benefit = **registered_nft_account.lamports.borrow() - terms.nft_pda_account;

    let benefit = total_benefit /nft_data.number_of_tokens * ata_unpacked.amount;

    
    **registered_nft_account.lamports.borrow_mut()-= benefit;
    **benefitor.lamports.borrow_mut()+= benefit;


    let seed1 = tokenization_mint.key.to_bytes();
    let seed2 = benefitor.key.to_bytes();

    invoke_signed(&system_instruction::create_account(
      &benefitor.key,
      &benefit_received.key,
      terms.small_account,
      0,
      program_id),
      &[benefitor.clone(),benefit_received.clone()],
      &[&[b"bnft",&seed1,&seed2, &[data.init_pda]]],
     )?;

    Ok(())

  }



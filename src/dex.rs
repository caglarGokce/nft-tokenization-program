
use crate::check::check_mint_and_owner;
use crate::service:: create_token_transfer_instruction;
use crate::state::{ BuyOrder, BuyToken, InitPDA, Lamports, NFTState, SellOrder, SellToken, Terms};
use borsh::{BorshDeserialize, BorshSerialize};

use solana_program::program::invoke;


use solana_program::{
  account_info::{next_account_info, AccountInfo},
  entrypoint::ProgramResult,
  pubkey::Pubkey,
  system_instruction,
  program::invoke_signed,
};
use spl_associated_token_account::instruction::create_associated_token_account;



pub fn make_offer_for_tokens(
    accounts: &[AccountInfo],
    program_id: &Pubkey,
    buytoken:BuyToken) -> ProgramResult {

    let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();

    let buyer: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let buyer_ata: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let tokenization_mint: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let registered_nft_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let buy_order: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let terms_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let token_2022: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let sysvar: &AccountInfo<'_> = next_account_info(accounts_iter)?;

    let terms: Terms = Terms::try_from_slice(&terms_account.data.borrow())?;

    if terms.is_init != 1 {panic!()}

    if !buyer.is_signer{panic!()}
    if terms_account.owner != program_id{panic!()}
    if terms_account.is_writable {panic!()}
    if tokenization_mint.owner!=&spl_token_2022::id(){panic!()}

    if buyer_ata.owner!=&spl_token_2022::id(){
      let create_buyer_ata: solana_program::instruction::Instruction = create_associated_token_account(
        buyer.key,
        buyer.key, 
        tokenization_mint.key, 
        token_2022.key);
  
      invoke(&create_buyer_ata,
          &[buyer.clone(),buyer_ata.clone(),tokenization_mint.clone(),token_2022.clone(),sysvar.clone()])?;
    }else{
      check_mint_and_owner(tokenization_mint.key,buyer.key,buyer_ata);
    }

    if registered_nft_account.owner != program_id {panic!()}
    
    let nft_data: NFTState = NFTState::try_from_slice(&registered_nft_account.data.borrow())?;
    let mint_from_bytes = Pubkey::new_from_array(nft_data.tokenization_mint);
    if &mint_from_bytes != tokenization_mint.key {panic!()}
    

    let total_value = buytoken.price_per_token*buytoken.amount_to_buy;

    invoke(
      &system_instruction::create_account(  
          &buyer.key, 
          &buy_order.key,
          terms.buy_sell_order_account+total_value,
          terms.buy_sell_order_account_size,
          &program_id
      ),
      &[
        buyer.clone(),
        buy_order.clone(), 
      ],

    )?;

    let order = BuyOrder{
        is_init: 13,
        buyer: buyer.key.to_bytes(),
        tokenization_mint: tokenization_mint.key.to_bytes(),
        price_per_token: buytoken.price_per_token,
        amount_to_buy: buytoken.amount_to_buy,
    };

    order.serialize(&mut &mut buy_order.data.borrow_mut()[..])?;

    Ok(())
  }

pub fn sell_tokens_to_offer(
    accounts: &[AccountInfo],
    program_id: &Pubkey,
    amount:Lamports) -> ProgramResult {

    let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();


      let buyer: &AccountInfo<'_> = next_account_info(accounts_iter)?;
      let buyer_ata: &AccountInfo<'_> = next_account_info(accounts_iter)?;
      let seller: &AccountInfo<'_> = next_account_info(accounts_iter)?;
      let seller_ata: &AccountInfo<'_> = next_account_info(accounts_iter)?;
      let tokenization_mint: &AccountInfo<'_> = next_account_info(accounts_iter)?;
      let buy_order: &AccountInfo<'_> = next_account_info(accounts_iter)?;
      let terms_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;
      let token_2022: &AccountInfo<'_> = next_account_info(accounts_iter)?;

      let terms: Terms = Terms::try_from_slice(&terms_account.data.borrow())?;
      let mut order: BuyOrder = BuyOrder::try_from_slice(&buy_order.data.borrow())?;

      let buyer_from_bytes = Pubkey::new_from_array(order.buyer);
      let mint_from_bytes = Pubkey::new_from_array(order.tokenization_mint);
      if &buyer_from_bytes != buyer.key {panic!()}
      if &mint_from_bytes != tokenization_mint.key {panic!()}
  
      if terms.is_init != 1 {panic!()}
      if order.is_init != 13 {panic!()}
  
      if !seller.is_signer{panic!()}
      if terms_account.owner != program_id{panic!()}
      if terms_account.is_writable {panic!()}
      if tokenization_mint.owner!=&spl_token_2022::id(){panic!()}


      check_mint_and_owner(tokenization_mint.key,seller.key,seller_ata);
      check_mint_and_owner(tokenization_mint.key,buyer.key,buyer_ata);

      if order.amount_to_buy < amount.lamports{panic!()}

      let total_value = amount.lamports*order.price_per_token;

      let transfer_token_ix = create_token_transfer_instruction(
        token_2022.key,
        seller_ata.key,
        tokenization_mint.key,
        buyer_ata.key,
        seller.key,
        amount.lamports);
  
      invoke(
         &transfer_token_ix, 
         &[token_2022.clone(),seller_ata.clone(),tokenization_mint.clone(),buyer_ata.clone(),seller.clone()],
         )?;

         order.amount_to_buy-=amount.lamports;

      **buy_order.lamports.borrow_mut()-= total_value;
      **seller.lamports.borrow_mut()+= total_value;

      order.serialize(&mut &mut buy_order.data.borrow_mut()[..])?;


    Ok(())
  }

pub fn cancel_offer(
    accounts: &[AccountInfo],
    program_id: &Pubkey) -> ProgramResult {


      let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();

      let buyer: &AccountInfo<'_> = next_account_info(accounts_iter)?;
      let buy_order: &AccountInfo<'_> = next_account_info(accounts_iter)?;

      if !buyer.is_signer{panic!()}
      if buy_order.owner != program_id{panic!()}

     let order: BuyOrder = BuyOrder::try_from_slice(&buy_order.data.borrow())?;

     let buyer_from_bytes = Pubkey::new_from_array(order.buyer);

     if &buyer_from_bytes != buyer.key {panic!()}
     if order.is_init != 13{panic!()}

     let total_amount = **buy_order.lamports.borrow();

    **buy_order.lamports.borrow_mut()-= total_amount;
    **buyer.lamports.borrow_mut()+= total_amount;

    Ok(())
  }

pub fn list_tokens_for_sale(
    accounts: &[AccountInfo],
    program_id: &Pubkey,
    selltoken:SellToken) -> ProgramResult {
   
  let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();

  let seller: &AccountInfo<'_> = next_account_info(accounts_iter)?;
  let seller_ata: &AccountInfo<'_> = next_account_info(accounts_iter)?;
  let dex: &AccountInfo<'_> = next_account_info(accounts_iter)?;
  let dex_ata: &AccountInfo<'_> = next_account_info(accounts_iter)?;
  let tokenization_mint: &AccountInfo<'_> = next_account_info(accounts_iter)?;
  let sell_order: &AccountInfo<'_> = next_account_info(accounts_iter)?;
  let registered_nft_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;
  let terms_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;
  let token_2022: &AccountInfo<'_> = next_account_info(accounts_iter)?;


  let terms: Terms = Terms::try_from_slice(&terms_account.data.borrow())?;

  if terms.is_init != 1 {panic!()}

  if sell_order.owner != program_id {panic!()}
  if dex.owner != program_id {panic!()}
  if terms_account.owner != program_id {panic!()}
  if terms_account.is_writable{panic!()}

  if seller_ata.owner!=&spl_token_2022::id(){panic!()}
  if dex_ata.owner!=&spl_token_2022::id(){panic!()}

  let uatm_data: NFTState = NFTState::try_from_slice(&registered_nft_account.data.borrow())?;
  let mint_from_bytes = Pubkey::new_from_array(uatm_data.tokenization_mint);
  if &mint_from_bytes != tokenization_mint.key {panic!()}
  
  let  dex_data: InitPDA = InitPDA::try_from_slice(&dex.data.borrow())?;

  if dex_data.init_pda != 5 {panic!()}


  if &mint_from_bytes != tokenization_mint.key {panic!()}


  check_mint_and_owner(tokenization_mint.key, seller.key, seller_ata);
  check_mint_and_owner(tokenization_mint.key, dex.key, dex_ata);

  let transfer_token_ix = create_token_transfer_instruction(
    token_2022.key,
    seller_ata.key,
    tokenization_mint.key,
    dex_ata.key,
    seller.key,
    selltoken.amount_to_sell);

  invoke(&transfer_token_ix, &[token_2022.clone(),seller_ata.clone(),tokenization_mint.clone(),dex_ata.clone(),seller.clone()])?;

  let order = SellOrder{
    is_init: 7,
    seller: seller.key.to_bytes(),
    tokenization_mint: tokenization_mint.key.to_bytes(),
    price_per_token: selltoken.price_per_token,
    amount_to_sell: selltoken.amount_to_sell,
  };

  order.serialize(&mut &mut sell_order.data.borrow_mut()[..])?;


    Ok(())
  }

pub fn buy_tokens(
    accounts: &[AccountInfo],
    program_id: &Pubkey,
    amount:Lamports) -> ProgramResult {


    let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();

    let seller: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let buyer: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let buyer_ata: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let dex: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let dex_ata: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let tokenization_mint: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let sell_order: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let temp: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let terms_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let token_2022: &AccountInfo<'_> = next_account_info(accounts_iter)?;


    let terms: Terms = Terms::try_from_slice(&terms_account.data.borrow())?;

    if terms.is_init != 1 {panic!()}

    if sell_order.owner != program_id {panic!()}
    if dex.owner != program_id {panic!()}
    if terms_account.owner != program_id {panic!()}
    if terms_account.is_writable{panic!()}

    if buyer_ata.owner!=&spl_token_2022::id(){panic!()}
    if dex_ata.owner!=&spl_token_2022::id(){panic!()}


    let mut order: SellOrder = SellOrder::try_from_slice(&sell_order.data.borrow())?;

    if order.is_init != 7 {panic!()}

    let mint_from_bytes = Pubkey::new_from_array(order.tokenization_mint);
    let seller_from_bytes = Pubkey::new_from_array(order.seller);

    if &mint_from_bytes != tokenization_mint.key {panic!()}
    if &seller_from_bytes != seller.key {panic!()}

    check_mint_and_owner(tokenization_mint.key, buyer.key, buyer_ata);
    check_mint_and_owner(tokenization_mint.key, dex.key, dex_ata);
    
    if amount.lamports > order.amount_to_sell {panic!()}

    let total_amount = amount.lamports*order.price_per_token;

    order.amount_to_sell -= amount.lamports;

    let create_temp: &solana_program::instruction::Instruction = &system_instruction::create_account(  
      &buyer.key, 
      &temp.key,
      total_amount,
      0,
      &program_id);

    invoke(create_temp,  &[buyer.clone(),temp.clone()])?;

    let transfer_token_ix = create_token_transfer_instruction(
      token_2022.key,
      dex_ata.key,
      tokenization_mint.key,
      buyer_ata.key,
      dex.key,
      amount.lamports);

    invoke_signed(
       &transfer_token_ix, 
       &[token_2022.clone(),dex_ata.clone(),tokenization_mint.clone(),buyer_ata.clone(),dex.clone()],
       &[&[b"dex", &[255]]],)?;


    **temp.lamports.borrow_mut()-= total_amount;
    **seller.lamports.borrow_mut()+= total_amount;

    order.serialize(&mut &mut sell_order.data.borrow_mut()[..])?;


    Ok(())
  }

pub fn cancel_token_sale(
    accounts: &[AccountInfo],
    program_id: &Pubkey) -> ProgramResult {


      let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();

      let seller: &AccountInfo<'_> = next_account_info(accounts_iter)?;
      let seller_ata: &AccountInfo<'_> = next_account_info(accounts_iter)?;
      let tokenization_mint: &AccountInfo<'_> = next_account_info(accounts_iter)?;
      let sell_order: &AccountInfo<'_> = next_account_info(accounts_iter)?;
      let dex: &AccountInfo<'_> = next_account_info(accounts_iter)?;
      let dex_ata: &AccountInfo<'_> = next_account_info(accounts_iter)?;
      let token_2022: &AccountInfo<'_> = next_account_info(accounts_iter)?;

      if !seller.is_signer{panic!()}
      if sell_order.owner != program_id{panic!()}

     let order: SellOrder = SellOrder::try_from_slice(&sell_order.data.borrow())?;

     if order.is_init != 7{panic!()}

     let seller_from_bytes = Pubkey::new_from_array(order.seller);
     let mint_from_bytes = Pubkey::new_from_array(order.tokenization_mint);

     if &seller_from_bytes != seller.key {panic!()}
     if &mint_from_bytes != tokenization_mint.key {panic!()}

     check_mint_and_owner(tokenization_mint.key, seller.key, seller_ata);
     check_mint_and_owner(tokenization_mint.key, dex.key, dex_ata);

     let transfer_token_ix = create_token_transfer_instruction(
      token_2022.key,
      dex_ata.key,
      tokenization_mint.key,
      seller_ata.key,
      dex.key,
      order.amount_to_sell);

    invoke_signed(
       &transfer_token_ix, 
       &[token_2022.clone(),dex_ata.clone(),tokenization_mint.clone(),seller_ata.clone(),dex.clone()],
       &[&[b"dex", &[255]]],)?;

    let total_amount = **sell_order.lamports.borrow();
      
    **sell_order.lamports.borrow_mut()-= total_amount;
    **seller.lamports.borrow_mut()+= total_amount;


    Ok(())
  }

pub fn init_dex(
    accounts: &[AccountInfo],
    program_id: &Pubkey) -> ProgramResult {

      let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();

      let initializer: &AccountInfo<'_> = next_account_info(accounts_iter)?;
      let dex: &AccountInfo<'_> = next_account_info(accounts_iter)?;

      let create_dex: &solana_program::instruction::Instruction = &system_instruction::create_account(  
        &initializer.key, 
        &dex.key,
        10000000,
        0,
        &program_id);

  
      invoke_signed(
         &create_dex, 
         &[initializer.clone(),dex.clone()],
         &[&[b"dex", &[255]]],)?;

      let dex_data = InitPDA{
        init_pda:5
      };

      dex_data.serialize(&mut &mut dex.data.borrow_mut()[..])?;


    Ok(())
  }

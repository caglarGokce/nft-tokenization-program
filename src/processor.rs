use std::str::FromStr;

use crate::{benefits, dex, fundraise, market, tokenizenft, vote};
use crate::instruction::NFTInstruction;
use crate::state::{ InitPDA,  NFTState, Terms, };
use borsh::{BorshDeserialize, BorshSerialize};


use spl_associated_token_account::instruction::create_associated_token_account;

use solana_program::{
  account_info::{next_account_info, AccountInfo},
  entrypoint::ProgramResult,
  pubkey::Pubkey,

  system_instruction,
  program::{invoke_signed,invoke},

};

//use spl_token_2022::extension::metadata_pointer::instruction::initialize;
//use spl_token_metadata_interface::instruction::initialize;

pub struct Processor;
impl Processor {
  pub fn process(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
  ) -> ProgramResult {
    let instruction: NFTInstruction = NFTInstruction::unpack(instruction_data)?;

    match instruction {

      NFTInstruction::StartFundRaising {data} => {
        fundraise::start_fund_raising_to_buy_nft(accounts,program_id,data)
      }
      NFTInstruction::JoinFundRaising {data} => {
        fundraise::join_fund_raising_to_buy_nft(accounts,program_id, data)
      }
      NFTInstruction::RemoveFunds {data} => {
        fundraise::remove_funds_from_the_fundraising(accounts,program_id,data)
      }
      NFTInstruction::GetTokenizedAsset {} => {
        fundraise::get_tokenized_assets_from_successfull_fundraising(accounts,program_id)
      }
      NFTInstruction::BuyNFTFunds {} => {
        fundraise::buy_nft_with_the_funds_cpi_to(accounts,program_id)
      }
      NFTInstruction::SellNFTtoFund {} => {
        fundraise::sell_nft_to_the_fundraising(accounts,program_id)
      }
      NFTInstruction::BuyNFTFundsProgrm {} => {
        fundraise::buy_nft_listed_in_program_with_the_funds(accounts,program_id)
      }
      NFTInstruction::CreateFunder {} => {
        fundraise::create_funding_account(accounts,program_id)
      }
      NFTInstruction::CreateFundRaisingAcc {data} => {
        fundraise::create_fundraising_account(accounts,program_id,data)
      }

      NFTInstruction::TokenSol {} => {
        tokenizenft::change_tokens_to_sol(accounts,program_id)
      }
      NFTInstruction::TokenizeNFTSell {data} => {
        tokenizenft::tokenize_nft_and_sell_in_this_program(accounts,program_id,data)
      }
      NFTInstruction::BuyTokenizedNFT {data} => {
        tokenizenft::buy_part_of_tokenized_nft_from_this_program(accounts,program_id,data)
      }
      NFTInstruction::StopTokenizedNFTSale {} => {
        tokenizenft::stop_sale_of_tokenized_nft_and_return_tokens(accounts,program_id)
      }
      NFTInstruction::BuyOutNFT {} => {
        tokenizenft::buy_out_tokenized_nft(accounts,program_id)
      }
      NFTInstruction::TokenizeNFT {data} => {
        tokenizenft::tokenize_your_nft(accounts,data,program_id)
      }
      NFTInstruction::InitVoting {data} => {
        vote::init_voting_to_set_new_buy_out_price(accounts,program_id,data)
      }
      NFTInstruction::RepeatVoting {data} => {
        vote::repeat_voting_to_set_new_buy_out_price(accounts,program_id,data)
      }
      NFTInstruction::SetVoteResult {} => {
        vote::set_new_buyout_price_after_voting(accounts,program_id)
      }
      NFTInstruction::Vote {data} => {
        vote::vote(accounts,program_id,data)
      }
      NFTInstruction::InitVoteAccount {data} => {
        vote::init_voter_account(accounts,program_id,data)
      }
      NFTInstruction::LiquidateProg{} => {
        market::sell_nft_owned_by_program_to_investor(accounts,program_id)
      }
      NFTInstruction::LiquidateIndv{} => {
        market::sell_nft_owned_by_individual_to_investor(accounts,program_id)
      }
      NFTInstruction::MakeOffer {data} => {
        market::make_an_offer_for_nft(accounts,program_id,data)
      }
      NFTInstruction::CreateInvestorAccount {data} => {
        market::create_investor_account(accounts,program_id,data)
      }
      NFTInstruction::FundInvestorAccount {data} => {
        market::fund_investor_account(accounts,program_id,data)
      }
      NFTInstruction::CreateMeta {} => {
        Self::create_metadata(accounts,program_id)
      }
      NFTInstruction::Register {data} => {
        Self::register_nft_in_program(accounts,program_id,data)
      }
      NFTInstruction::SellWholeNFT {data} => {
        market::list_nft_forsale_as_whole_in_this_program(accounts,program_id,data)
      }
      NFTInstruction::CancelWholeNFTSale {} => {
        market::cancel_sale_of_nft_as_whole_in_this_program(accounts,program_id)
      }
      NFTInstruction::BuyWholeNFTProgram {} => {
        market::buy_whole_nft_from_this_program(accounts,program_id)
      }
      NFTInstruction::RemoveFundsInvestorAcc {data} => {
        market::remove_funds_from_investor_account(accounts,program_id,data)
      }
      NFTInstruction::UpdateTerms {data} => {
        Self::update_terms(accounts,program_id,data)
      }
      NFTInstruction::MakeOfferForTokens {data} => {
        dex::make_offer_for_tokens(accounts,program_id,data)
      }
      NFTInstruction::SellTokensToOffer {data} => {
        dex::sell_tokens_to_offer(accounts,program_id,data)
      }
      NFTInstruction::CancelOffer  => {
        dex::cancel_offer(accounts,program_id)
      }
      NFTInstruction::ListTokensForSale {data} => {
        dex::list_tokens_for_sale(accounts,program_id,data)
      }
      NFTInstruction::BuyTokens {data} => {
        dex::buy_tokens(accounts,program_id,data)
      }
      NFTInstruction::CancelTokenSale  => {
        dex::cancel_token_sale(accounts,program_id)
      }
      NFTInstruction::InitDex  => {
        dex::init_dex(accounts,program_id)
      }
      NFTInstruction::GetTokenizedBenefit {data} => {
        benefits::get_tokenized_benefit(accounts,program_id,data)
      }
      NFTInstruction::GetSolBenefit {data} => {
        benefits::get_sol_benefit(accounts,program_id,data)
      }
    }
  }

  


  fn create_metadata(
      accounts: &[AccountInfo],
      program_id: &Pubkey,
  ) -> ProgramResult {
  
      let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();
  
      let registered_nft_account: &AccountInfo<'_> = next_account_info(accounts_iter)?; //token mint metadata update authority
      let tokenization_mint: &AccountInfo<'_> = next_account_info(accounts_iter)?;
      let nft_mint: &AccountInfo<'_> = next_account_info(accounts_iter)?;
      //let nft_metadata_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;
      //let token_program_2022: &AccountInfo<'_> = next_account_info(accounts_iter)?;

      if registered_nft_account.owner != program_id {panic!()}

      let pda_account_data: NFTState = NFTState::try_from_slice(&registered_nft_account.data.borrow())?;

  
      let tokenization_mint_from_bytes: Pubkey = Pubkey::new_from_array(pda_account_data.tokenization_mint);
      let nft_mint_key_from_bytes: Pubkey = Pubkey::new_from_array(pda_account_data.nft_mint);

      if tokenization_mint.key != &tokenization_mint_from_bytes{panic!()}
      if nft_mint.key != &nft_mint_key_from_bytes{panic!()}

  
      Ok(())
    }

  fn register_nft_in_program(
    accounts: &[AccountInfo],
    program_id: &Pubkey,
    init:InitPDA
  ) -> ProgramResult {


    let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();

    let initializer: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let pda: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let registered_nft_account_ata: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let nft_mint: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let token_program: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let terms_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let sysvar: &AccountInfo<'_> = next_account_info(accounts_iter)?;

    if terms_account.owner != program_id{panic!()}

    let terms: Terms = Terms::try_from_slice(&terms_account.data.borrow())?;
    if terms.is_init != 1{panic!()}

    let seed: &[u8] = &nft_mint.key.to_bytes();

    //creating nft owner pda in the program
    invoke_signed(
      &system_instruction::create_account(  
          &initializer.key, 
          &pda.key,
          terms.nft_pda_account,
          terms.nft_pda_account_size,
          &program_id
      ),
      &[
        initializer.clone(),
        pda.clone(), 
      ],
      &[&[seed, &[init.init_pda]]],
    )?;

    //creating registered_nft_account_ata
    let create_registered_nft_account_ata: solana_program::instruction::Instruction = create_associated_token_account(
      initializer.key,
      pda.key, 
      nft_mint.key, 
      token_program.key);

    invoke(&create_registered_nft_account_ata,  &[initializer.clone(),pda.clone(),registered_nft_account_ata.clone(),nft_mint.clone(),token_program.clone(),sysvar.clone()])?;

    let nft_terms: NFTState = NFTState{
      
      owner: [0;32],
      nft_mint: nft_mint.key.to_bytes(),
      tokenization_mint: [0;32],
      for_sale:0,
      buy_out_allowed:0,
      owned_by_pda:0,
      tokenized_for_sale:0,
      price: 0, 
      buy_out_price: 0, 
      lamports_per_token_buyout: 0, 
      number_of_tokens: 0, 
      lamports_per_token:0,
      tokens_sold: 0,
      bump:init.init_pda,
      vote_open:0,
     };

     nft_terms.serialize(&mut &mut pda.data.borrow_mut()[..])?;

    Ok(())
  }

  fn update_terms(
    accounts: &[AccountInfo],
    program_id: &Pubkey,
    nft_terms:Terms
  ) -> ProgramResult {



    let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();

    let authority: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let terms: &AccountInfo<'_> = next_account_info(accounts_iter)?;




    let auth: Pubkey = Pubkey::from_str("Frfz5jf4mR7QFNqrYKAMKCjRbCGycX1by6r26UmHHCoL").unwrap();

    if &auth != authority.key {panic!()}

    if terms.owner != program_id{panic!()}

    nft_terms.serialize(&mut &mut terms.data.borrow_mut()[..])?;


    Ok(())
  }


}

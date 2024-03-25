
use crate::state::{ InitPDA, StartVoting,  InitAccount, NFTState,  Terms, VoteAccount, VoteData};
use borsh::{BorshDeserialize, BorshSerialize};

use solana_program::program_pack::Pack;

use solana_program::{
  account_info::{next_account_info, AccountInfo},
  entrypoint::ProgramResult,
  pubkey::Pubkey,
  sysvar::{clock::Clock, Sysvar,},
  system_instruction,
  program::invoke_signed,

};

use spl_token::state::Account;

pub fn init_voting_to_set_new_buy_out_price(
    accounts: &[AccountInfo],
    program_id: &Pubkey,
data:StartVoting) -> ProgramResult {


    let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();

    let proposer: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let proposer_tokenization_ata: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let proposer_pda: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let vote_account_pda: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let registered_nft_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let terms_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;


    if !proposer.is_signer{panic!()}
    if terms_account.owner != program_id{panic!()}

    let mut registered_nft_account_data: NFTState = NFTState::try_from_slice(&registered_nft_account.data.borrow())?;
    let terms: Terms = Terms::try_from_slice(&terms_account.data.borrow())?;

    if terms.is_init != 1{panic!()}

    let tokenization_mint_key_from_bytes: Pubkey = Pubkey::new_from_array(registered_nft_account_data.tokenization_mint);

    if proposer_tokenization_ata.owner!=&spl_token::id() && proposer_tokenization_ata.owner!=&spl_token_2022::id(){panic!()}
    if registered_nft_account_data.for_sale != 0{panic!()}
    if registered_nft_account_data.tokenized_for_sale != 0{panic!()}
    if registered_nft_account_data.vote_open != 0{panic!()}

    let owner: Pubkey = Pubkey::new_from_array(registered_nft_account_data.owner);
    if &owner != registered_nft_account.key {panic!()}

    let proposer_ata_unpacked: Account = Account::unpack_from_slice(&proposer_tokenization_ata.data.borrow())?;
    if tokenization_mint_key_from_bytes != proposer_ata_unpacked.mint {panic!()}
    if &proposer_ata_unpacked.owner != proposer.key{panic!()}

    let seed:&[u8]= &proposer.key.to_bytes();
    let seed2:&[u8]= &registered_nft_account_data.tokenization_mint;

    let derived_pda: Pubkey = Pubkey::create_program_address(&[seed,seed2, &[data.proposer_pda]], program_id)?;
    if &derived_pda != proposer_pda.key{panic!()}


    invoke_signed(
      &system_instruction::create_account(  
          &proposer.key, 
          &vote_account_pda.key,
          terms.vote_account,
          terms.vote_account_size,
          &program_id
      ),
      &[
        proposer.clone(),
        vote_account_pda.clone(), 
      ],
      &[&[b"vote",seed2, &[data.vote_account_pda]]],
    )?;

    let clock: Clock= Clock::get()?;
    let current_time: u64 = clock.unix_timestamp as u64;

    let vote: VoteAccount = VoteAccount{
       tokenization_mint: registered_nft_account_data.tokenization_mint, 
       new_buy_out_price_accept_votes: proposer_ata_unpacked.amount, 
       new_buy_out_price_refuse_votes: 0, 
       voting_ends: current_time + 86400, 
       new_buy_out_offer: data.offer, 
       voting_no:1
    };

    let voter_account: InitPDA = InitPDA{
        init_pda: 1,
    };

    registered_nft_account_data.vote_open = 1;
  
    vote.serialize(&mut &mut vote_account_pda.data.borrow_mut()[..])?;
    voter_account.serialize(&mut &mut proposer_pda.data.borrow_mut()[..])?;
    registered_nft_account_data.serialize(&mut &mut registered_nft_account.data.borrow_mut()[..])?;

    //nft icin yeni bir satis fiyati belirlemek icin oylama baslatir


    Ok(())
  }

pub fn repeat_voting_to_set_new_buy_out_price(
    accounts: &[AccountInfo],
    program_id: &Pubkey,
data:StartVoting) -> ProgramResult {

      let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();

      let proposer: &AccountInfo<'_> = next_account_info(accounts_iter)?;
      let propser_tokenization_ata: &AccountInfo<'_> = next_account_info(accounts_iter)?;
      let proposer_pda: &AccountInfo<'_> = next_account_info(accounts_iter)?;
      let vote_account_pda: &AccountInfo<'_> = next_account_info(accounts_iter)?;
      let registered_nft_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;
  
      
      if !proposer.is_signer{panic!()}
      if vote_account_pda.owner != program_id{panic!()}
      if proposer_pda.owner != program_id{panic!()}
      if registered_nft_account.owner != program_id{panic!()}
      
  
      if propser_tokenization_ata.owner!=&spl_token::id() && propser_tokenization_ata.owner!=&spl_token_2022::id(){panic!()}
      

      let mut votes: VoteAccount = VoteAccount::try_from_slice(&vote_account_pda.data.borrow())?;
      let mut proposer_account: InitPDA = InitPDA::try_from_slice(&proposer_pda.data.borrow())?;
      let mut registered_nft_account_data: NFTState = NFTState::try_from_slice(&registered_nft_account.data.borrow())?;

      let owner: Pubkey = Pubkey::new_from_array(registered_nft_account_data.owner);
      if &owner != registered_nft_account.key {panic!()}
      

      if registered_nft_account_data.for_sale != 0{panic!()}
      if registered_nft_account_data.tokenized_for_sale != 0{panic!()}
      if registered_nft_account_data.vote_open != 0{panic!()}
      

      let tokenization_mint_key_from_bytes: Pubkey = Pubkey::new_from_array(votes.tokenization_mint);
      let tokenization_mint_key_from_bytes_2: Pubkey = Pubkey::new_from_array(registered_nft_account_data.tokenization_mint);
      

      let proposer_ata_unpacked: Account = Account::unpack_from_slice(&propser_tokenization_ata.data.borrow())?;
      if tokenization_mint_key_from_bytes != proposer_ata_unpacked.mint {panic!()}
      if tokenization_mint_key_from_bytes != tokenization_mint_key_from_bytes_2 {panic!()}
      if &proposer_ata_unpacked.owner != proposer.key{panic!()}
      


      let clock: Clock= Clock::get()?;
      let current_time: u64 = clock.unix_timestamp as u64;

      if votes.voting_ends > current_time {panic!()}
      
  
      let seed:&[u8]= &proposer.key.to_bytes();
      let seed2:&[u8]= &votes.tokenization_mint;
  
      
  
      let derived_pda: Pubkey = Pubkey::create_program_address(&[seed,seed2, &[data.proposer_pda]], program_id)?;
      if &derived_pda != proposer_pda.key{panic!()}


      votes.new_buy_out_offer = data.offer;
      votes.voting_no += 1;
      votes.voting_ends = current_time + 86400;
      votes.new_buy_out_price_accept_votes += proposer_ata_unpacked.amount;
      votes.new_buy_out_price_refuse_votes += 0;
      proposer_account.init_pda = votes.voting_no;
      registered_nft_account_data.vote_open =1;
      
    
      votes.serialize(&mut &mut vote_account_pda.data.borrow_mut()[..])?;
      proposer_account.serialize(&mut &mut proposer_pda.data.borrow_mut()[..])?;
      registered_nft_account_data.serialize(&mut &mut registered_nft_account.data.borrow_mut()[..])?;

    //nftnin likide olmasi icin oylama baslatir


    Ok(())
  }

pub fn vote(
    accounts: &[AccountInfo],
    program_id: &Pubkey,
data:VoteData) -> ProgramResult {

      let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();

      let voter: &AccountInfo<'_> = next_account_info(accounts_iter)?;
      let voter_ata: &AccountInfo<'_> = next_account_info(accounts_iter)?;
      let voter_pda: &AccountInfo<'_> = next_account_info(accounts_iter)?;
      let vote_account_pda: &AccountInfo<'_> = next_account_info(accounts_iter)?;
  
  
      if !voter.is_signer{panic!()}
      if vote_account_pda.owner != program_id{panic!()}
      
      if voter_pda.owner != program_id{panic!()}
      
      if voter_ata.owner!=&spl_token::id() && voter_ata.owner!=&spl_token_2022::id(){panic!()}
      

      let mut votes: VoteAccount = VoteAccount::try_from_slice(&vote_account_pda.data.borrow())?;
      let mut voter_account: InitPDA = InitPDA::try_from_slice(&voter_pda.data.borrow())?;

      let tokenization_mint_key_from_bytes: Pubkey = Pubkey::new_from_array(votes.tokenization_mint);

      let voter_ata_unpacked: Account = Account::unpack_from_slice(&voter_ata.data.borrow())?;
      if tokenization_mint_key_from_bytes != voter_ata_unpacked.mint {panic!()}
      if &voter_ata_unpacked.owner != voter.key{panic!()}
      

      if voter_account.init_pda >= votes.voting_no{panic!()}
      


      let clock: Clock= Clock::get()?;
      let current_time: u64 = clock.unix_timestamp as u64;

      if votes.voting_ends < current_time {panic!()}
      
  
      let seed:&[u8]= &voter.key.to_bytes();
      let seed2:&[u8]= &votes.tokenization_mint;
  
  
      let derived_pda: Pubkey = Pubkey::create_program_address(&[seed,seed2, &[data.vote_account_pda_bump]], program_id)?;
      if &derived_pda != voter_pda.key{panic!()}
      

      if data.refuse_accept == 1 {
        votes.new_buy_out_price_refuse_votes += voter_ata_unpacked.amount;

      }else if data.refuse_accept == 2 {
      votes.new_buy_out_price_accept_votes += voter_ata_unpacked.amount;

      }else{panic!()}
      

      voter_account.init_pda = votes.voting_no;
    
      votes.serialize(&mut &mut vote_account_pda.data.borrow_mut()[..])?;
      voter_account.serialize(&mut &mut voter_pda.data.borrow_mut()[..])?;

    
    //likide oylamasinda oy kullanir
 
    Ok(())
  }

pub fn set_new_buyout_price_after_voting(
    accounts: &[AccountInfo],
program_id: &Pubkey) -> ProgramResult {

      let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();


      let vote_account_pda: &AccountInfo<'_> = next_account_info(accounts_iter)?;
      let registered_nft_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;
  
  
      if vote_account_pda.owner != program_id{panic!()}
      if registered_nft_account.owner != program_id{panic!()}
      

      let mut votes: VoteAccount = VoteAccount::try_from_slice(&vote_account_pda.data.borrow())?;
      let mut pda_account_data: NFTState = NFTState::try_from_slice(&registered_nft_account.data.borrow())?;
      let owner: Pubkey = Pubkey::new_from_array(pda_account_data.owner);
      

      if pda_account_data.for_sale != 0{panic!()}
      
      if pda_account_data.tokenized_for_sale != 0{panic!()}
      
      if &owner != registered_nft_account.key {panic!()}
      
      if pda_account_data.vote_open != 1 {panic!()}
      

      let tokenization_mint_key_from_bytes: Pubkey = Pubkey::new_from_array(votes.tokenization_mint);
      let tokenization_mint_key_from_bytes_2: Pubkey = Pubkey::new_from_array(pda_account_data.tokenization_mint);

      
      if tokenization_mint_key_from_bytes != tokenization_mint_key_from_bytes_2 {panic!()}


      let clock: Clock= Clock::get()?;
      let current_time: u64 = clock.unix_timestamp as u64;


      if votes.voting_ends > current_time {panic!()}

      if votes.new_buy_out_price_accept_votes > votes.new_buy_out_price_refuse_votes {
        pda_account_data.buy_out_price = votes.new_buy_out_offer;
      }


      pda_account_data.buy_out_allowed = 1;
      pda_account_data.vote_open = 0;
      votes.new_buy_out_offer = 0;
      votes.new_buy_out_price_accept_votes = 0;
      votes.new_buy_out_price_refuse_votes = 0;
  
    
      votes.serialize(&mut &mut vote_account_pda.data.borrow_mut()[..])?;
      pda_account_data.serialize(&mut &mut registered_nft_account.data.borrow_mut()[..])?;

    //nftnin fiyatini degistirmek icin oylama baslatir


    Ok(())
  }

pub fn init_voter_account(
    accounts: &[AccountInfo],
    program_id: &Pubkey,
data:InitAccount) -> ProgramResult {

      let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();

      let voter: &AccountInfo<'_> = next_account_info(accounts_iter)?;
      let voter_ata: &AccountInfo<'_> = next_account_info(accounts_iter)?;
      let voter_pda: &AccountInfo<'_> = next_account_info(accounts_iter)?;


      if !voter.is_signer{panic!()}
  
      if voter_ata.owner!=&spl_token::id() && voter_ata.owner!=&spl_token_2022::id(){panic!()}

      let voter_ata_unpacked: Account = Account::unpack_from_slice(&voter_ata.data.borrow())?;
      if &voter_ata_unpacked.owner != voter.key{panic!()}

  
      let seed:&[u8]= &voter.key.to_bytes();
      let seed2:&[u8]= &voter_ata_unpacked.mint.to_bytes();
  
      invoke_signed(
        &system_instruction::create_account(  
            &voter.key, 
            &voter_pda.key,
            data.lamports,
            1,
            &program_id
        ),
        &[
          voter.clone(),
          voter_pda.clone(), 
        ],
        &[&[seed,seed2, &[data.bump]]],
      )?;
    
    //likide oylamasinda oy kullanir
 
    Ok(())
  }

use std::str::FromStr;

use crate::instruction::NFTInstruction;
use crate::state::{DistData, FundRaising, FunderAccount, InitPDA, StartVoting, InvestorAccount, Lamports, InitAccount, NFTState, TokenToSol, Proposal, Terms, UserAddresTokenMint, VoteAccount, VoteData};
use borsh::{BorshDeserialize, BorshSerialize};


use solana_program::msg;
use solana_program::program_pack::Pack;
use spl_associated_token_account::instruction::create_associated_token_account;

use solana_program::{
  account_info::{next_account_info, AccountInfo},
  entrypoint::ProgramResult,
  pubkey::Pubkey,
  sysvar::{clock::Clock, Sysvar,},
  system_instruction,
  program::{invoke_signed,invoke},

};


use spl_token::state::Account;
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
        Self::start_fund_raising_to_buy_nft(accounts,program_id,data)
      }
      NFTInstruction::JoinFundRaising {data} => {
        Self::join_fund_raising_to_buy_nft(accounts,program_id, data)
      }
      NFTInstruction::RemoveFunds {data} => {
        Self::remove_funds_from_the_fundraising(accounts,program_id,data)
      }
      NFTInstruction::GetTokenizedAsset {} => {
        Self::get_tokenized_assets_from_successfull_fundraising(accounts,program_id)
      }
      NFTInstruction::BuyNFTFunds {} => {
        Self::buy_nft_with_the_funds_cpi_to(accounts,program_id)
      }
      NFTInstruction::SellNFTtoFund {} => {
        Self::sell_nft_to_the_fundraising(accounts,program_id)
      }
      NFTInstruction::BuyNFTFundsProgrm {} => {
        Self::buy_nft_with_the_funds_from_program(accounts,program_id)
      }
      NFTInstruction::CreateFunder {} => {
        Self::create_funding_account(accounts,program_id)
      }
      NFTInstruction::CreateFundRaisingAcc {data} => {
        Self::create_fundraising_account(accounts,program_id,data)
      }
      NFTInstruction::SellWholeNFT {data} => {
        Self::list_nft_forsale_as_whole_in_this_program(accounts,program_id,data)
      }
      NFTInstruction::CancelWholeNFTSale {} => {
        Self::cancel_sale_of_nft_as_whole_in_this_program(accounts,program_id)
      }
      NFTInstruction::BuyWholeNFTProgram {} => {
        Self::buy_whole_nft_from_this_program(accounts,program_id)
      }
      NFTInstruction::TokenSol {} => {
        Self::change_tokens_to_sol(accounts,program_id)
      }
      NFTInstruction::TokenizeNFTSell {data} => {
        Self::tokenize_nft_and_sell_in_this_program(accounts,program_id,data)
      }
      NFTInstruction::BuyTokenizedNFT {data} => {
        Self::buy_part_of_tokenized_nft_from_this_program(accounts,program_id,data)
      }
      NFTInstruction::StopTokenizedNFTSale {} => {
        Self::stop_sale_of_tokenized_nft_and_return_tokens(accounts,program_id)
      }
      NFTInstruction::BuyOutNFT {} => {
        Self::buy_out_tokenized_nft(accounts,program_id)
      }
      NFTInstruction::TokenizeNFT {data} => {
        Self::tokenize_your_nft(accounts,data,program_id)
      }
      NFTInstruction::InitVoting {data} => {
        Self::init_voting_to_set_new_buy_out_price(accounts,program_id,data)
      }
      NFTInstruction::RepeatVoting {data} => {
        Self::repeat_voting_to_set_new_buy_out_price(accounts,program_id,data)
      }
      NFTInstruction::SetVoteResult {} => {
        Self::set_new_buyout_price_after_voting(accounts,program_id)
      }
      NFTInstruction::Vote {data} => {
        Self::vote(accounts,program_id,data)
      }
      NFTInstruction::InitVoteAccount {data} => {
        Self::init_voter_account(accounts,program_id,data)
      }
      NFTInstruction::LiquidateProg{} => {
        Self::sell_nft_owned_by_program_to_investor(accounts,program_id)
      }
      NFTInstruction::LiquidateIndv{} => {
        Self::sell_nft_owned_by_individual_to_investor(accounts,program_id)
      }
      NFTInstruction::MakeOffer {data} => {
        Self::make_an_offer_for_nft(accounts,program_id,data)
      }
      NFTInstruction::CreateInvestorAccount {data} => {
        Self::create_investor_account(accounts,program_id,data)
      }
      NFTInstruction::FundInvestorAccount {data} => {
        Self::fund_investor_account(accounts,program_id,data)
      }
      NFTInstruction::CreateMeta {} => {
        Self::create_metadata(accounts,program_id)
      }
      NFTInstruction::Register {data} => {
        Self::register_nft_in_program(accounts,program_id,data)
      }
      NFTInstruction::UpdateTerms {data} => {
        Self::update_terms(accounts,program_id,data)
      }
      NFTInstruction::RemoveFundsInvestorAcc {data} => {
        Self::remove_funds_from_investor_account(accounts,program_id,data)
      }
    }
  }


  //NFT satin almak icin fon toplama baslatilir.
  //Ayni NFT icin sadece bir tane fon toplama girisimi olabilir
  //Eger fon toplama girisimi devam ediyorsa ayrica bir girisim baslatilamaz
  //Eger NFT tokenize edilmis ise fon toplama girisimi baslatilamaz. Yatirimcinin mevcut hisse sahiplerinden hisse yani token satin almasi beklenir.
  //Kisacasi sahiplik bir topluluktan otekine gecemez
  //Fon toplama girisimi basarili olursa yatirimci tokenlarini talep eder.
  //yatirimci yatirim ekleyebilir veya yatirimini cekebilir.
  fn start_fund_raising_to_buy_nft(
    accounts: &[AccountInfo],
    program_id: &Pubkey,
    create_account: InitAccount,
  ) -> ProgramResult {

    let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();

    let initializer: &AccountInfo<'_> = next_account_info(accounts_iter)?; //fon toplama girisimini baslatan hesap writable signer
    let tokenization_mint: &AccountInfo<'_> = next_account_info(accounts_iter)?; //NFT'nin tokenize olmus halinin adresi - fungible assets
    let fundrasing_account: &AccountInfo<'_> = next_account_info(accounts_iter)?; //fon toplama girisim hesabi. her NFT icin bir tane bulunur.
    let token_dist_data: &AccountInfo<'_> = next_account_info(accounts_iter)?; //Fon toplama girisimi basarili olursa yatirimci tokenlarini almak icin bu hesabi kullanir
    let registered_nft_account: &AccountInfo<'_> = next_account_info(accounts_iter)?; //NFT ile ilgili butun bilgilerin tutuldugu hesaptir
    let token_2022_program: &AccountInfo<'_> = next_account_info(accounts_iter)?; 
    let terms_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;//Hesaplarin size ve rent datasi burda
    let sysvar: &AccountInfo<'_> = next_account_info(accounts_iter)?;

    if terms_account.owner != program_id{panic!()}
    if fundrasing_account.owner != program_id{panic!()}
    if registered_nft_account.owner != program_id{panic!()}

    let terms: Terms = Terms::try_from_slice(&terms_account.data.borrow())?;

    if terms.is_init != 1 {panic!()}

    if !initializer.is_signer{panic!()}

    let mut fundraising: FundRaising = FundRaising::try_from_slice(&fundrasing_account.data.borrow())?;
    let registered_nft_account_data: NFTState = NFTState::try_from_slice(&registered_nft_account.data.borrow())?;

    if fundraising.fund_raising != 0 {panic!()} //if there is an active fundrasing panic
    if registered_nft_account_data.tokenized_for_sale != 0 {panic!()} //if the nft is tokenized cant start a fundrasing. go buy tokens
    if registered_nft_account_data.owned_by_pda != 0 {panic!()} //if nft already owned by a community cant start a funsraise


    let nft_mint_from_bytes2: Pubkey = Pubkey::new_from_array(registered_nft_account_data.nft_mint);
    let nft_mint_from_bytes3: Pubkey = Pubkey::new_from_array(fundraising.nft_mint);
    if nft_mint_from_bytes3 != nft_mint_from_bytes2 {panic!()}

    let seed: &[u8] = &tokenization_mint.key.to_bytes();


    //initializing token distribution account
    invoke_signed(
        &system_instruction::create_account(
          &initializer.key, 
          &token_dist_data.key,
          terms.token_distribution_account,
          terms.token_distribution_account_size,
          program_id),
        &[
          initializer.clone(),
          token_dist_data.clone(),
        ],
        &[&[seed, &[create_account.bump]]],
     )?;

    //creating mint account
    let ix = &system_instruction::create_account(  
      &initializer.key, 
      &tokenization_mint.key,
      terms.mint,
      terms.mint_size,
      &token_2022_program.key);

    let init_metadata_pointer = spl_token_2022::extension::metadata_pointer::instruction::initialize(
        token_2022_program.key,
        tokenization_mint.key,
        Some(*token_dist_data.key),
        Some(*tokenization_mint.key),
      )?;
    
    //initializing mint for the nft token
    let init_mint = spl_token_2022::instruction::initialize_mint(
      token_2022_program.key,
      tokenization_mint.key,
      token_dist_data.key,
      Some(token_dist_data.key),
      0)?;


    invoke(ix,  &[initializer.clone(),tokenization_mint.clone(),token_2022_program.clone(),])?;
    invoke(&init_metadata_pointer,  &[initializer.clone(),token_dist_data.clone(),tokenization_mint.clone(),token_2022_program.clone(),sysvar.clone()])?;
    invoke(&init_mint,  &[initializer.clone(),token_dist_data.clone(),tokenization_mint.clone(),token_2022_program.clone(),sysvar.clone()])?;



    fundraising.fund_raising=1; // fundraising is set to active
    fundraising.tokens_mint = tokenization_mint.key.to_bytes(); //tokenization mint address of the nft - investor mint these tokens if FR successful
    fundraising.funds_collected = 0; 
    fundraising.number_of_tokens = 0; 
    fundraising.lamports_per_token = terms.lamports_per_token_fundraising;



  let distribution: DistData = DistData{
    token_mint:tokenization_mint.key.to_bytes(),
    distribution_open: 0,
    tokens_left: 0,
    bump: create_account.bump,
  };


  distribution.serialize(&mut &mut token_dist_data.data.borrow_mut()[..])?;
  fundraising.serialize(&mut &mut fundrasing_account.data.borrow_mut()[..])?;


    Ok(())
  }

  fn join_fund_raising_to_buy_nft(
    accounts: &[AccountInfo],
    program_id: &Pubkey,
    data:Lamports
  ) -> ProgramResult {


    let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();

    let funder: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let funders_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let fundraising_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;//nft owner in the program
    let temp: &AccountInfo<'_> = next_account_info(accounts_iter)?;//nft owner in the program

    if !funder.is_signer{panic!()}

    if funders_account.owner != program_id{panic!()}
    if fundraising_account.owner != program_id{panic!()}

    let mut funders_account_data: FunderAccount = FunderAccount::try_from_slice(&funders_account.data.borrow())?;
    let mut fundraising: FundRaising = FundRaising::try_from_slice(&fundraising_account.data.borrow())?;

    if fundraising.fund_raising != 1 {panic!()}

    let funder_address_from_bytes: Pubkey = Pubkey::new_from_array(funders_account_data.funder);
    if &funder_address_from_bytes != funder.key {panic!()}


    let nft_mint_from_bytes: Pubkey = Pubkey::new_from_array(funders_account_data.nft_mint);
    let nft_mint_from_bytes2: Pubkey = Pubkey::new_from_array(fundraising.nft_mint);
    if nft_mint_from_bytes != nft_mint_from_bytes2 {panic!()}


    let tokenization_mint_from_bytes: Pubkey = Pubkey::new_from_array(funders_account_data.tokens_mint);
    let tokenization_mint_from_bytes2: Pubkey = Pubkey::new_from_array(fundraising.tokens_mint);
    if tokenization_mint_from_bytes != tokenization_mint_from_bytes2{panic!()}

    let ix = &system_instruction::create_account(  
      &funder.key, 
      &temp.key,
      data.lamports,
      0,
      &program_id);

    invoke(ix,  &[funder.clone(),temp.clone()])?;


    let number_of_tokens: u64 = data.lamports/fundraising.lamports_per_token;

    let fund_raise: u64 = number_of_tokens*fundraising.lamports_per_token;

    let value: u64 = **temp.lamports.borrow();

    if value != fund_raise {panic!()}

    **temp.lamports.borrow_mut()-= fund_raise;
    **fundraising_account.lamports.borrow_mut()+= fund_raise;

    funders_account_data.fund_invested += fund_raise;
    fundraising.funds_collected += fund_raise;
    fundraising.number_of_tokens += number_of_tokens;

    fundraising.serialize(&mut &mut fundraising_account.data.borrow_mut()[..])?;
    funders_account_data.serialize(&mut &mut funders_account.data.borrow_mut()[..])?;


    Ok(())
  }

  fn remove_funds_from_the_fundraising(
    accounts: &[AccountInfo],
    program_id: &Pubkey,
    data:Lamports
  ) -> ProgramResult {


    let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();

    let funder: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let funder_funds_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let fundraising_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;//nft owner in the program

    if !funder.is_signer{panic!()}

    if funder_funds_account.owner != program_id{panic!()}
    if fundraising_account.owner != program_id{panic!()}

    let mut funder_account: FunderAccount = FunderAccount::try_from_slice(&funder_funds_account.data.borrow())?;
    let mut fundraising: FundRaising = FundRaising::try_from_slice(&fundraising_account.data.borrow())?;

    if fundraising.fund_raising != 1 {panic!()}

    let funder_address_from_bytes: Pubkey = Pubkey::new_from_array(funder_account.funder);
    if &funder_address_from_bytes != funder.key {panic!()}


    let nft_mint_from_bytes: Pubkey = Pubkey::new_from_array(funder_account.nft_mint);
    let nft_mint_from_bytes2: Pubkey = Pubkey::new_from_array(fundraising.nft_mint);
    if nft_mint_from_bytes != nft_mint_from_bytes2 {panic!()}


    let tokenization_mint_from_bytes: Pubkey = Pubkey::new_from_array(funder_account.tokens_mint);
    let tokenization_mint_from_bytes2: Pubkey = Pubkey::new_from_array(fundraising.tokens_mint);
    if tokenization_mint_from_bytes != tokenization_mint_from_bytes2{panic!()}

    let number_of_tokens: u64 = data.lamports/fundraising.lamports_per_token;

    let fund_raise: u64 = number_of_tokens*fundraising.lamports_per_token;

    if data.lamports > funder_account.fund_invested{panic!()}


    **fundraising_account.lamports.borrow_mut()-= fund_raise;
    **funder.lamports.borrow_mut()+= fund_raise;
      
    funder_account.fund_invested -= fund_raise;
    fundraising.funds_collected -= fund_raise;
    fundraising.number_of_tokens -= number_of_tokens;

    fundraising.serialize(&mut &mut fundraising_account.data.borrow_mut()[..])?;
    funder_account.serialize(&mut &mut funder_funds_account.data.borrow_mut()[..])?;


    Ok(())
  }

  fn get_tokenized_assets_from_successfull_fundraising(
    accounts: &[AccountInfo],
    program_id: &Pubkey
  ) -> ProgramResult {

    let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();

    let funder: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let funder_ata: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let funder_funds_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let token_distribution_data: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let tokenization_mint: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let token_2022_program: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let useradresstokenmint: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let terms_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;

    if terms_account.owner != program_id{panic!()}
    if terms_account.is_writable{panic!()}
    let terms: Terms = Terms::try_from_slice(&terms_account.data.borrow())?;
    if terms.is_init != 1 {panic!()}

    if !funder.is_signer{panic!()}

    if funder_funds_account.owner != program_id{panic!()}
    if token_distribution_data.owner != program_id{panic!()}
    
    let funder_ata_unpacked: Account = Account::unpack_from_slice(&funder_ata.data.borrow())?;
    if tokenization_mint.key != &funder_ata_unpacked.mint {panic!();}
    if &funder_ata_unpacked.owner != funder.key{panic!()}

    let funder_account: FunderAccount = FunderAccount::try_from_slice(&funder_funds_account.data.borrow())?;
    let mut distribution: DistData = DistData::try_from_slice(&token_distribution_data.data.borrow())?;

    if distribution.distribution_open != 1{panic!()}

    let funder_address_from_bytes: Pubkey = Pubkey::new_from_array(funder_account.funder);
    if &funder_address_from_bytes != funder.key {panic!()}

    let tokenization_mint_from_bytes: Pubkey = Pubkey::new_from_array(funder_account.tokens_mint);
    if tokenization_mint.key != &tokenization_mint_from_bytes {panic!()}

    let tokens_to_receive: u64 = funder_account.fund_invested/funder_account.lamports_per_token;

    distribution.tokens_left -= tokens_to_receive;
    
    let ix = spl_token_2022::instruction::mint_to_checked(
      token_2022_program.key,
      tokenization_mint.key, 
      funder_ata.key, 
      token_distribution_data.key,
      &[token_distribution_data.key],
      tokens_to_receive,
      0)?;

      let seed: &[u8] = &tokenization_mint.key.to_bytes();

    invoke_signed(
        &ix,
        &[
          funder_ata.clone(),
          tokenization_mint.clone(),
          token_distribution_data.clone(), 
          token_2022_program.clone()
        ],
        &[&[seed, &[distribution.bump]]],
      )?;

      
      let ix2: &solana_program::instruction::Instruction = &system_instruction::create_account(  
        &funder.key, 
        &useradresstokenmint.key,
        terms.usertokenmint_account,
        terms.usertokenmint_account_size,
        &program_id
      );

      invoke(&ix2,  &[funder.clone(),useradresstokenmint.clone(),])?;


      let usertoken = UserAddresTokenMint{
        user:funder.key.to_bytes(),
        mint:tokenization_mint.key.to_bytes()
      };

      let val: u64 =  **funder_funds_account.lamports.borrow();

      **funder_funds_account.lamports.borrow_mut()-= val;
      **funder.lamports.borrow_mut()+= val;


      distribution.serialize(&mut &mut token_distribution_data.data.borrow_mut()[..])?;
      usertoken.serialize(&mut &mut useradresstokenmint.data.borrow_mut()[..])?;


    Ok(())
  }

  fn buy_nft_with_the_funds_cpi_to(
    _accounts: &[AccountInfo],
    _program_id: &Pubkey
  ) -> ProgramResult {


    //let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();

    //Cross program invocation to another market place

    Ok(())
  }

  fn sell_nft_to_the_fundraising(
    accounts: &[AccountInfo],
    program_id: &Pubkey
  ) -> ProgramResult {

    let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();

    let seller: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let seller_ata: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let token_distribution_data: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let fundraising_account: &AccountInfo<'_> = next_account_info(accounts_iter)?; 
    let registered_nft_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let registered_nft_account_ata: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let nft_mint: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let token_program: &AccountInfo<'_> = next_account_info(accounts_iter)?;


    if token_distribution_data.owner != program_id{panic!()}
    if fundraising_account.owner != program_id{panic!()}
    if registered_nft_account.owner != program_id{panic!()}

    let mut fundraising: FundRaising = FundRaising::try_from_slice(&fundraising_account.data.borrow())?;
    let mut pda_account: NFTState = NFTState::try_from_slice(&registered_nft_account.data.borrow())?;
    let mut distribution: DistData = DistData::try_from_slice(&token_distribution_data.data.borrow())?;

    let seed: &[u8] = &fundraising.tokens_mint;

    let derived_dist: Pubkey = Pubkey::create_program_address(&[seed, &[distribution.bump]], program_id)?;

    if &derived_dist != token_distribution_data.key {panic!()}
    if fundraising.fund_raising != 1{panic!()}

 
    let seller_ata_unpacked: Account = Account::unpack_from_slice(&seller_ata.data.borrow())?;
    if nft_mint.key != &seller_ata_unpacked.mint {panic!()}
    if &seller_ata_unpacked.owner != seller.key{panic!()}

    let registered_nft_account_ata_unpacked:Account = Account::unpack_from_slice(&registered_nft_account_ata.data.borrow())?;
    if nft_mint.key != &registered_nft_account_ata_unpacked.mint {panic!()}
    if &registered_nft_account_ata_unpacked.owner != registered_nft_account.key{panic!()}

    let nft_mint_from_bytes: Pubkey = Pubkey::new_from_array(fundraising.nft_mint);
    if &nft_mint_from_bytes != nft_mint.key {panic!()}

    let nft_mint_from_bytes2: Pubkey = Pubkey::new_from_array(pda_account.nft_mint);
    if &nft_mint_from_bytes2 != nft_mint.key {panic!()}


    if token_program.key == &spl_token::id(){

      let  transfer_nft_to_registered_nft_account_ata = spl_token::instruction::transfer_checked( &spl_token::id(),
      &seller_ata.key, 
      &nft_mint.key, 
      &registered_nft_account_ata.key, 
      &seller.key, 
      &[],1,0)?;

      invoke(&transfer_nft_to_registered_nft_account_ata,&[token_program.clone(),seller_ata.clone(),registered_nft_account_ata.clone(),seller.clone()])?; 

    }else if token_program.key == &spl_token_2022::id(){

      let  transfer_nft_to_registered_nft_account_ata_2022 = spl_token_2022::instruction::transfer_checked( &token_program.key,
        &seller_ata.key, 
        &nft_mint.key, 
        &registered_nft_account_ata.key, 
        &seller.key, 
        &[],1,0)?;

       invoke(&transfer_nft_to_registered_nft_account_ata_2022,&[token_program.clone(),seller_ata.clone(),nft_mint.clone(),registered_nft_account_ata.clone(),seller.clone()])?; 

    }else{ panic!()}


    pda_account.owner = registered_nft_account.key.to_bytes();
    pda_account.owned_by_pda = 1; //nft sahipligi topluluga(programa) gecer
    pda_account.buy_out_allowed = 0; // topluluk oylama yaparak bir satis fiyati belirleyebilir
    pda_account.for_sale = 0; //satisa cikmasi icin topluluk oylamasi gerekir.
    pda_account.tokenized_for_sale = 0; //tokenize olmus bir nft tekrar tokenize edilip satilamaz
    pda_account.tokenization_mint = fundraising.tokens_mint; //fon toplama girisimine katilanlarin sahip olacagi token minti

    fundraising.fund_raising = 0; //fon toplama girismi bitti

    distribution.tokens_left = fundraising.number_of_tokens; //bolunmus hisse sayisi. 
    distribution.distribution_open = 1; //yatirimcilar tokenlarini mint edebilirler. fon toplama basarili

    if fundraising.funds_collected < pda_account.price{panic!()} //fon talep edilen fiyattan az ise geri cevrilir


    **fundraising_account.lamports.borrow_mut()-= fundraising.funds_collected;
    **seller.lamports.borrow_mut()+= fundraising.funds_collected;


    fundraising.serialize(&mut &mut fundraising_account.data.borrow_mut()[..])?;
    pda_account.serialize(&mut &mut registered_nft_account.data.borrow_mut()[..])?;
    distribution.serialize(&mut &mut token_distribution_data.data.borrow_mut()[..])?;


    Ok(())
  }

  fn buy_nft_with_the_funds_from_program(
    accounts: &[AccountInfo],
    program_id: &Pubkey
  ) -> ProgramResult {

    let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();

    let seller: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let token_distribution_data: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let fundraising_account: &AccountInfo<'_> = next_account_info(accounts_iter)?; 
    let pda: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let registered_nft_account_ata: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let nft_mint: &AccountInfo<'_> = next_account_info(accounts_iter)?;


    if token_distribution_data.owner != program_id{panic!()}
    if fundraising_account.owner != program_id{panic!()}
    if pda.owner != program_id{panic!()}

    let mut fundraising: FundRaising = FundRaising::try_from_slice(&fundraising_account.data.borrow())?;
    let mut pda_account: NFTState = NFTState::try_from_slice(&pda.data.borrow())?;
    let mut distribution: DistData = DistData::try_from_slice(&token_distribution_data.data.borrow())?;
    
    
    if pda_account.for_sale != 1{panic!()}


    let seed: &[u8] = &fundraising.tokens_mint;

    let derived_dist: Pubkey = Pubkey::create_program_address(&[seed, &[distribution.bump]], program_id)?;

    if &derived_dist != token_distribution_data.key {panic!()}
    if fundraising.fund_raising != 1{panic!()}

    

    let registered_nft_account_ata_unpacked:Account = Account::unpack_from_slice(&registered_nft_account_ata.data.borrow())?;
    if nft_mint.key != &registered_nft_account_ata_unpacked.mint {panic!()}
    if &registered_nft_account_ata_unpacked.owner != pda.key{panic!()}

    let nft_mint_from_bytes: Pubkey = Pubkey::new_from_array(fundraising.nft_mint);
    if &nft_mint_from_bytes != nft_mint.key {panic!()}
    

    let nft_mint_from_bytes2: Pubkey = Pubkey::new_from_array(pda_account.nft_mint);
    if &nft_mint_from_bytes2 != nft_mint.key {panic!()}
    

    let owner_from_bytes: Pubkey = Pubkey::new_from_array(pda_account.owner);
    if &owner_from_bytes != seller.key {panic!()}
    if &owner_from_bytes == pda.key{panic!()}
    


    pda_account.owner = pda.key.to_bytes();
    pda_account.owned_by_pda = 1; //nft sahipligi topluluga(programa) gecer
    pda_account.buy_out_allowed = 0; // topluluk oylama yaparak bir satis fiyati belirleyebilir
    pda_account.for_sale = 0; //satisa cikmasi icin topluluk oylamasi gerekir.
    pda_account.tokenized_for_sale = 0; //tokenize olmus bir nft tekrar tokenize edilip satilamaz
    pda_account.tokenization_mint = fundraising.tokens_mint; //fon toplama girisimine katilanlarin sahip olacagi token minti

    fundraising.fund_raising = 0; //fon toplama girismi bitti

    distribution.tokens_left = fundraising.number_of_tokens; //bolunmus hisse sayisi. 
    distribution.distribution_open = 1; //yatirimcilar tokenlarini mint edebilirler. fon toplama basarili

    if fundraising.funds_collected < pda_account.price{panic!()} //fon talep edilen fiyattan az ise geri cevrilir


    **fundraising_account.lamports.borrow_mut()-= fundraising.funds_collected;
    **seller.lamports.borrow_mut()+= fundraising.funds_collected;
    


    fundraising.serialize(&mut &mut fundraising_account.data.borrow_mut()[..])?;
    pda_account.serialize(&mut &mut pda.data.borrow_mut()[..])?;
    distribution.serialize(&mut &mut token_distribution_data.data.borrow_mut()[..])?;


    Ok(())
  }

  fn create_funding_account(
    accounts: &[AccountInfo],
    program_id: &Pubkey
  ) -> ProgramResult {

    //yatirimci fon girisimine yatirim yapmak icin hesap olusturur. 
    //farkli zamanlarda olmak uzere her nft icin birden fazla satin alma girisimi olabilir ancak
    //her satin alma girisiminin tokenizasyon minti kendine ozgudur. boylece
    //her girisim birbirinden ayri tutulmus ve yatirimlar birbirine karismamis olur

    let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();

    let funder: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let funder_ata: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let funder_funds_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let tokenization_mint: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let fundraising_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let token_program: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let terms_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;

    if !funder.is_signer{panic!()}

    if fundraising_account.owner != program_id{panic!()}
    if terms_account.owner != program_id{panic!()}

    let fundraising: FundRaising = FundRaising::try_from_slice(&fundraising_account.data.borrow())?;
    let terms: Terms = Terms::try_from_slice(&terms_account.data.borrow())?;

    if fundraising.fund_raising != 1{panic!()}
    if terms.is_init != 1{panic!()}

    let tokenization_mint_from_bytes: Pubkey = Pubkey::new_from_array(fundraising.tokens_mint);
    if &tokenization_mint_from_bytes != tokenization_mint.key {panic!()}

    invoke(
      &system_instruction::create_account(  
          &funder.key, 
          &funder_funds_account.key,
          terms.funder_account,
          terms.funder_account_size,
          &program_id
      ),
      &[
        funder.clone(),
        funder_funds_account.clone(), 
      ],
    )?;

    let funds: FunderAccount = FunderAccount{
      funder:funder.key.to_bytes(),
      nft_mint:fundraising.nft_mint,
      tokens_mint:tokenization_mint.key.to_bytes(),
      fund_invested:0,
      lamports_per_token:fundraising.lamports_per_token
    };

    let create_funder_ata: solana_program::instruction::Instruction = create_associated_token_account(
      funder.key,
      funder.key, 
      tokenization_mint.key, 
      token_program.key);

    invoke(&create_funder_ata,  &[funder.clone(),tokenization_mint.clone(),funder_ata.clone(),token_program.clone()])?;

    funds.serialize(&mut &mut funder_funds_account.data.borrow_mut()[..])?;

    Ok(())
  }

  fn create_fundraising_account(
    accounts: &[AccountInfo],
    program_id: &Pubkey,
    data:InitPDA
  ) -> ProgramResult {

    //fundrasing account fonlarin toplandigi hesaptir. her nft icin sadece bir fon toplama hesabi vardir.
    //fonlama basarili olursa yatirimcilar tokenizaston minti adresi uzerinden tokenlarini talep ederler

    let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();

    let initializer: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let fundraising_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let nft_mint: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let terms_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;

    if terms_account.owner != program_id{panic!()}
    if terms_account.is_writable{panic!()}

    let terms: Terms = Terms::try_from_slice(&terms_account.data.borrow())?;
    if terms.is_init != 1 {panic!()}
    if nft_mint.owner != &spl_token::id() && nft_mint.owner != &spl_token_2022::id(){panic!()}

    let seed: &[u8] = &nft_mint.key.to_bytes();

    invoke_signed(
        &system_instruction::create_account(
          &initializer.key, 
          &fundraising_account.key,
          terms.fundrasing_account,
            terms.fundrasing_account_size, 
            program_id),
        &[
          initializer.clone(),
          fundraising_account.clone(),

        ],
        &[&[b"fund",seed, &[data.init_pda]]],
     )?;

    let funds: FundRaising = FundRaising{ 
      fund_raising:0,
      nft_mint: nft_mint.key.to_bytes(),
      tokens_mint:[0;32],
      funds_collected: 0,
      number_of_tokens: 0,
      lamports_per_token: terms.lamports_per_token_fundraising,
      bump:data.init_pda
      };

    funds.serialize(&mut &mut fundraising_account.data.borrow_mut()[..])?;

    Ok(())
  }


//TODO fiyat belirleme
  fn list_nft_forsale_as_whole_in_this_program(
    accounts: &[AccountInfo],
    program_id: &Pubkey,
    data: NFTState,
    ) -> ProgramResult {

      //nft saticisi pazar yerimizde nftsini satisa cikarir.
      //pesin odeme talep eder. Birisi pesin odeme ile yada bir fon toplama girisimi alabilir 

    let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();

    let seller: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let seller_ata: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let pda: &AccountInfo<'_> = next_account_info(accounts_iter)?;//nft owner in the program
    let registered_nft_account_ata: &AccountInfo<'_> = next_account_info(accounts_iter)?;//nft ata of the pda
    let nft_mint: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let token_program: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let terms_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;

    if terms_account.owner != program_id{panic!()}
    if terms_account.is_writable{panic!()}
    let terms: Terms = Terms::try_from_slice(&terms_account.data.borrow())?;
    if terms.is_init != 1 {panic!()}

    if seller_ata.owner!= &spl_token::id() && seller_ata.owner != &spl_token_2022::id(){panic!()}
    if nft_mint.owner!= &spl_token::id() && nft_mint.owner !=  &spl_token_2022::id(){panic!()}

    if pda.owner != program_id{panic!()}
    let seller_ata_unpacked :Account = Account::unpack_from_slice(&seller_ata.data.borrow())?;
    if nft_mint.key != &seller_ata_unpacked.mint {panic!()}
    if &seller_ata_unpacked.owner != seller.key{panic!()}

    let mut pda_account_data: NFTState = NFTState::try_from_slice(&pda.data.borrow())?;

    if pda_account_data.tokenized_for_sale != 0{panic!()}
    if pda_account_data.for_sale != 0{panic!()}

    let nft_from_bytes: Pubkey = Pubkey::new_from_array(pda_account_data.nft_mint);

    if &nft_from_bytes != nft_mint.key {panic!()}


    //transfering nft to program
    if token_program.key == &spl_token::id(){

      let transfer_nft_to_registered_nft_account_ata = spl_token::instruction::transfer_checked( &token_program.key,
            &seller_ata.key, 
            &nft_mint.key, 
            &registered_nft_account_ata.key, 
            &seller.key, 
            &[],1,0)?;

      invoke(&transfer_nft_to_registered_nft_account_ata,&[token_program.clone(),seller_ata.clone(),registered_nft_account_ata.clone(),seller.clone()])?; 
          

    }else if token_program.key == &spl_token_2022::id(){

      let transfer_nft_to_registered_nft_account_ata = spl_token_2022::instruction::transfer_checked( &token_program.key,
            &seller_ata.key, 
            &nft_mint.key, 
            &registered_nft_account_ata.key, 
            &seller.key, 
            &[],1,0)?;

      invoke(&transfer_nft_to_registered_nft_account_ata,&[token_program.clone(),seller_ata.clone(),registered_nft_account_ata.clone(),seller.clone(),nft_mint.clone()])?; 

    }else{panic!()}

    let lamports_per_token: u64 = terms.lamports_per_token_fundraising;

    let number_of_tokens = data.price/lamports_per_token;

    if data.price < lamports_per_token{panic!()}

    let price: u64 = number_of_tokens*lamports_per_token;


    pda_account_data.owner= seller.key.to_bytes();
    pda_account_data.for_sale=1;
    pda_account_data.price= price;
    pda_account_data.buy_out_price= data.buy_out_price;
    pda_account_data.lamports_per_token_buyout= data.lamports_per_token_buyout;
    pda_account_data.number_of_tokens= number_of_tokens;
    pda_account_data.lamports_per_token = lamports_per_token;


    pda_account_data.serialize(&mut &mut pda.data.borrow_mut()[..])?;

    Ok(())
  }

  fn cancel_sale_of_nft_as_whole_in_this_program(
    accounts: &[AccountInfo],
    program_id: &Pubkey,
    ) -> ProgramResult {

    let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();

    let seller: &AccountInfo<'_> = next_account_info(accounts_iter)?; //check
    let seller_ata: &AccountInfo<'_> = next_account_info(accounts_iter)?; //transfer nft back to here
    let registered_nft_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;//transfer nft back
    let registered_nft_account_ata: &AccountInfo<'_> = next_account_info(accounts_iter)?;//nft ata of the pda
    let nft_mint: &AccountInfo<'_> = next_account_info(accounts_iter)?;//check
    let token_program: &AccountInfo<'_> = next_account_info(accounts_iter)?;


    if registered_nft_account.owner != program_id {panic!()}

    if !seller.is_signer{panic!()}
    if seller_ata.owner!=&spl_token::id() && seller_ata.owner!= &spl_token_2022::id(){panic!()}
    if nft_mint.owner!=&spl_token::id() && nft_mint.owner!= &spl_token_2022::id(){panic!()}

    let seller_ata_unpacked: Account = Account::unpack_from_slice(&seller_ata.data.borrow())?;
    if nft_mint.key != &seller_ata_unpacked.mint {panic!()}
    if &seller_ata_unpacked.owner != seller.key{panic!()}

    let registered_nft_account_ata_unpacked: Account = Account::unpack_from_slice(&registered_nft_account_ata.data.borrow())?;
    if nft_mint.key != &registered_nft_account_ata_unpacked.mint {panic!()}
    if &registered_nft_account_ata_unpacked.owner != registered_nft_account.key{panic!()}

    let mut pda_account_data: NFTState = NFTState::try_from_slice(&registered_nft_account.data.borrow())?;

    if pda_account_data.for_sale != 1{panic!()}
    if pda_account_data.tokenized_for_sale != 0{panic!()}

    let seller_key_from_bytes: Pubkey = Pubkey::new_from_array(pda_account_data.owner);
    let nft_mint_key_from_bytes: Pubkey = Pubkey::new_from_array(pda_account_data.nft_mint);

    if &seller_key_from_bytes != seller.key{panic!()}
    if &nft_mint_key_from_bytes != nft_mint.key{panic!()}



    let transfer_nft_to_seller: solana_program::instruction::Instruction;

    if token_program.key == &spl_token::id(){
      transfer_nft_to_seller=spl_token::instruction::transfer_checked( &token_program.key,
        &registered_nft_account_ata.key, 
        &nft_mint.key, 
        &seller_ata.key, 
        &registered_nft_account.key, 
        &[],1,0)?;
    }else if token_program.key == &spl_token_2022::id(){
      transfer_nft_to_seller=spl_token_2022::instruction::transfer_checked( &token_program.key,
        &registered_nft_account_ata.key, 
        &nft_mint.key, 
        &seller_ata.key, 
        &registered_nft_account.key, 
        &[],1,0)?;
    }else{panic!()}

  let seed: &[u8] = &nft_mint.key.to_bytes();

    //transfer nft back to seller
    invoke_signed(
      &transfer_nft_to_seller,
      &[
        seller_ata.clone(),
        registered_nft_account_ata.clone(),
        registered_nft_account.clone(), 
        token_program.clone(),
        nft_mint.clone()
      ],
      &[&[seed, &[pda_account_data.bump]]],
    )?;


    pda_account_data.tokenization_mint = [0;32];
    pda_account_data.for_sale = 0;
    pda_account_data.number_of_tokens = 0;
    pda_account_data.tokens_sold = 0;
    pda_account_data.price = 0;
    pda_account_data.buy_out_price = 0;
    pda_account_data.lamports_per_token = 0;

    pda_account_data.serialize(&mut &mut registered_nft_account.data.borrow_mut()[..])?;


    //nft saticisi sell_nft_as_whole_in_this_program fonksiyonu ile baslattigi satisi iptal eder.
    //yatirimcilar varlikalrini geri talep eder

    Ok(())
  }

  fn buy_whole_nft_from_this_program(
    accounts: &[AccountInfo],
    program_id: &Pubkey
  ) -> ProgramResult {


    let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();

    let seller: &AccountInfo<'_> = next_account_info(accounts_iter)?; //check
    let buyer: &AccountInfo<'_> = next_account_info(accounts_iter)?; 
    let temp: &AccountInfo<'_> = next_account_info(accounts_iter)?; 
    let buyer_ata: &AccountInfo<'_> = next_account_info(accounts_iter)?; 
    let registered_nft_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;//transfer nft back
    let registered_nft_account_ata: &AccountInfo<'_> = next_account_info(accounts_iter)?;//nft ata of the pda
    let nft_mint: &AccountInfo<'_> = next_account_info(accounts_iter)?;//check
    let token_program: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let sysvar: &AccountInfo<'_> = next_account_info(accounts_iter)?;

    
    if registered_nft_account.owner != program_id {panic!()}
    if !buyer.is_signer {panic!()}

    if buyer_ata.owner!=&spl_token::id() && buyer_ata.owner!=&spl_token_2022::id(){

      let create_buyer_ata: solana_program::instruction::Instruction = create_associated_token_account(
        buyer.key,
        buyer.key, 
        nft_mint.key, 
        token_program.key);
  
      invoke(&create_buyer_ata,
          &[buyer.clone(),buyer_ata.clone(),nft_mint.clone(),token_program.clone(),sysvar.clone()])?;
  
    }
    if nft_mint.owner!=&spl_token::id() && nft_mint.owner!=&spl_token_2022::id(){panic!()}
    

    let buyer_ata_unpacked: Account = Account::unpack_from_slice(&buyer_ata.data.borrow())?;
    if nft_mint.key != &buyer_ata_unpacked.mint {panic!()}
    if &buyer_ata_unpacked.owner != buyer.key{panic!()}
    

    let registered_nft_account_ata_unpacked: Account = Account::unpack_from_slice(&registered_nft_account_ata.data.borrow())?;
    if nft_mint.key != &registered_nft_account_ata_unpacked.mint {panic!()}
    if &registered_nft_account_ata_unpacked.owner != registered_nft_account.key{panic!()}
    

    let mut pda_account_data: NFTState = NFTState::try_from_slice(&registered_nft_account.data.borrow())?;

    if pda_account_data.for_sale != 1{panic!()}
    if pda_account_data.tokenized_for_sale != 0{panic!()}
    

    let seller_key_from_bytes: Pubkey = Pubkey::new_from_array(pda_account_data.owner);
    let nft_mint_key_from_bytes: Pubkey = Pubkey::new_from_array(pda_account_data.nft_mint);

    if &seller_key_from_bytes != seller.key{panic!()}
    if &nft_mint_key_from_bytes != nft_mint.key{panic!()}
    

    let ix = &system_instruction::create_account(  
      &buyer.key, 
      &temp.key,
      pda_account_data.price,
      0,
      &program_id);

    invoke(ix,  &[buyer.clone(),temp.clone()])?;
    

    **temp.lamports.borrow_mut()-= pda_account_data.price;
    **seller.lamports.borrow_mut()+= pda_account_data.price;

    let seed: &[u8] = &nft_mint.key.to_bytes();

    let transfer_nft_to_buyer: solana_program::instruction::Instruction;

    if token_program.key == &spl_token::id(){
      transfer_nft_to_buyer =spl_token::instruction::transfer_checked( &token_program.key,
        &registered_nft_account_ata.key, 
        &nft_mint.key, 
        &buyer_ata.key, 
        &registered_nft_account.key, &[],
        1,0)?;
    }else if token_program.key == &spl_token_2022::id(){
      transfer_nft_to_buyer =spl_token_2022::instruction::transfer_checked( &token_program.key,
        &registered_nft_account_ata.key, 
        &nft_mint.key, 
        &buyer_ata.key, 
        &registered_nft_account.key, &[],
        1,0)?;
    }else{panic!()}
    

    invoke_signed(
        &transfer_nft_to_buyer,
        &[
          buyer_ata.clone(),
          registered_nft_account_ata.clone(),
          registered_nft_account.clone(), 
          token_program.clone(),
          nft_mint.clone()
        ],
        &[&[seed, &[pda_account_data.bump]]],
     )?;

    pda_account_data.owner = [0;32];
    pda_account_data.tokenization_mint = [0;32];
    pda_account_data.for_sale = 0;
    pda_account_data.number_of_tokens = 0;
    pda_account_data.tokens_sold = 0;
    pda_account_data.price = 0;
    pda_account_data.buy_out_price = 0;
    pda_account_data.lamports_per_token = 0;

    

    pda_account_data.serialize(&mut &mut registered_nft_account.data.borrow_mut()[..])?;
  
    //fonksiyonu cagiran bu program da satilan bir nft'nin hepsini satin alir ve cuzdaninda sahip olur.
    //oteki yatirimcilar varliklarini geri iade alir.

    Ok(())
  }

  //tokenization accounttan iade aliyorlar
  fn change_tokens_to_sol(
    accounts: &[AccountInfo],
    program_id: &Pubkey
  ) -> ProgramResult {

    let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();

    let buyer: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let buyer_ata: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let tokenized_nft_token_mint: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let tokenized_nft_token_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let token_program: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let sysvar: &AccountInfo<'_> = next_account_info(accounts_iter)?;

    if tokenized_nft_token_account.owner != program_id {panic!()}
    if !buyer.is_signer {panic!()}

    if buyer_ata.owner!=&spl_token_2022::id(){
      let create_buyer_ata: solana_program::instruction::Instruction = create_associated_token_account(
        buyer.key,
        buyer.key, 
        tokenized_nft_token_mint.key, 
        token_program.key);
  
      invoke(&create_buyer_ata,
          &[buyer.clone(),buyer_ata.clone(),tokenized_nft_token_mint.clone(),token_program.clone(),sysvar.clone()])?;
    }
    if tokenized_nft_token_mint.owner!=&spl_token::id() && tokenized_nft_token_mint.owner!=&spl_token_2022::id(){panic!()}

    let buyer_ata_unpacked: Account = Account::unpack_from_slice(&buyer_ata.data.borrow())?;
    if tokenized_nft_token_mint.key != &buyer_ata_unpacked.mint {panic!()}
    if &buyer_ata_unpacked.owner != buyer.key{panic!()}

    let mut tokenized_nft_data: TokenToSol = TokenToSol::try_from_slice(&tokenized_nft_token_account.data.borrow())?;

    let tokenization_mint_key_from_bytes: Pubkey = Pubkey::new_from_array(tokenized_nft_data.tokenization_mint);

    if &tokenization_mint_key_from_bytes != tokenized_nft_token_mint.key{panic!()}

    if buyer_ata_unpacked.amount == 0 {panic!()}

    if tokenized_nft_data.tokens_sold == 0{panic!()}

    let total_value: u64 = tokenized_nft_data.lamports_per_token*buyer_ata_unpacked.amount;

    tokenized_nft_data.tokens_sold -= buyer_ata_unpacked.amount;


    let ix = spl_token_2022::instruction::burn_checked(
      token_program.key,
      buyer_ata.key, 
      tokenized_nft_token_mint.key, 
       buyer.key,
       &[buyer.key],
       buyer_ata_unpacked.amount,
      0)?;

    let ix2 = spl_token_2022::instruction::close_account(
          token_program.key,
          buyer_ata.key, 
          buyer.key, 
           buyer.key,
           &[buyer.key],
      )?;


    invoke(&ix, &[token_program.clone(),buyer_ata.clone(),tokenized_nft_token_mint.clone(),buyer.clone()])?;
    invoke(&ix2, &[token_program.clone(),buyer_ata.clone(),buyer.clone()])?;

    **tokenized_nft_token_account.lamports.borrow_mut()-= total_value;
    **buyer.lamports.borrow_mut()+= total_value;

    tokenized_nft_data.serialize(&mut &mut tokenized_nft_token_account.data.borrow_mut()[..])?;


    //sell_nft_as_whole_in_this_program ile acilan hesaba yatirim gonderir.
    //cuzdanina fungible asset gonderilir

    Ok(())
  }


  fn tokenize_nft_and_sell_in_this_program(
  accounts: &[AccountInfo],
  program_id: &Pubkey,
  data:NFTState
) -> ProgramResult {


  let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();

  let seller: &AccountInfo<'_> = next_account_info(accounts_iter)?;
  let seller_ata: &AccountInfo<'_> = next_account_info(accounts_iter)?;
  let seller_tokenization_ata: &AccountInfo<'_> = next_account_info(accounts_iter)?;
  let pda: &AccountInfo<'_> = next_account_info(accounts_iter)?;
  let registered_nft_account_ata: &AccountInfo<'_> = next_account_info(accounts_iter)?;
  let nft_mint: &AccountInfo<'_> = next_account_info(accounts_iter)?;
  let tokenization_mint: &AccountInfo<'_> = next_account_info(accounts_iter)?;
  let token_program: &AccountInfo<'_> = next_account_info(accounts_iter)?;
  let token_program_2022: &AccountInfo<'_> = next_account_info(accounts_iter)?;
  let sysvar: &AccountInfo<'_> = next_account_info(accounts_iter)?;
  let terms_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;

  
  if terms_account.owner != program_id{panic!()}
  if terms_account.is_writable{panic!()}
  let terms: Terms = Terms::try_from_slice(&terms_account.data.borrow())?;
  if terms.is_init != 1 {panic!()}
  

  if pda.owner != program_id {panic!()}
  if seller_ata.owner!=&spl_token::id() && seller_ata.owner!=&spl_token_2022::id(){panic!()}
  if nft_mint.owner!=&spl_token::id() && nft_mint.owner!=&spl_token_2022::id(){panic!()}
  
  

  let seller_ata_unpacked :Account = Account::unpack_from_slice(&seller_ata.data.borrow())?;
  if nft_mint.key != &seller_ata_unpacked.mint {panic!()}
  if &seller_ata_unpacked.owner != seller.key{panic!()}
  

  let mut pda_account_data: NFTState = NFTState::try_from_slice(&pda.data.borrow())?;

  if pda_account_data.for_sale != 0{panic!()}
  if pda_account_data.tokenized_for_sale != 0{panic!()}
  

  let nft_from_bytes: Pubkey = Pubkey::new_from_array(pda_account_data.nft_mint);

  if &nft_from_bytes != nft_mint.key {panic!()}
  

  //creating mint account
  let ix = &system_instruction::create_account(  
    &seller.key, 
    &tokenization_mint.key,
    terms.mint,
    terms.mint_size,
    &token_program_2022.key
  );

  let init_metadata_pointer = spl_token_2022::extension::metadata_pointer::instruction::initialize(
    token_program_2022.key,
    tokenization_mint.key,
    Some(*pda.key),
    Some(*tokenization_mint.key),
  )?;

  //initializing mint for the nft token
  let ix_2 = spl_token_2022::instruction::initialize_mint(
   token_program_2022.key,
   tokenization_mint.key,
   pda.key,
   Some(pda.key),
   0)?;

  let create_seller_tokenization_ata: solana_program::instruction::Instruction = create_associated_token_account(
    seller.key,
    seller.key, 
    tokenization_mint.key, 
    token_program_2022.key);


  invoke(ix,  &[seller.clone(),tokenization_mint.clone(),token_program_2022.clone(),])?;
  invoke(&init_metadata_pointer,  &[pda.clone(),tokenization_mint.clone(),token_program_2022.clone(),sysvar.clone()])?;
  invoke(&ix_2,  &[pda.clone(),tokenization_mint.clone(),token_program_2022.clone(),sysvar.clone()])?;
  invoke(&create_seller_tokenization_ata, &[seller.clone(),seller_tokenization_ata.clone(),tokenization_mint.clone(),token_program_2022.clone(),sysvar.clone()])?;

  //transfering nft to program
  if token_program.key == &spl_token::id(){
          let transfer_nft_to_registered_nft_account_ata = spl_token::instruction::transfer_checked( &token_program.key,
            &seller_ata.key, 
            &nft_mint.key, 
            &registered_nft_account_ata.key, 
            &seller.key, 
            &[],1,0)?;
         invoke(&transfer_nft_to_registered_nft_account_ata,&[token_program.clone(),seller_ata.clone(),nft_mint.clone(),registered_nft_account_ata.clone(),seller.clone()])?; 
  }else if token_program.key == &spl_token_2022::id(){
          let transfer_nft_to_registered_nft_account_ata = spl_token_2022::instruction::transfer_checked( &token_program.key,
            &seller_ata.key, 
            &nft_mint.key, 
            &registered_nft_account_ata.key, 
            &seller.key, 
            &[],1,0)?;
          invoke(&transfer_nft_to_registered_nft_account_ata,&[token_program.clone(),nft_mint.clone(),seller_ata.clone(),registered_nft_account_ata.clone(),seller.clone()])?; 
  }else{panic!()}
  

  if data.buy_out_price < data.price{panic!()}
  
  if data.lamports_per_token < terms.minimum_lamports_per_token{panic!()}
  
  if data.lamports_per_token % terms.minimum_lamports_per_token != 0{panic!()}
  
  if data.lamports_per_token_buyout < terms.minimum_lamports_per_token{panic!()}
  
  if data.lamports_per_token_buyout % terms.minimum_lamports_per_token != 0{panic!()}


  let price: u64 = data.number_of_tokens*data.lamports_per_token;

  let buy_out_price = data.number_of_tokens * data.lamports_per_token_buyout;

  pda_account_data.owner= seller.key.to_bytes();
  pda_account_data.nft_mint= nft_mint.key.to_bytes();
  pda_account_data.tokenization_mint= tokenization_mint.key.to_bytes();
  pda_account_data.for_sale=0;
  pda_account_data.buy_out_allowed=1;
  pda_account_data.owned_by_pda=1;
  pda_account_data.tokenized_for_sale=1;
  pda_account_data.price= price;
  pda_account_data.buy_out_price= buy_out_price;
  pda_account_data.lamports_per_token_buyout= data.lamports_per_token_buyout;
  pda_account_data.number_of_tokens= data.number_of_tokens;
  pda_account_data.lamports_per_token = data.lamports_per_token;
  pda_account_data.tokens_sold= 0;

  pda_account_data.serialize(&mut &mut pda.data.borrow_mut()[..])?;

  //nft saticisi nftsini bu programda hisselendirip satisa cikarir.
  //nft hisseleri satildikca nft saticisinin cuzdanina parcalar halinde varlik gelir
  //alici ise cuzdanina token yani hisse alir

  Ok(())
}

  fn buy_part_of_tokenized_nft_from_this_program(
    accounts: &[AccountInfo],
    program_id: &Pubkey,
    data:Lamports
  ) -> ProgramResult {


    let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();

    let seller: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let buyer: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let temp: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let buyer_tokenization_ata: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let registered_nft_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;//nft owner in the program
    let nft_mint: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let tokenization_mint: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let token_program_2022: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let sysvar: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let useradresstokenmint: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let terms_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;

    if terms_account.owner != program_id{panic!()}
    if terms_account.is_writable{panic!()}
    let terms: Terms = Terms::try_from_slice(&terms_account.data.borrow())?;
    if terms.is_init != 1 {panic!()}

    if registered_nft_account.owner != program_id {panic!()}
    if !buyer.is_signer {panic!()}

    if buyer_tokenization_ata.owner!=&spl_token::id() && buyer_tokenization_ata.owner!=&spl_token_2022::id(){
      let create_buyer_ata: solana_program::instruction::Instruction = create_associated_token_account(
        buyer.key,
        buyer.key, 
        tokenization_mint.key, 
        token_program_2022.key);

      invoke(&create_buyer_ata,
          &[buyer.clone(),buyer_tokenization_ata.clone(),tokenization_mint.clone(),token_program_2022.clone(),sysvar.clone()])?;
    }
    if tokenization_mint.owner!=&spl_token::id() && tokenization_mint.owner!=&spl_token_2022::id(){panic!()}

    let buyer_ata_unpacked: Account = Account::unpack_from_slice(&buyer_tokenization_ata.data.borrow())?;
    if tokenization_mint.key != &buyer_ata_unpacked.mint {panic!()}
    if &buyer_ata_unpacked.owner != buyer.key{panic!()}

    let mut pda_account_data: NFTState = NFTState::try_from_slice(&registered_nft_account.data.borrow())?;

    if pda_account_data.for_sale != 0{panic!()}
    if pda_account_data.tokenized_for_sale != 1{panic!()}

    let seller_key_from_bytes: Pubkey = Pubkey::new_from_array(pda_account_data.owner);
    let tokenization_mint_key_from_bytes: Pubkey = Pubkey::new_from_array(pda_account_data.tokenization_mint);
    let nft_mint_key_from_bytes: Pubkey = Pubkey::new_from_array(pda_account_data.nft_mint);

    if &seller_key_from_bytes != seller.key{panic!()}
    if &tokenization_mint_key_from_bytes != tokenization_mint.key{panic!()}
    if &nft_mint_key_from_bytes != nft_mint.key{panic!()}


    let value: u64 = **buyer.lamports.borrow();
    let total_to_pay: u64 = pda_account_data.lamports_per_token*data.lamports/*number of tokens to buy*/;
    if value <  total_to_pay {panic!()}

    let tokens_left: u64 = pda_account_data.number_of_tokens - pda_account_data.tokens_sold;
    if tokens_left < data.lamports/*number of tokens to buy*/ {panic!()}

    let create_temp: &solana_program::instruction::Instruction = &system_instruction::create_account(  
      &buyer.key, 
      &temp.key,
      total_to_pay,
      0,
      &program_id);

    invoke(create_temp,  &[buyer.clone(),temp.clone()])?;


    let seed: &[u8] = &nft_mint.key.to_bytes();

    let ix = spl_token_2022::instruction::mint_to_checked(
      token_program_2022.key,
      tokenization_mint.key, 
      buyer_tokenization_ata.key, 
       registered_nft_account.key,//ata_owner???
       &[registered_nft_account.key],
       data.lamports,0)?;

      invoke_signed(
        &ix,
        &[
          buyer_tokenization_ata.clone(),
          tokenization_mint.clone(),
          registered_nft_account.clone(), 
          token_program_2022.clone()
        ],
        &[&[seed, &[pda_account_data.bump]]],
      )?;

      let ix2: &solana_program::instruction::Instruction = &system_instruction::create_account(  
        &buyer.key, 
        &useradresstokenmint.key,
        terms.usertokenmint_account,
        terms.usertokenmint_account_size,
        &program_id
      );

      invoke(&ix2,  &[buyer.clone(),useradresstokenmint.clone(),])?;


    pda_account_data.tokens_sold += data.lamports;

    //if all required amount is collected transfer ownership to pda
    if pda_account_data.number_of_tokens == pda_account_data.tokens_sold{
      pda_account_data.owner = registered_nft_account.key.to_bytes();
      pda_account_data.tokenized_for_sale = 0;

    }

    let usertoken = UserAddresTokenMint{
      user:buyer.key.to_bytes(),
      mint:tokenization_mint.key.to_bytes()
    };


    **temp.lamports.borrow_mut()-= total_to_pay;
    **seller.lamports.borrow_mut()+= total_to_pay;

    pda_account_data.serialize(&mut &mut registered_nft_account.data.borrow_mut()[..])?;
    usertoken.serialize(&mut &mut useradresstokenmint.data.borrow_mut()[..])?;


    //fonksiyonu cagiran tokenize_nft_and_sell_in_this_program fonksiyonu ile tokenize edilen nftden parcalar alir

    Ok(())
  }

  fn stop_sale_of_tokenized_nft_and_return_tokens(
    accounts: &[AccountInfo],
    program_id: &Pubkey
  ) -> ProgramResult {


    let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();

    let owner: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let owner_tokenization_ata: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let nft_mint: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let tokenization_mint: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let registered_nft_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;//nft owner in the program
    let token_program: &AccountInfo<'_> = next_account_info(accounts_iter)?;


    if registered_nft_account.owner != program_id {panic!()}
    if !owner.is_signer {panic!()}

    if owner_tokenization_ata.owner!=&spl_token::id() && owner_tokenization_ata.owner!=&spl_token_2022::id(){panic!()}
    if tokenization_mint.owner!=&spl_token::id() && tokenization_mint.owner!=&spl_token_2022::id(){panic!()}



    let owner_tokenized_nft_token_ata_unpacked: Account = Account::unpack_from_slice(&owner_tokenization_ata.data.borrow())?;
    if tokenization_mint.key != &owner_tokenized_nft_token_ata_unpacked.mint {panic!()}
    if &owner_tokenized_nft_token_ata_unpacked.owner != owner.key{panic!()}

    let mut pda_account_data: NFTState = NFTState::try_from_slice(&registered_nft_account.data.borrow())?;

    if pda_account_data.for_sale != 0{panic!()}
    if pda_account_data.tokenized_for_sale != 1{panic!()}

    let owner_key_from_bytes: Pubkey = Pubkey::new_from_array(pda_account_data.owner);
    let tokenization_mint_key_from_bytes: Pubkey = Pubkey::new_from_array(pda_account_data.tokenization_mint);
    let nft_mint_key_from_bytes: Pubkey = Pubkey::new_from_array(pda_account_data.nft_mint);
    if &nft_mint_key_from_bytes != nft_mint.key {panic!()}

    if &owner_key_from_bytes != owner.key {panic!()}
    if &tokenization_mint_key_from_bytes != tokenization_mint.key {panic!()}

    let tokens_left: u64 = pda_account_data.number_of_tokens - pda_account_data.tokens_sold;


    let transfer_tokens_to_owner = spl_token_2022::instruction::mint_to_checked( &token_program.key,
        &tokenization_mint.key, 
        &owner_tokenization_ata.key, 
        &registered_nft_account.key, 
        &[&registered_nft_account.key], 
        tokens_left,0)?;

    let seed: &[u8] = &nft_mint.key.to_bytes();
    

    invoke_signed(&transfer_tokens_to_owner, &[
      token_program.clone(),
      owner_tokenization_ata.clone(),
      tokenization_mint.clone(),
      registered_nft_account.clone(),
    ],
    &[&[seed, &[pda_account_data.bump]]],)?;
    pda_account_data.owner = registered_nft_account.key.to_bytes();


    pda_account_data.tokenized_for_sale = 0;


    pda_account_data.serialize(&mut &mut registered_nft_account.data.borrow_mut()[..])?;
   
    //nft saticisi tokenize_nft_and_sell_in_this_program fonksiyonu ile satisa cikardigi nftsinin satisini iptal eder
    //bedel token olarak cuzdanina gelir, nft tokenize olarak programda kalir

    Ok(())
  }

  fn buy_out_tokenized_nft(
    accounts: &[AccountInfo],
    program_id: &Pubkey
  ) -> ProgramResult {


    let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();

    let owner: &AccountInfo<'_> = next_account_info(accounts_iter)?; //check
    let buyer: &AccountInfo<'_> = next_account_info(accounts_iter)?; 
    let temp: &AccountInfo<'_> = next_account_info(accounts_iter)?; 
    let buyer_ata: &AccountInfo<'_> = next_account_info(accounts_iter)?; 
    let registered_nft_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;//transfer nft back
    let registered_nft_account_ata: &AccountInfo<'_> = next_account_info(accounts_iter)?;//nft ata of the pda
    let nft_mint: &AccountInfo<'_> = next_account_info(accounts_iter)?;//check
    let token_program: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let tokenized_nft_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;    
    let terms_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let sysvar: &AccountInfo<'_> = next_account_info(accounts_iter)?;

    if terms_account.owner != program_id{panic!()}

    let terms: Terms = Terms::try_from_slice(&terms_account.data.borrow())?;
    if terms.is_init != 1{panic!()}

    if registered_nft_account.owner != program_id {panic!()}
    if !buyer.is_signer {panic!()}

    if buyer_ata.owner!=&spl_token::id() && buyer_ata.owner!=&spl_token_2022::id(){

      let create_buyer_ata: solana_program::instruction::Instruction = create_associated_token_account(
        buyer.key,
        buyer.key, 
        nft_mint.key, 
        token_program.key);
  
      invoke(&create_buyer_ata,
          &[buyer.clone(),buyer_ata.clone(),nft_mint.clone(),token_program.clone(),sysvar.clone()])?;
    }

    if nft_mint.owner!=&spl_token::id() && nft_mint.owner!=&spl_token_2022::id(){panic!()}

    let buyer_ata_unpacked: Account = Account::unpack_from_slice(&buyer_ata.data.borrow())?;
    if nft_mint.key != &buyer_ata_unpacked.mint {panic!()}
    if &buyer_ata_unpacked.owner != buyer.key{panic!()}

    let registered_nft_account_ata_unpacked: Account = Account::unpack_from_slice(&registered_nft_account_ata.data.borrow())?;
    if nft_mint.key != &registered_nft_account_ata_unpacked.mint {panic!()}
    if &registered_nft_account_ata_unpacked.owner != registered_nft_account.key{panic!()}

    let mut pda_account_data: NFTState = NFTState::try_from_slice(&registered_nft_account.data.borrow())?;

    if pda_account_data.buy_out_allowed != 1 {panic!()}

    let owner_key_from_bytes: Pubkey = Pubkey::new_from_array(pda_account_data.owner);
    let nft_mint_key_from_bytes: Pubkey = Pubkey::new_from_array(pda_account_data.nft_mint);

    if &owner_key_from_bytes != owner.key{panic!()}
    if &nft_mint_key_from_bytes != nft_mint.key{panic!()}


    let create_temp: &solana_program::instruction::Instruction = &system_instruction::create_account(  
      &buyer.key, 
      &temp.key,
      pda_account_data.buy_out_price,
      0,
      &program_id);

    invoke(create_temp,  &[buyer.clone(),temp.clone()])?;


    let transfer_nft_to_buyer: solana_program::instruction::Instruction;
    if token_program.key == &spl_token::id(){
        transfer_nft_to_buyer =spl_token::instruction::transfer_checked( &token_program.key,
          &registered_nft_account_ata.key, 
          &nft_mint.key, 
          &buyer_ata.key, 
          &registered_nft_account.key, &[],
          1,0)?;
    }else if token_program.key == &spl_token_2022::id(){
        transfer_nft_to_buyer =spl_token_2022::instruction::transfer_checked( &token_program.key,
          &registered_nft_account_ata.key, 
          &nft_mint.key, 
          &buyer_ata.key, 
          &registered_nft_account.key, &[],
          1,0)?;
    }else{panic!()}

    let seed: &[u8] = &nft_mint.key.to_bytes();
    invoke_signed(
        &transfer_nft_to_buyer,
        &[
        buyer.clone(),
        buyer_ata.clone(),
        nft_mint.clone(),
        registered_nft_account_ata.clone(),
        registered_nft_account.clone(), 
        token_program.clone()
      ],
        &[&[seed, &[pda_account_data.bump]]],
    )?;

    let ix: &solana_program::instruction::Instruction = &system_instruction::create_account(  
      &buyer.key, 
      &tokenized_nft_account.key,
      terms.token_to_sol_account,
      terms.token_to_sol_account_size,
      &program_id
    );
    invoke(&ix,  &[buyer.clone(),token_program.clone(),tokenized_nft_account.clone(),])?;

    let lamports_per_token = pda_account_data.buy_out_price/pda_account_data.number_of_tokens;


    let tokenization_account_data: TokenToSol = TokenToSol{
        tokenization_mint: pda_account_data.tokenization_mint,
        number_of_tokens: pda_account_data.number_of_tokens,
        lamports_per_token,
        tokens_sold: pda_account_data.tokens_sold,
    };

    if pda_account_data.tokenized_for_sale == 0 && pda_account_data.tokens_sold ==  pda_account_data.number_of_tokens && owner.owner == program_id {

      **temp.lamports.borrow_mut()-= pda_account_data.buy_out_price;
      **tokenized_nft_account.lamports.borrow_mut()+= pda_account_data.buy_out_price;
    
    }else{

      let tokens_left = pda_account_data.number_of_tokens - pda_account_data.tokens_sold;
      let owners_share = lamports_per_token * tokens_left;
      let comm_share = lamports_per_token * pda_account_data.tokens_sold;
      if comm_share+owners_share != pda_account_data.buy_out_price{panic!()}

      **temp.lamports.borrow_mut()-= owners_share;
      **owner.lamports.borrow_mut()+= owners_share;
      
      **temp.lamports.borrow_mut()-= comm_share;
      **tokenized_nft_account.lamports.borrow_mut()+= comm_share;
    }

    pda_account_data.owner = [0;32];
    pda_account_data.tokenization_mint = [0;32];
    pda_account_data.for_sale = 0;
    pda_account_data.tokenized_for_sale = 0;
    pda_account_data.number_of_tokens = 0;
    pda_account_data.tokens_sold = 0;
    pda_account_data.price = 0;
    pda_account_data.buy_out_price = 0;
    pda_account_data.lamports_per_token = 0;


    tokenization_account_data.serialize(&mut &mut tokenized_nft_account.data.borrow_mut()[..])?;
    pda_account_data.serialize(&mut &mut registered_nft_account.data.borrow_mut()[..])?;
   
    //buy out ucretini odeyip nftyi cuzdanina al

    Ok(())
  }

//////////////////////////////////////////////////////////////////////////////////

  fn tokenize_your_nft(
  accounts: &[AccountInfo],
  data:NFTState,
  program_id: &Pubkey
  ) -> ProgramResult {


  let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();

  let owner: &AccountInfo<'_> = next_account_info(accounts_iter)?;
  let owner_ata: &AccountInfo<'_> = next_account_info(accounts_iter)?;
  let owner_tokenization_ata: &AccountInfo<'_> = next_account_info(accounts_iter)?;
  let registered_nft_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;//nft owner in the program
  let registered_nft_account_ata: &AccountInfo<'_> = next_account_info(accounts_iter)?;//nft ata of the pda
  let nft_mint: &AccountInfo<'_> = next_account_info(accounts_iter)?;
  let tokenization_mint: &AccountInfo<'_> = next_account_info(accounts_iter)?;
  let token_program: &AccountInfo<'_> = next_account_info(accounts_iter)?;
  let token_program_2022: &AccountInfo<'_> = next_account_info(accounts_iter)?;
  let terms_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;
  let sysvar: &AccountInfo<'_> = next_account_info(accounts_iter)?;

  if terms_account.owner != program_id{panic!()}
  if registered_nft_account.owner != program_id{panic!()}

  if terms_account.is_writable{panic!()}
  let terms: Terms = Terms::try_from_slice(&terms_account.data.borrow())?;
  if terms.is_init != 1 {panic!()}
  

  if !owner.is_signer{panic!()}
  if owner_ata.owner!=&spl_token::id() && owner_ata.owner!=&spl_token_2022::id(){panic!()}
  if nft_mint.owner!=&spl_token::id() && nft_mint.owner!=&spl_token_2022::id(){panic!()}

  let owner_ata_unpacked: Account = Account::unpack_from_slice(&owner_ata.data.borrow())?;
  if nft_mint.key != &owner_ata_unpacked.mint {panic!()}
  if &owner_ata_unpacked.owner != owner.key{panic!()}
  

  let mut registered_nft_account_data: NFTState = NFTState::try_from_slice(&registered_nft_account.data.borrow())?;
  

  if registered_nft_account_data.for_sale != 0{panic!()}
  if registered_nft_account_data.tokenized_for_sale != 0{panic!()}

  let nft_from_bytes: Pubkey = Pubkey::new_from_array(registered_nft_account_data.nft_mint);

  if &nft_from_bytes != nft_mint.key {panic!()}

  //creating mint account
  let ix = &system_instruction::create_account(  
    &owner.key, 
    &tokenization_mint.key,
    terms.mint,
    terms.mint_size,
    &token_program_2022.key
  );

  let init_metadata_pointer = spl_token_2022::extension::metadata_pointer::instruction::initialize(
    token_program_2022.key,
    tokenization_mint.key,
    Some(*registered_nft_account.key),
    Some(*tokenization_mint.key),
  )?;

  //minting tokens to nft owner
  let ix_2 = spl_token_2022::instruction::initialize_mint(
    token_program_2022.key,
   tokenization_mint.key,
   registered_nft_account.key,
   Some(registered_nft_account.key),
   0)?;

  let ix_3 = create_associated_token_account(
    owner.key,
    owner.key,
    tokenization_mint.key,
    token_program_2022.key,
    );

  let ix_4 = spl_token_2022::instruction::mint_to_checked(
      token_program_2022.key,
      tokenization_mint.key,
      owner_tokenization_ata.key,
      registered_nft_account.key,
      &[&registered_nft_account.key],
      data.number_of_tokens,0
    )?;

  invoke(ix,  &[owner.clone(),tokenization_mint.clone(),token_program_2022.clone(),])?;
  invoke(&init_metadata_pointer,  &[registered_nft_account.clone(),tokenization_mint.clone(),token_program_2022.clone(),sysvar.clone()])?;
  invoke(&ix_2,  &[registered_nft_account.clone(),tokenization_mint.clone(),token_program_2022.clone(),sysvar.clone()])?;
  invoke(&ix_3,  &[owner.clone(),tokenization_mint.clone(),token_program_2022.clone(),sysvar.clone(),owner_tokenization_ata.clone()])?;
  
  let seed: &[u8] = &nft_mint.key.to_bytes();
  
  invoke_signed(&ix_4,
      &[registered_nft_account.clone(),tokenization_mint.clone(),token_program_2022.clone(),owner_tokenization_ata.clone()],
      &[&[seed, &[data.bump]]],
    )?;


  let transfer_nft_to_registered_nft_account_ata: solana_program::instruction::Instruction;

        //transfering nft to program
  if token_program.key == &spl_token::id(){
          transfer_nft_to_registered_nft_account_ata = spl_token::instruction::transfer_checked( &token_program.key,
            &owner_ata.key, 
            &nft_mint.key, 
            &registered_nft_account_ata.key, 
            &owner.key, 
            &[],1,0)?;
  }else if token_program.key == &spl_token_2022::id(){
          transfer_nft_to_registered_nft_account_ata = spl_token_2022::instruction::transfer_checked( &token_program.key,
            &owner_ata.key, 
            &nft_mint.key, 
            &registered_nft_account_ata.key, 
            &owner.key, 
            &[],1,0)?;
  }else{panic!()}

  invoke(&transfer_nft_to_registered_nft_account_ata,&[token_program.clone(),nft_mint.clone(),owner_ata.clone(),registered_nft_account_ata.clone(),owner.clone()])?; 


  if data.lamports_per_token < terms.minimum_lamports_per_token{panic!()}
  if data.lamports_per_token_buyout < data.lamports_per_token{panic!()}

  if data.lamports_per_token % terms.minimum_lamports_per_token != 0{panic!()}
  if data.lamports_per_token_buyout % terms.minimum_lamports_per_token != 0{panic!()}

  let price = data.number_of_tokens*data.lamports_per_token;
  let buy_out_price = data.number_of_tokens*data.lamports_per_token_buyout;

  registered_nft_account_data.owner= owner.key.to_bytes();
  registered_nft_account_data.nft_mint= nft_mint.key.to_bytes();
  registered_nft_account_data.tokenization_mint= tokenization_mint.key.to_bytes();
  registered_nft_account_data.for_sale=0;
  registered_nft_account_data.buy_out_allowed=1;
  registered_nft_account_data.owned_by_pda=1;
  registered_nft_account_data.tokenized_for_sale=0;
  registered_nft_account_data.price= price;
  registered_nft_account_data.buy_out_price= buy_out_price;
  registered_nft_account_data.lamports_per_token_buyout= data.lamports_per_token_buyout;
  registered_nft_account_data.number_of_tokens= data.number_of_tokens;
  registered_nft_account_data.lamports_per_token = data.lamports_per_token;
  registered_nft_account_data.tokens_sold= 0;
  registered_nft_account_data.bump=data.bump;


  registered_nft_account_data.serialize(&mut &mut registered_nft_account.data.borrow_mut()[..])?;

  //nft saticisi nftsini program gonderip tokenlara

  Ok(())
}

//////////////////////////////////////////////////////////////////////////////////

  fn init_voting_to_set_new_buy_out_price(
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

  fn repeat_voting_to_set_new_buy_out_price(
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

  fn vote(
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

  fn set_new_buyout_price_after_voting(
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

    //nftnin likide olmasi icin oylama baslatir


    Ok(())
  }

  fn init_voter_account(
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

////////////////////////////////////////////////////////////////////////////////////

  fn sell_nft_owned_by_program_to_investor(
    accounts: &[AccountInfo],
    program_id: &Pubkey) -> ProgramResult {


      let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();

      let proposer: &AccountInfo<'_> = next_account_info(accounts_iter)?;
      let proposer_ata: &AccountInfo<'_> = next_account_info(accounts_iter)?;
      let proposer_investor_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;
      let proposal_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;
      let registered_nft_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;//transfer nft back
      let registered_nft_account_ata: &AccountInfo<'_> = next_account_info(accounts_iter)?;//nft ata of the pda
      let nft_mint: &AccountInfo<'_> = next_account_info(accounts_iter)?;//check
      let tokenized_nft_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;
      let token_program: &AccountInfo<'_> = next_account_info(accounts_iter)?;
      let terms_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;

      if terms_account.owner != program_id{panic!()}
      if terms_account.is_writable{panic!()}
      let terms: Terms = Terms::try_from_slice(&terms_account.data.borrow())?;
      if terms.is_init != 1 {panic!()}
  
      if registered_nft_account.owner != program_id {panic!()}
      if proposer_investor_account.owner != program_id {panic!()}
      if proposal_account.owner != program_id {panic!()}

      if proposer_ata.owner!=&spl_token::id() && proposer_ata.owner!=&spl_token_2022::id(){panic!()}
      if nft_mint.owner!=&spl_token::id() && nft_mint.owner!=&spl_token_2022::id(){panic!()}
      if registered_nft_account_ata.owner!=&spl_token::id() && registered_nft_account_ata.owner!=&spl_token_2022::id(){panic!()}
  
      let proposer_ata_unpacked: Account = Account::unpack_from_slice(&proposer_ata.data.borrow())?;
      if nft_mint.key != &proposer_ata_unpacked.mint {panic!()}
      if &proposer_ata_unpacked.owner != proposer.key{panic!()}
  
      let registered_nft_account_ata_unpacked: Account = Account::unpack_from_slice(&registered_nft_account_ata.data.borrow())?;
      if nft_mint.key != &registered_nft_account_ata_unpacked.mint {panic!()}
      if &registered_nft_account_ata_unpacked.owner != registered_nft_account.key{panic!()}
  
      let mut pda_account_data: NFTState = NFTState::try_from_slice(&registered_nft_account.data.borrow())?;
      let mut investor_acc: InvestorAccount = InvestorAccount::try_from_slice(&proposer_investor_account.data.borrow())?;
      let proposal: Proposal = Proposal::try_from_slice(&proposal_account.data.borrow())?;


      let owner_key_from_bytes: Pubkey = Pubkey::new_from_array(pda_account_data.owner);
      let nft_mint_key_from_bytes: Pubkey = Pubkey::new_from_array(pda_account_data.nft_mint);
      let nft_mint_key_from_bytes2: Pubkey = Pubkey::new_from_array(proposal.nft_mint);
      let proposer_key_from_bytes: Pubkey = Pubkey::new_from_array(investor_acc.investor);
      let proposer_key_from_bytes2: Pubkey = Pubkey::new_from_array(proposal.proposer);

      if &owner_key_from_bytes != registered_nft_account.key{panic!()}
      if &nft_mint_key_from_bytes != nft_mint.key{panic!()}
      if &nft_mint_key_from_bytes2 != nft_mint.key{panic!()}
      if &proposer_key_from_bytes != proposer.key{panic!()}
      if &proposer_key_from_bytes2 != proposer.key{panic!()}

      if proposal.offer < pda_account_data.buy_out_price{panic!()}
      if proposal.offer > investor_acc.lamports{panic!()}
      if pda_account_data.buy_out_allowed != 1{panic!()}
        //transfering nft to program
        let transfer_nft_to_buyer: solana_program::instruction::Instruction;


      if token_program.key == &spl_token::id(){
          transfer_nft_to_buyer=spl_token::instruction::transfer_checked( &token_program.key,
            &registered_nft_account_ata.key, 
             &nft_mint.key, 
             &proposer_ata.key, 
             &registered_nft_account.key, 
             &[], 
             1, 
             0)?;
      }else if token_program.key == &spl_token_2022::id(){
          transfer_nft_to_buyer=spl_token_2022::instruction::transfer_checked(&token_program.key,
            &registered_nft_account_ata.key, 
             &nft_mint.key, 
             &proposer_ata.key, 
             &registered_nft_account.key, 
             &[], 
             1, 
             0)?;
      }else{panic!()}


      let seed: &[u8] = &nft_mint.key.to_bytes();

      invoke_signed(
          &transfer_nft_to_buyer,
          &[
            proposer_ata.clone(),
          registered_nft_account_ata.clone(),
          registered_nft_account.clone(), 
          token_program.clone(),
          nft_mint.clone(),
        ],
          &[&[seed, &[pda_account_data.bump]]],
      )?;

      let lamports_per_token = pda_account_data.buy_out_price/pda_account_data.number_of_tokens;


      let ix: &solana_program::instruction::Instruction = &system_instruction::create_account(  
        &proposer.key, 
        &tokenized_nft_account.key,
        terms.token_to_sol_account,
        terms.token_to_sol_account_size,
        &program_id
      );
  
      invoke(&ix,  &[proposer.clone(),tokenized_nft_account.clone(),])?;

      let tokenization_account_data: TokenToSol = TokenToSol{
          tokenization_mint: pda_account_data.tokenization_mint,
          number_of_tokens: pda_account_data.number_of_tokens,
          lamports_per_token,
          tokens_sold: pda_account_data.tokens_sold,
      };
  
      pda_account_data.owner = [0;32];
      pda_account_data.tokenization_mint = [0;32];
      pda_account_data.for_sale = 0;
      pda_account_data.number_of_tokens = 0;
      pda_account_data.tokens_sold = 0;
      pda_account_data.price = 0;
      pda_account_data.buy_out_price = 0;
      pda_account_data.lamports_per_token = 0;

      investor_acc.lamports -= pda_account_data.buy_out_price;

      let value: u64 = **proposal_account.lamports.borrow();
      **proposal_account.lamports.borrow_mut()-= value;
      **proposer.lamports.borrow_mut()+= value;

      **proposer_investor_account.lamports.borrow_mut()-= pda_account_data.buy_out_price;
      **tokenized_nft_account.lamports.borrow_mut()+= pda_account_data.buy_out_price;

      tokenization_account_data.serialize(&mut &mut tokenized_nft_account.data.borrow_mut()[..])?;
      pda_account_data.serialize(&mut &mut registered_nft_account.data.borrow_mut()[..])?;
      investor_acc.serialize(&mut &mut proposer_investor_account.data.borrow_mut()[..])?;

    Ok(())
  }

  fn sell_nft_owned_by_individual_to_investor(
    accounts: &[AccountInfo],
    program_id: &Pubkey) -> ProgramResult {


      let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();


      let seller: &AccountInfo<'_> = next_account_info(accounts_iter)?;
      let seller_ata: &AccountInfo<'_> = next_account_info(accounts_iter)?;
      let proposer: &AccountInfo<'_> = next_account_info(accounts_iter)?;
      let proposer_ata: &AccountInfo<'_> = next_account_info(accounts_iter)?;
      let proposer_investor_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;
      let proposal_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;
      let nft_mint: &AccountInfo<'_> = next_account_info(accounts_iter)?;//check
      let token_program: &AccountInfo<'_> = next_account_info(accounts_iter)?;
  
      if !seller.is_signer{panic!()}
      if proposer_investor_account.owner != program_id {panic!()}
      if proposal_account.owner != program_id {panic!()}

      if proposer_ata.owner!=&spl_token::id() && proposer_ata.owner!=&spl_token_2022::id(){panic!()}
      if nft_mint.owner!=&spl_token::id() && nft_mint.owner!=&spl_token_2022::id(){panic!()}
  
      let proposer_ata_unpacked: Account = Account::unpack_from_slice(&proposer_ata.data.borrow())?;
      if nft_mint.key != &proposer_ata_unpacked.mint {panic!()}
      if &proposer_ata_unpacked.owner != proposer.key{panic!()}
  
      let seller_ata_unpacked :Account = Account::unpack_from_slice(&seller_ata.data.borrow())?;
      if nft_mint.key != &seller_ata_unpacked.mint {panic!()}
      if &seller_ata_unpacked.owner != seller.key{panic!()}
  
      let mut investor_acc: InvestorAccount = InvestorAccount::try_from_slice(&proposer_investor_account.data.borrow())?;
      let proposal: Proposal = Proposal::try_from_slice(&proposal_account.data.borrow())?;
  
  
      let nft_mint_key_from_bytes2: Pubkey = Pubkey::new_from_array(proposal.nft_mint);
      let proposer_key_from_bytes: Pubkey = Pubkey::new_from_array(investor_acc.investor);
      let proposer_key_from_bytes2: Pubkey = Pubkey::new_from_array(proposal.proposer);
  

      if &nft_mint_key_from_bytes2 != nft_mint.key{panic!()}
      if &proposer_key_from_bytes != proposer.key{panic!()}
      if &proposer_key_from_bytes2 != proposer.key{panic!()}

      if proposal.offer > investor_acc.lamports{panic!()}


        let transfer_nft_to_proposer: solana_program::instruction::Instruction;


        if token_program.key == &spl_token::id(){
          transfer_nft_to_proposer=spl_token::instruction::transfer_checked( &token_program.key,
            &seller_ata.key, 
             &nft_mint.key, 
             &proposer_ata.key, 
             &seller.key, 
             &[], 
             1, 
             0)?;
        }else if token_program.key == &spl_token_2022::id(){
          transfer_nft_to_proposer=spl_token_2022::instruction::transfer_checked(&token_program.key,
            &seller_ata.key, 
             &nft_mint.key, 
             &proposer_ata.key, 
             &seller.key, 
             &[], 
             1, 
             0)?;

        }else{panic!()}
  
  
      invoke(
          &transfer_nft_to_proposer,
          &[
            proposer_ata.clone(),
            seller_ata.clone(),
            seller.clone(), 
          token_program.clone(),
          nft_mint.clone()
        ],
      )?;

      investor_acc.lamports -= proposal.offer;


      **proposer_investor_account.lamports.borrow_mut()-= proposal.offer;
      **seller.lamports.borrow_mut()+= proposal.offer;

      investor_acc.serialize(&mut &mut proposer_investor_account.data.borrow_mut()[..])?;

    Ok(())
  }

  fn make_an_offer_for_nft(
  accounts: &[AccountInfo],
  program_id: &Pubkey,
  data:Lamports
  ) -> ProgramResult {


  let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();

  let proposer: &AccountInfo<'_> = next_account_info(accounts_iter)?;
  let proposer_ata: &AccountInfo<'_> = next_account_info(accounts_iter)?;
  let proposer_investor_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;
  let proposal_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;
  let nft_mint: &AccountInfo<'_> = next_account_info(accounts_iter)?;
  let token_program: &AccountInfo<'_> = next_account_info(accounts_iter)?;
  let terms_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;
  let sysvar: &AccountInfo<'_> = next_account_info(accounts_iter)?;

  if terms_account.owner != program_id{panic!()}

  if terms_account.is_writable{panic!()}
  let terms: Terms = Terms::try_from_slice(&terms_account.data.borrow())?;
  if terms.is_init != 1 {panic!()}

  if !proposer.is_signer{panic!()}

  let create_buyer_ata: solana_program::instruction::Instruction = create_associated_token_account(
    proposer.key,
    proposer.key, 
    nft_mint.key, 
    token_program.key);

  invoke(&create_buyer_ata,
      &[proposer.clone(),proposer_ata.clone(),nft_mint.clone(),token_program.clone(),sysvar.clone()])?;

  let investor_acc: InvestorAccount = InvestorAccount::try_from_slice(&proposer_investor_account.data.borrow())?;


  if investor_acc.lamports < data.lamports{panic!()}

  let ix: &solana_program::instruction::Instruction = &system_instruction::create_account(  
    &proposer.key, 
    &proposal_account.key,
    terms.proposal_account,
    terms.proposal_account_size,
    &program_id
  );

  let proposal = Proposal{
    proposer:proposer.key.to_bytes(),
    nft_mint:nft_mint.key.to_bytes(),
    offer:data.lamports
  };

  invoke(&ix,  &[proposal_account.clone(),proposer.clone()])?;

  proposal.serialize(&mut &mut proposal_account.data.borrow_mut()[..])?;


   //fonksiyon belirlenen mintteki nftnin alimi icin teklif acar.
   //satis gerceklesirse teklifteki bedel alicinin yatirim hesabindan cekilir.
   //teklif bir kisiye aittir.
   //nft sahibi teklifi kabul ederse nft alici cuzdanina gonderilir 
   //nft sahibinin cuzdanina teklifteki tutar gonderilir

  Ok(())
}

  fn create_investor_account(
    accounts: &[AccountInfo],
    program_id: &Pubkey,
    data:Lamports) -> ProgramResult {


    let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();

    let investor: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let investor_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let terms_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;
  
    if terms_account.owner != program_id{panic!()}
  
    if terms_account.is_writable{panic!()}
    let terms: Terms = Terms::try_from_slice(&terms_account.data.borrow())?;
    if terms.is_init != 1 {panic!()}

    if !investor.is_signer{panic!()}


    if **investor.lamports.borrow() < data.lamports{panic!()}

    let ix: &solana_program::instruction::Instruction = &system_instruction::create_account(  
      &investor.key, 
      &investor_account.key,
      terms.investor_account,
      terms.investor_account_size,
      &program_id
    );

    let inv = InvestorAccount{
      investor:investor.key.to_bytes(),
      lamports:data.lamports
    };

    invoke(&ix,  &[investor_account.clone(),investor.clone()])?;

    inv.serialize(&mut &mut investor_account.data.borrow_mut()[..])?;

    //yatirimci hesabi yatirimcinin birden fazla nft icin teklif acabilmesine olanak saglar.
    //yatirimci hesabinda yeterli yatirim kalmamissa nft icin verdigi teklifler goruntulenmez

    Ok(())
  }

  fn fund_investor_account(
    accounts: &[AccountInfo],
    program_id: &Pubkey,
    data:Lamports) -> ProgramResult {

    let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();

    let investor: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let investor_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let temp: &AccountInfo<'_> = next_account_info(accounts_iter)?;

    let ix = &system_instruction::create_account(  
      &investor.key, 
      &temp.key,
      data.lamports,
      0,
      &program_id);

    invoke(ix,  &[temp.clone(),investor.clone()])?;

    if !investor.is_signer{panic!()}
    if investor_account.owner != program_id{panic!()}

    let mut investor_acc: InvestorAccount = InvestorAccount::try_from_slice(&investor_account.data.borrow())?;


    **temp.lamports.borrow_mut()-= data.lamports;
    **investor_account.lamports.borrow_mut()+= data.lamports;

    let inverstor_address_from_bytes: Pubkey = Pubkey::new_from_array(investor_acc.investor);

    if &inverstor_address_from_bytes != investor.key {panic!()}
    
    investor_acc.lamports += data.lamports;

    investor_acc.serialize(&mut &mut investor_account.data.borrow_mut()[..])?;


    Ok(())
  }

  fn remove_funds_from_investor_account(
    accounts: &[AccountInfo],
    program_id: &Pubkey,
    data:Lamports) -> ProgramResult {

    let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();

    let investor: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let investor_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;

    if !investor.is_signer{panic!()}
    if investor_account.owner != program_id{panic!()}

    let mut investor_acc: InvestorAccount = InvestorAccount::try_from_slice(&investor_account.data.borrow())?;

    if **investor_account.lamports.borrow() < data.lamports{panic!()}
    
    **investor_account.lamports.borrow_mut()-= data.lamports;
    **investor.lamports.borrow_mut() += data.lamports;

    let inverstor_address_from_bytes: Pubkey = Pubkey::new_from_array(investor_acc.investor);

    if &inverstor_address_from_bytes != investor.key {panic!()}
    
    investor_acc.lamports -= data.lamports;

    investor_acc.serialize(&mut &mut investor_account.data.borrow_mut()[..])?;


    Ok(())
  }

  //fn create_metadata_for_tokenized_nft_tokens(accounts: &[AccountInfo], program_id: &Pubkey  ){}
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

use crate::instruction::NFTInstruction;
use crate::state::{DistData, FundRaising, FunderAccount, InitPDA, InitVoting, InvestorAccount, Lamports, InitAccount, NFTTerms, NFTToken, Proposal, Terms, UserAddresTokenMint, VoteAccount, VoteData};
use borsh::{BorshDeserialize, BorshSerialize};



use spl_associated_token_account::instruction::create_associated_token_account;

use solana_program::{
  account_info::{next_account_info, AccountInfo},
  entrypoint::ProgramResult,
  pubkey::Pubkey,
  sysvar::{clock::Clock, Sysvar,},
  system_instruction,
  program::{invoke_signed,invoke},
  program_pack::Pack,

};

use spl_token::instruction::{initialize_mint, mint_to, close_account,burn};


use spl_token::state::Account;
/*
use mpl_token_metadata::{
  instructions,
  types::DataV2
  };
*/

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
        Self::buy_nft_with_the_funds(accounts,program_id)
      }
      NFTInstruction::SellNFTtoFund {} => {
        Self::sell_nft_to_the_fundraising(accounts,program_id)
      }
      NFTInstruction::CreateFunder {} => {
        Self::create_funding_account(accounts,program_id)
      }
      NFTInstruction::CreateFundRaisingAcc {data} => {
        Self::create_fundraising_account(accounts,program_id,data)
      }
      NFTInstruction::SellWholeNFT {data,data2} => {
        Self::list_nft_forsale_as_whole_in_this_program(accounts,program_id,data,data2)
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
      NFTInstruction::TokenizeNFTSell {data,data2} => {
        Self::tokenize_nft_and_sell_in_this_program(accounts,program_id,data,data2)
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
      NFTInstruction::TokenizeNFT {data,data2} => {
        Self::tokenize_your_nft(accounts,data,data2)
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
      NFTInstruction::CreateMeta {data} => {
        Self::create_metadata(accounts,program_id,data)
      }
      NFTInstruction::Register {data} => {
        Self::register_nft_in_program(accounts,program_id,data)
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
    let fundrasing_account: &AccountInfo<'_> = next_account_info(accounts_iter)?; //fon toplama girisim hesabi. her NFT icin bir tane bulunur.
    let token_dist_data: &AccountInfo<'_> = next_account_info(accounts_iter)?; //Fon toplama girisimi basarili olursa yatirimci tokenlarini almak icin bu hesabi kullanir
    let pda: &AccountInfo<'_> = next_account_info(accounts_iter)?; //NFT ile ilgili butun bilgilerin tutuldugu hesaptir
    let tokenization_mint: &AccountInfo<'_> = next_account_info(accounts_iter)?; //NFT'nin tokenize olmus halinin adresi - fungible assets
    let token_program: &AccountInfo<'_> = next_account_info(accounts_iter)?; 
    let terms_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;//Hesaplarin size ve rent datasi burda
    let sysvar: &AccountInfo<'_> = next_account_info(accounts_iter)?;

    if terms_account.owner != program_id{panic!()}
    if fundrasing_account.owner != program_id{panic!()}
    if pda.owner != program_id{panic!()}

    let terms: Terms = Terms::try_from_slice(&terms_account.data.borrow())?;

    if terms.is_init != 1 {panic!()}

    if !initializer.is_signer{panic!()}

    let mut fundraising: FundRaising = FundRaising::try_from_slice(&fundrasing_account.data.borrow())?;
    let pda_data: NFTTerms = NFTTerms::try_from_slice(&pda.data.borrow())?;

    if fundraising.fund_raising != 0 {panic!()} //if there is an active fundrasing panic
    if pda_data.tokenized_for_sale != 0 {panic!()} //if the nft is tokenized cant start a fundrasing. go buy tokens
    if pda_data.owned_by_pda != 0 {panic!()} //if nft already owned by a community cant start a funsraise


    let nft_mint_from_bytes2: Pubkey = Pubkey::new_from_array(pda_data.nft_mint);
    let nft_mint_from_bytes3: Pubkey = Pubkey::new_from_array(fundraising.nft_mint);
    if nft_mint_from_bytes3 != nft_mint_from_bytes2 {panic!()}

    //creating mint account
    let ix = &system_instruction::create_account(  
      &initializer.key, 
      &tokenization_mint.key,
      terms.tokenization_account,
      terms.tokenization_account_size,
      &token_program.key);
    
    //initializing mint for the nft token
    let ix_2 = initialize_mint(
      token_program.key,
      tokenization_mint.key,
      token_dist_data.key,
      Some(token_dist_data.key),
      0)?;
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

    invoke(ix,  &[initializer.clone(),tokenization_mint.clone(),token_program.clone(),])?;
    invoke(&ix_2,  &[token_dist_data.clone(),tokenization_mint.clone(),token_program.clone(),sysvar.clone()])?;

    let number_of_tokens: u64 = create_account.lamports/1000000; //each 1 million lamports is 1 token. 

    let fund_raise: u64 = number_of_tokens*1000000; // Investors can invest 1 million lamports and multipliers of it

    let lamport_per_tokens: u64 = 1000000;

    fundraising.fund_raising=1; // fundraising is set to active
    fundraising.tokens_mint = tokenization_mint.key.to_bytes(); //tokenization mint address of the nft - investor mint these tokens if FR successful
    fundraising.funds_collected = fund_raise; //funds that initializer added
    fundraising.number_of_tokens; //tokenization is proportional to funds
    fundraising.lamports_per_token = lamport_per_tokens; //fixed number

    let value: u64 = **initializer.lamports.borrow();

    if value < fund_raise {panic!()}


  **initializer.lamports.borrow_mut()-= fund_raise;
  **fundrasing_account.lamports.borrow_mut()+= fund_raise;


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


    let number_of_tokens: u64 = data.lamports/1000000;

    let fund_raise: u64 = number_of_tokens*1000000;

    let value: u64 = **funder.lamports.borrow();

    if value < fund_raise {panic!()}

    **funder.lamports.borrow_mut()-= fund_raise;
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

    let number_of_tokens: u64 = data.lamports/1000000;

    let fund_raise: u64 = number_of_tokens*1000000;


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
    let token_program: &AccountInfo<'_> = next_account_info(accounts_iter)?;

    if !funder.is_signer{panic!()}

    if funder_funds_account.owner != program_id{panic!()}
    if token_distribution_data.owner != program_id{panic!()}
    
    let funder_ata_unpacked: Account = Account::unpack_from_slice(&funder_ata.data.borrow())?;
    if tokenization_mint.key != &funder_ata_unpacked.mint {panic!()}
    if &funder_ata_unpacked.owner != funder.key{panic!()}

    let funder_account: FunderAccount = FunderAccount::try_from_slice(&funder_funds_account.data.borrow())?;
    let mut distribution: DistData = DistData::try_from_slice(&token_distribution_data.data.borrow())?;

    if distribution.distribution_open != 1{panic!()}

    let funder_address_from_bytes: Pubkey = Pubkey::new_from_array(funder_account.funder);
    if &funder_address_from_bytes != funder.key {panic!()}

    let tokenization_mint_from_bytes: Pubkey = Pubkey::new_from_array(funder_account.tokens_mint);
    if tokenization_mint.key != &tokenization_mint_from_bytes {panic!()}

    let tokens_to_receive: u64 = funder_account.fund_invested/1000000;

    distribution.tokens_left -= tokens_to_receive;
    
    let ix = mint_to(
      token_program.key,
      tokenization_mint.key, 
      funder_ata.key, 
      token_distribution_data.key,
      &[token_distribution_data.key],
      tokens_to_receive)?;

      let seed: &[u8] = &tokenization_mint.key.to_bytes();

    invoke_signed(
        &ix,
        &[
          funder_ata.clone(),
          tokenization_mint.clone(),
          token_distribution_data.clone(), 
          token_program.clone()
        ],
        &[&[seed, &[distribution.bump]]],
      )?;

      let val: u64 =  **funder_funds_account.lamports.borrow();

      **funder_funds_account.lamports.borrow_mut()-= val;
      **funder.lamports.borrow_mut()+= val;


      distribution.serialize(&mut &mut token_distribution_data.data.borrow_mut()[..])?;

    Ok(())
  }

  fn buy_nft_with_the_funds(
    accounts: &[AccountInfo],
    program_id: &Pubkey
  ) -> ProgramResult {


    let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();

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
    let pda: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let pda_ata: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let nft_mint: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let token_program: &AccountInfo<'_> = next_account_info(accounts_iter)?;

    if !seller.is_signer{panic!()}

    if token_distribution_data.owner != program_id{panic!()}
    if fundraising_account.owner != program_id{panic!()}
    if pda.owner != program_id{panic!()}

    let mut fundraising: FundRaising = FundRaising::try_from_slice(&fundraising_account.data.borrow())?;
    let mut pda_account: NFTTerms = NFTTerms::try_from_slice(&pda.data.borrow())?;
    let mut distribution: DistData = DistData::try_from_slice(&token_distribution_data.data.borrow())?;

    let seed: &[u8] = &fundraising.tokens_mint;

    let derived_dist: Pubkey = Pubkey::create_program_address(&[seed, &[distribution.bump]], program_id)?;

    if &derived_dist != token_distribution_data.key {panic!()}
    if fundraising.fund_raising != 1{panic!()}

    let seller_ata_unpacked: Account = Account::unpack_from_slice(&seller_ata.data.borrow())?;
    if nft_mint.key != &seller_ata_unpacked.mint {panic!()}
    if &seller_ata_unpacked.owner != seller.key{panic!()}

    let pda_ata_unpacked: Account = Account::unpack_from_slice(&pda_ata.data.borrow())?;
    if nft_mint.key != &pda_ata_unpacked.mint {panic!()}
    if &pda_ata_unpacked.owner != pda.key{panic!()}

    let nft_mint_from_bytes: Pubkey = Pubkey::new_from_array(fundraising.nft_mint);
    if &nft_mint_from_bytes != nft_mint.key {panic!()}

    let nft_mint_from_bytes2: Pubkey = Pubkey::new_from_array(pda_account.nft_mint);
    if &nft_mint_from_bytes2 != nft_mint.key {panic!()}

    let transfer_nft_to_pda_ata = spl_token::instruction::transfer( &token_program.key,
      &seller_ata.key, 
      &pda_ata.key, 
      &seller.key, 
      &[&seller.key], 
      1)?;

  invoke(&transfer_nft_to_pda_ata,&[token_program.clone(),seller_ata.clone(),pda_ata.clone(),seller.clone()])?; 

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


    if !funder.is_signer{panic!()}

    if funder_funds_account.owner != program_id{panic!()}
    if fundraising_account.owner != program_id{panic!()}
    

    let fundraising: FundRaising = FundRaising::try_from_slice(&fundraising_account.data.borrow())?;

    if fundraising.fund_raising != 1{panic!()}


    let tokenization_mint_from_bytes: Pubkey = Pubkey::new_from_array(fundraising.tokens_mint);
    if &tokenization_mint_from_bytes != tokenization_mint.key {panic!()}

    invoke(
      &system_instruction::create_account(  
          &funder.key, 
          &funder_funds_account.key,
          1000000,
          72,
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
      fund_invested:0
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

    let funder: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let fundraising_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let nft_mint: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let terms_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;

    if terms_account.owner != program_id{panic!()}
    if fundraising_account.owner != program_id{panic!()}

    let terms: Terms = Terms::try_from_slice(&terms_account.data.borrow())?;

    if terms.is_init != 1 {panic!()}

    if nft_mint.owner != &spl_token::id(){panic!()}

    if !funder.is_signer{panic!()}

    let seed: &[u8] = &nft_mint.key.to_bytes();

    invoke_signed(
        &system_instruction::create_account(
          &funder.key, 
          &fundraising_account.key,
          terms.funder_account,
            terms.fundrasing_account_size, 
            program_id),
        &[
          funder.clone(),
          fundraising_account.clone(),

        ],
        &[&[seed, &[data.init_pda]]],
     )?;

    let funds = FundRaising{ 
      fund_raising:0,
      nft_mint: nft_mint.key.to_bytes(),
      tokens_mint:[0;32],
      funds_collected: 0,
      number_of_tokens: 0,
      lamports_per_token: 1000000,
      bump:data.init_pda
      };

    funds.serialize(&mut &mut fundraising_account.data.borrow_mut()[..])?;

    Ok(())
  }

//////////////////////////////////////////////////////////////////////////////////


//TODO fiyat belirleme
  fn list_nft_forsale_as_whole_in_this_program(
    accounts: &[AccountInfo],
    program_id: &Pubkey,
    create_account: InitAccount,
    data: NFTTerms,
    ) -> ProgramResult {

      //nft saticisi pazar yerimizde nftsini satisa cikarir.
      //pesin odeme talep eder. Birisi pesin odeme ile yada bir fon toplama girisimi alabilir 

    let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();

    let seller: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let seller_ata: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let pda: &AccountInfo<'_> = next_account_info(accounts_iter)?;//nft owner in the program
    let pda_ata: &AccountInfo<'_> = next_account_info(accounts_iter)?;//nft ata of the pda
    let nft_mint: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let tokenized_nft_token_mint: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let token_program: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let sysvar: &AccountInfo<'_> = next_account_info(accounts_iter)?;

    if !seller.is_signer{panic!()}
    if seller_ata.owner!=&spl_token::id(){panic!()}//token2022 degil
    if nft_mint.owner!=&spl_token::id(){panic!()}//token2022 degil

    if pda.owner != program_id{panic!()}

    let seller_ata_unpacked: Account = Account::unpack_from_slice(&seller_ata.data.borrow())?;
    if nft_mint.key != &seller_ata_unpacked.mint {panic!()}
    if &seller_ata_unpacked.owner != seller.key{panic!()}

    let mut pda_account_data: NFTTerms = NFTTerms::try_from_slice(&pda.data.borrow())?;

    if pda_account_data.tokenized_for_sale != 0{panic!()}
    if pda_account_data.for_sale != 0{panic!()}

    let nft_from_bytes: Pubkey = Pubkey::new_from_array(pda_account_data.nft_mint);

    if &nft_from_bytes != nft_mint.key {panic!()}

    //creating mint account
    let ix: &solana_program::instruction::Instruction = &system_instruction::create_account(  
      &seller.key, 
      &tokenized_nft_token_mint.key,
      create_account.lamports,
      create_account.size,
      &token_program.key
    );

    //initializing mint for the nft token
    let ix_2 = initialize_mint(
     token_program.key,
     tokenized_nft_token_mint.key,
     pda.key,
     Some(pda.key),
     0)?;


    invoke(ix,  &[seller.clone(),tokenized_nft_token_mint.clone(),token_program.clone(),])?;
    invoke(&ix_2,  &[pda.clone(),tokenized_nft_token_mint.clone(),token_program.clone(),sysvar.clone()])?;


    //transfering nft to program
    let transfer_nft_to_pda_ata =spl_token::instruction::transfer( &token_program.key,
        &seller_ata.key, 
        &pda_ata.key, 
        &seller.key, 
        &[&seller.key], 
        1)?;

    invoke(&transfer_nft_to_pda_ata,&[token_program.clone(),seller_ata.clone(),pda_ata.clone(),seller.clone()])?; 

    let number_of_tokens = data.price/1000000;

    if data.price < 1000000{panic!()}

    let price: u64 = number_of_tokens*1000000;

    let lamports_per_token: u64 = 1000000;

    pda_account_data.owner= seller.key.to_bytes();
    pda_account_data.tokenization_mint= tokenized_nft_token_mint.key.to_bytes();
    pda_account_data.for_sale=1;
    pda_account_data.price= price;
    pda_account_data.buy_out_price= data.buy_out_price;
    pda_account_data.list_in_main_page= data.list_in_main_page;
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
    let pda: &AccountInfo<'_> = next_account_info(accounts_iter)?;//transfer nft back
    let pda_ata: &AccountInfo<'_> = next_account_info(accounts_iter)?;//nft ata of the pda
    let nft_mint: &AccountInfo<'_> = next_account_info(accounts_iter)?;//check
    let token_program: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let terms_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;

    if terms_account.owner != program_id{panic!()}

    let terms: Terms = Terms::try_from_slice(&terms_account.data.borrow())?;
    if terms.is_init != 1{panic!()}

    if pda.owner != program_id {panic!()}

    if !seller.is_signer{panic!()}
    if seller_ata.owner!=&spl_token::id(){panic!()}//token2022 degil
    if nft_mint.owner!=&spl_token::id(){panic!()}//token2022 degil

    let seller_ata_unpacked: Account = Account::unpack_from_slice(&seller_ata.data.borrow())?;
    if nft_mint.key != &seller_ata_unpacked.mint {panic!()}
    if &seller_ata_unpacked.owner != seller.key{panic!()}

    let pda_ata_unpacked: Account = Account::unpack_from_slice(&pda_ata.data.borrow())?;
    if nft_mint.key != &pda_ata_unpacked.mint {panic!()}
    if &pda_ata_unpacked.owner != pda.key{panic!()}

    let mut pda_account_data: NFTTerms = NFTTerms::try_from_slice(&pda.data.borrow())?;

    if pda_account_data.for_sale != 1{panic!()}
    if pda_account_data.tokenized_for_sale != 0{panic!()}

    let seller_key_from_bytes: Pubkey = Pubkey::new_from_array(pda_account_data.owner);
    let nft_mint_key_from_bytes: Pubkey = Pubkey::new_from_array(pda_account_data.nft_mint);

    if &seller_key_from_bytes != seller.key{panic!()}
    if &nft_mint_key_from_bytes != nft_mint.key{panic!()}

    let transfer_nft_to_seller=spl_token::instruction::transfer( &token_program.key,
      &pda_ata.key, 
      &seller_ata.key, 
      &pda.key, 
      &[&pda.key], 
      1)?;

  let seed: &[u8] = &nft_mint.key.to_bytes();

    //transfer nft back to seller
    invoke_signed(
      &transfer_nft_to_seller,
      &[
        seller_ata.clone(),
        pda_ata.clone(),
        pda.clone(), 
        token_program.clone()
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

    pda_account_data.serialize(&mut &mut pda.data.borrow_mut()[..])?;


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
    let buyer_ata: &AccountInfo<'_> = next_account_info(accounts_iter)?; 
    let pda: &AccountInfo<'_> = next_account_info(accounts_iter)?;//transfer nft back
    let pda_ata: &AccountInfo<'_> = next_account_info(accounts_iter)?;//nft ata of the pda
    let nft_mint: &AccountInfo<'_> = next_account_info(accounts_iter)?;//check
    let token_program: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let tokenized_nft_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let terms_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;

    if terms_account.owner != program_id{panic!()}

    let terms: Terms = Terms::try_from_slice(&terms_account.data.borrow())?;
    if terms.is_init != 1{panic!()}

    if pda.owner != program_id {panic!()}
    if !buyer.is_signer {panic!()}

    if buyer_ata.owner!=&spl_token::id(){panic!()}//token2022 degil
    if nft_mint.owner!=&spl_token::id(){panic!()}//token2022 degil

    let buyer_ata_unpacked: Account = Account::unpack_from_slice(&buyer_ata.data.borrow())?;
    if nft_mint.key != &buyer_ata_unpacked.mint {panic!()}
    if &buyer_ata_unpacked.owner != buyer.key{panic!()}

    let pda_ata_unpacked: Account = Account::unpack_from_slice(&pda_ata.data.borrow())?;
    if nft_mint.key != &pda_ata_unpacked.mint {panic!()}
    if &pda_ata_unpacked.owner != pda.key{panic!()}

    let mut pda_account_data: NFTTerms = NFTTerms::try_from_slice(&pda.data.borrow())?;

    if pda_account_data.for_sale != 1{panic!()}
    if pda_account_data.tokenized_for_sale != 0{panic!()}

    let seller_key_from_bytes: Pubkey = Pubkey::new_from_array(pda_account_data.owner);
    let nft_mint_key_from_bytes: Pubkey = Pubkey::new_from_array(pda_account_data.nft_mint);

    if &seller_key_from_bytes != seller.key{panic!()}
    if &nft_mint_key_from_bytes != nft_mint.key{panic!()}

    let value: u64 = **buyer.lamports.borrow();

    if value < pda_account_data.price {panic!()}

    **buyer.lamports.borrow_mut()-= pda_account_data.price;
    **seller.lamports.borrow_mut()+= pda_account_data.price;


    let transfer_nft_to_buyer =spl_token::instruction::transfer( &token_program.key,
      &pda_ata.key, 
      &buyer_ata.key, 
      &pda.key, 
      &[&pda.key], 
      1)?;
    let seed: &[u8] = &nft_mint.key.to_bytes();

    invoke_signed(
        &transfer_nft_to_buyer,
        &[
          buyer_ata.clone(),
          pda_ata.clone(),
          pda.clone(), 
          token_program.clone()
        ],
        &[&[seed, &[pda_account_data.bump]]],
     )?;

    let ix: &solana_program::instruction::Instruction = &system_instruction::create_account(  
      &seller.key, 
      &tokenized_nft_account.key,
      1000000,
      50,
      &program_id
    );

    invoke(&ix,  &[buyer.clone(),token_program.clone(),tokenized_nft_account.clone(),])?;

    let tokenization_account_data: NFTToken = NFTToken{
        tokenized_nft_mint: pda_account_data.tokenization_mint,
        number_of_tokens: pda_account_data.number_of_tokens,
        lamports_per_token: pda_account_data.lamports_per_token,
        tokens_sold: pda_account_data.tokens_sold,
    };

    pda_account_data.owner = buyer.key.to_bytes();
    pda_account_data.tokenization_mint = [0;32];
    pda_account_data.for_sale = 0;
    pda_account_data.number_of_tokens = 0;
    pda_account_data.tokens_sold = 0;
    pda_account_data.price = 0;
    pda_account_data.buy_out_price = 0;
    pda_account_data.lamports_per_token = 0;

    let value: u64 = **pda.lamports.borrow() - terms.nft_pda_account;

    **pda.lamports.borrow_mut()-= value;
    **tokenized_nft_account.lamports.borrow_mut()+= value;

    tokenization_account_data.serialize(&mut &mut tokenized_nft_account.data.borrow_mut()[..])?;
    pda_account_data.serialize(&mut &mut pda.data.borrow_mut()[..])?;
  
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

    if tokenized_nft_token_account.owner != program_id {panic!()}
    if !buyer.is_signer {panic!()}

    if buyer_ata.owner!=&spl_token::id(){panic!()}//token2022 degil
    if tokenized_nft_token_mint.owner!=&spl_token::id(){panic!()}//token2022 degil

    let buyer_ata_unpacked: Account = Account::unpack_from_slice(&buyer_ata.data.borrow())?;
    if tokenized_nft_token_mint.key != &buyer_ata_unpacked.mint {panic!()}
    if &buyer_ata_unpacked.owner != buyer.key{panic!()}

    let mut tokenized_nft_data: NFTToken = NFTToken::try_from_slice(&tokenized_nft_token_account.data.borrow())?;

    let tokenization_mint_key_from_bytes: Pubkey = Pubkey::new_from_array(tokenized_nft_data.tokenized_nft_mint);

    if &tokenization_mint_key_from_bytes != tokenized_nft_token_mint.key{panic!()}

    if buyer_ata_unpacked.amount == 0 {panic!()}

    let total_value: u64 = tokenized_nft_data.lamports_per_token*buyer_ata_unpacked.amount;

    tokenized_nft_data.tokens_sold -= buyer_ata_unpacked.amount;

    if tokenized_nft_data.tokens_sold == 0{panic!()}

    **tokenized_nft_token_account.lamports.borrow_mut()-= total_value;
    **buyer.lamports.borrow_mut()+= total_value;

    let ix = burn(
      token_program.key,
      buyer_ata.key, 
      tokenized_nft_token_mint.key, 
       buyer.key,
       &[buyer.key],
       buyer_ata_unpacked.amount)?;

    let ix2 = close_account(
          token_program.key,
          buyer_ata.key, 
          buyer.key, 
           buyer.key,
           &[buyer.key],
      )?;


    invoke(&ix, &[token_program.clone(),buyer_ata.clone(),tokenized_nft_token_mint.clone(),buyer.clone()])?;
    invoke(&ix2, &[token_program.clone(),buyer_ata.clone(),buyer.clone()])?;


    tokenized_nft_data.serialize(&mut &mut tokenized_nft_token_account.data.borrow_mut()[..])?;


    //sell_nft_as_whole_in_this_program ile acilan hesaba yatirim gonderir.
    //cuzdanina fungible asset gonderilir

    Ok(())
  }


//////////////////////////////////////////////////////////////////////////////////


  fn tokenize_nft_and_sell_in_this_program(
  accounts: &[AccountInfo],
  program_id: &Pubkey,
  create_account:InitAccount,
  data:NFTTerms
) -> ProgramResult {


  let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();

  let seller: &AccountInfo<'_> = next_account_info(accounts_iter)?;
  let seller_ata: &AccountInfo<'_> = next_account_info(accounts_iter)?;
  let pda: &AccountInfo<'_> = next_account_info(accounts_iter)?;
  let pda_ata: &AccountInfo<'_> = next_account_info(accounts_iter)?;
  let nft_mint: &AccountInfo<'_> = next_account_info(accounts_iter)?;
  let tokenized_nft_token_mint: &AccountInfo<'_> = next_account_info(accounts_iter)?;
  let token_program: &AccountInfo<'_> = next_account_info(accounts_iter)?;
  let sysvar: &AccountInfo<'_> = next_account_info(accounts_iter)?;

  if !seller.is_signer{panic!()}
  if seller_ata.owner!=&spl_token::id(){panic!()}//token2022 degil
  if nft_mint.owner!=&spl_token::id(){panic!()}//token2022 degil

  let seller_ata_unpacked: Account = Account::unpack_from_slice(&seller_ata.data.borrow())?;
  if nft_mint.key != &seller_ata_unpacked.mint {panic!()}
  if &seller_ata_unpacked.owner != seller.key{panic!()}

  let mut pda_account_data: NFTTerms = NFTTerms::try_from_slice(&pda.data.borrow())?;

  if pda_account_data.for_sale != 0{panic!()}
  if pda_account_data.tokenized_for_sale != 0{panic!()}

  let nft_from_bytes: Pubkey = Pubkey::new_from_array(pda_account_data.nft_mint);

  if &nft_from_bytes != nft_mint.key {panic!()}

  //creating mint account
  let ix = &system_instruction::create_account(  
    &seller.key, 
    &tokenized_nft_token_mint.key,
    create_account.lamports,
    create_account.size,
    &token_program.key
  );

  //initializing mint for the nft token
  let ix_2 = initialize_mint(
   token_program.key,
   tokenized_nft_token_mint.key,
   pda.key,
   Some(pda.key),
   0)?;


  invoke(ix,  &[seller.clone(),tokenized_nft_token_mint.clone(),token_program.clone(),])?;
  invoke(&ix_2,  &[pda.clone(),tokenized_nft_token_mint.clone(),token_program.clone(),sysvar.clone()])?;


  //transfering nft to program
  let transfer_nft_to_pda_ata =spl_token::instruction::transfer( &token_program.key,
      &seller_ata.key, 
      &pda_ata.key, 
      &seller.key, 
      &[&seller.key], 
      1)?;

  invoke(&transfer_nft_to_pda_ata,&[token_program.clone(),seller_ata.clone(),pda_ata.clone(),seller.clone()])?; 


  let number_of_tokens = data.price/1000000;

  if data.price < 1000000{panic!()}
  if data.buy_out_price < 1000000{panic!()}

  let price: u64 = number_of_tokens*1000000;

  let lamports_per_token: u64 = 1000000;

  let buy_out_token_number: u64 = data.buy_out_price/1000000;

  let buy_out_price: u64 = buy_out_token_number*1000000;


     pda_account_data.owner= seller.key.to_bytes();
     pda_account_data.nft_mint= nft_mint.key.to_bytes();
     pda_account_data.tokenization_mint= tokenized_nft_token_mint.key.to_bytes();
     pda_account_data.for_sale=0;
     pda_account_data.buy_out_allowed=1;
     pda_account_data.owned_by_pda=1;
     pda_account_data.tokenized_for_sale=1;
     pda_account_data.price= price;
     pda_account_data.buy_out_price= buy_out_price;
     pda_account_data.list_in_main_page= "X".to_string();
     pda_account_data.number_of_tokens= number_of_tokens;
     pda_account_data.lamports_per_token = lamports_per_token;
     pda_account_data.tokens_sold= 0;
     pda_account_data.bump=create_account.bump;


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
    let buyer_ata: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let pda: &AccountInfo<'_> = next_account_info(accounts_iter)?;//nft owner in the program
    let nft_mint: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let tokenized_nft_token_mint: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let token_program: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let useradresstokenmint: &AccountInfo<'_> = next_account_info(accounts_iter)?;

    
    if pda.owner != program_id {panic!()}
    if !buyer.is_signer {panic!()}

    if buyer_ata.owner!=&spl_token::id(){panic!()}//token2022 degil
    if tokenized_nft_token_mint.owner!=&spl_token::id(){panic!()}//token2022 degil

    let buyer_ata_unpacked: Account = Account::unpack_from_slice(&buyer_ata.data.borrow())?;
    if tokenized_nft_token_mint.key != &buyer_ata_unpacked.mint {panic!()}
    if &buyer_ata_unpacked.owner != buyer.key{panic!()}

    let mut pda_account_data: NFTTerms = NFTTerms::try_from_slice(&pda.data.borrow())?;

    if pda_account_data.for_sale != 0{panic!()}
    if pda_account_data.tokenized_for_sale != 1{panic!()}

    let seller_key_from_bytes: Pubkey = Pubkey::new_from_array(pda_account_data.owner);
    let tokenization_mint_key_from_bytes: Pubkey = Pubkey::new_from_array(pda_account_data.tokenization_mint);
    let nft_mint_key_from_bytes: Pubkey = Pubkey::new_from_array(pda_account_data.nft_mint);

    if &seller_key_from_bytes != seller.key{panic!()}
    if &tokenization_mint_key_from_bytes != tokenized_nft_token_mint.key{panic!()}
    if &nft_mint_key_from_bytes != nft_mint.key{panic!()}


    let value: u64 = **buyer.lamports.borrow();
    let total_value: u64 = data.lamports*pda_account_data.lamports_per_token;

    if value < total_value{panic!()}

    let tokens_left: u64 = pda_account_data.number_of_tokens - pda_account_data.tokens_sold;
    if tokens_left < data.lamports {panic!()}


    **buyer.lamports.borrow_mut()-= total_value;
    **seller.lamports.borrow_mut()+= total_value;

    let seed: &[u8] = &nft_mint.key.to_bytes();

    let ix = mint_to(
      token_program.key,
      tokenized_nft_token_mint.key, 
      buyer_ata.key, 
       pda.key,//ata_owner???
       &[pda.key],
       data.lamports)?;

      invoke_signed(
        &ix,
        &[
          buyer_ata.clone(),
          tokenized_nft_token_mint.clone(),
          pda.clone(), 
          token_program.clone()
        ],
        &[&[seed, &[pda_account_data.bump]]],
      )?;

      let ix2: &solana_program::instruction::Instruction = &system_instruction::create_account(  
        &buyer.key, 
        &useradresstokenmint.key,
        1000000,
        64,
        &program_id
      );
  
      invoke(&ix2,  &[buyer.clone(),token_program.clone(),useradresstokenmint.clone(),])?;
  

    pda_account_data.tokens_sold += data.lamports;

    //if all required amount is collected transfer ownership to pda
    if pda_account_data.number_of_tokens == pda_account_data.tokens_sold{
      pda_account_data.owner = pda.key.to_bytes();
      pda_account_data.tokenized_for_sale = 0;
    }

    let usertoken = UserAddresTokenMint{
      user:buyer.key.to_bytes(),
      mint:tokenized_nft_token_mint.key.to_bytes()
    };

    pda_account_data.serialize(&mut &mut pda.data.borrow_mut()[..])?;
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
    let owner_nft_ata: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let owner_tokenized_nft_token_ata: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let nft_mint: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let tokenized_nft_token_mint: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let pda: &AccountInfo<'_> = next_account_info(accounts_iter)?;//nft owner in the program
    let pda_nft_ata: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let token_program: &AccountInfo<'_> = next_account_info(accounts_iter)?;

    

    if pda.owner != program_id {panic!()}
    if !owner.is_signer {panic!()}

    if owner_nft_ata.owner!=&spl_token::id(){panic!()}//token2022 degil
    if owner_tokenized_nft_token_ata.owner!=&spl_token::id(){panic!()}
    if nft_mint.owner!=&spl_token::id(){panic!()}
    if pda_nft_ata.owner!=&spl_token::id(){panic!()}
    if tokenized_nft_token_mint.owner!=&spl_token::id(){panic!()}

    let owner_nft_ata_unpacked: Account = Account::unpack_from_slice(&owner_nft_ata.data.borrow())?;
    if nft_mint.key != &owner_nft_ata_unpacked.mint {panic!()}
    if &owner_nft_ata_unpacked.owner != owner.key{panic!()}

    let pda_nft_ata_unpacked: Account = Account::unpack_from_slice(&pda_nft_ata.data.borrow())?;
    if nft_mint.key != &pda_nft_ata_unpacked.mint {panic!()}
    if &pda_nft_ata_unpacked.owner != pda.key{panic!()}

    let owner_tokenized_nft_token_ata_unpacked: Account = Account::unpack_from_slice(&owner_tokenized_nft_token_ata.data.borrow())?;
    if tokenized_nft_token_mint.key != &owner_tokenized_nft_token_ata_unpacked.mint {panic!()}
    if &owner_tokenized_nft_token_ata_unpacked.owner != owner.key{panic!()}

    let mut pda_account_data: NFTTerms = NFTTerms::try_from_slice(&pda.data.borrow())?;

    if pda_account_data.for_sale != 0{panic!()}
    if pda_account_data.tokenized_for_sale != 1{panic!()}

    let owner_key_from_bytes: Pubkey = Pubkey::new_from_array(pda_account_data.owner);
    let tokenization_mint_key_from_bytes: Pubkey = Pubkey::new_from_array(pda_account_data.tokenization_mint);
    let nft_mint_key_from_bytes: Pubkey = Pubkey::new_from_array(pda_account_data.nft_mint);

    if &owner_key_from_bytes != owner.key {panic!()}
    if &tokenization_mint_key_from_bytes != tokenized_nft_token_mint.key {panic!()}
    if &nft_mint_key_from_bytes != nft_mint.key {panic!()}

    let tokens_left = pda_account_data.number_of_tokens - pda_account_data.tokens_sold;


/*
    let transfer_nft_to_owner = transfer( &token_program.key,
      &pda_nft_ata.key, 
      &owner_nft_ata.key, 
      &pda.key, 
      &[&pda.key], 
      1)?;
*/
    let transfer_tokens_to_owner = mint_to( &token_program.key,
        &tokenized_nft_token_mint.key, 
        &owner_tokenized_nft_token_ata.key, 
        &pda.key, 
        &[&pda.key], 
        tokens_left)?;

  let seed: &[u8] = &nft_mint.key.to_bytes();
    
/*
    if pda_account_data.tokens_sold == 0{
      invoke_signed(&transfer_nft_to_owner,       
        &[
        pda_nft_ata.clone(), 
        owner_nft_ata.clone(), 
        pda.clone(), 
        token_program.clone()
      ],
      &[&[seed, &[pda_account_data.bump]]],)?;
      pda_account_data.owner = owner.key.to_bytes();

    }else{*/
      invoke_signed(&transfer_tokens_to_owner, &[
        token_program.clone(),
        owner_tokenized_nft_token_ata.clone(),
        tokenized_nft_token_mint.clone(),
        pda.clone(),
      ],
      &[&[seed, &[pda_account_data.bump]]],)?;
      pda_account_data.owner = pda.key.to_bytes();
    //}

    pda_account_data.tokenized_for_sale = 0;


    pda_account_data.serialize(&mut &mut pda.data.borrow_mut()[..])?;
   
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
    let buyer_ata: &AccountInfo<'_> = next_account_info(accounts_iter)?; 
    let pda: &AccountInfo<'_> = next_account_info(accounts_iter)?;//transfer nft back
    let pda_ata: &AccountInfo<'_> = next_account_info(accounts_iter)?;//nft ata of the pda
    let nft_mint: &AccountInfo<'_> = next_account_info(accounts_iter)?;//check
    let token_program: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let tokenized_nft_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;    
    let terms_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;

    if terms_account.owner != program_id{panic!()}

    let terms: Terms = Terms::try_from_slice(&terms_account.data.borrow())?;
    if terms.is_init != 1{panic!()}

    if pda.owner != program_id {panic!()}
    if !buyer.is_signer {panic!()}

    if buyer_ata.owner!=&spl_token::id(){panic!()}//token2022 degil
    if nft_mint.owner!=&spl_token::id(){panic!()}//token2022 degil

    let buyer_ata_unpacked: Account = Account::unpack_from_slice(&buyer_ata.data.borrow())?;
    if nft_mint.key != &buyer_ata_unpacked.mint {panic!()}
    if &buyer_ata_unpacked.owner != buyer.key{panic!()}

    let pda_ata_unpacked: Account = Account::unpack_from_slice(&pda_ata.data.borrow())?;
    if nft_mint.key != &pda_ata_unpacked.mint {panic!()}
    if &pda_ata_unpacked.owner != pda.key{panic!()}

    let mut pda_account_data: NFTTerms = NFTTerms::try_from_slice(&pda.data.borrow())?;

    if pda_account_data.buy_out_allowed != 1 {panic!()}

    let owner_key_from_bytes: Pubkey = Pubkey::new_from_array(pda_account_data.owner);
    let nft_mint_key_from_bytes: Pubkey = Pubkey::new_from_array(pda_account_data.nft_mint);

    if &owner_key_from_bytes != owner.key{panic!()}
    if &nft_mint_key_from_bytes != nft_mint.key{panic!()}

    let value = **buyer.lamports.borrow();

    if value < pda_account_data.buy_out_price {panic!()}

    **buyer.lamports.borrow_mut()-= pda_account_data.buy_out_price;
    **pda.lamports.borrow_mut()+= pda_account_data.buy_out_price;


    let transfer_nft_to_buyer =spl_token::instruction::transfer( &token_program.key,
      &pda_ata.key, 
      &buyer_ata.key, 
      &pda.key, 
      &[&pda.key], 
      1)?;

    let seed: &[u8] = &nft_mint.key.to_bytes();
    

    invoke_signed(
        &transfer_nft_to_buyer,
        &[
        buyer_ata.clone(),
        pda_ata.clone(),
        pda.clone(), 
        token_program.clone()
      ],
        &[&[seed, &[pda_account_data.bump]]],
    )?;

    let lamports_per_token = pda_account_data.buy_out_price/pda_account_data.number_of_tokens;
    //TODO hesaplama uzerine calisilmasi gerek


    let ix: &solana_program::instruction::Instruction = &system_instruction::create_account(  
      &buyer.key, 
      &tokenized_nft_account.key,
      1000000,
      50,
      &program_id
    );

    invoke(&ix,  &[buyer.clone(),token_program.clone(),tokenized_nft_account.clone(),])?;

    let tokenization_account_data: NFTToken = NFTToken{
        tokenized_nft_mint: pda_account_data.tokenization_mint,
        number_of_tokens: pda_account_data.number_of_tokens,
        lamports_per_token,
        tokens_sold: pda_account_data.tokens_sold,
    };

    pda_account_data.owner = buyer.key.to_bytes();
    pda_account_data.tokenization_mint = [0;32];
    pda_account_data.for_sale = 0;
    pda_account_data.number_of_tokens = 0;
    pda_account_data.tokens_sold = 0;
    pda_account_data.price = 0;
    pda_account_data.buy_out_price = 0;
    pda_account_data.lamports_per_token = 0;

    let value: u64 = **pda.lamports.borrow() - terms.nft_pda_account;

    **pda.lamports.borrow_mut()-= value;
    **tokenized_nft_account.lamports.borrow_mut()+= value;

    tokenization_account_data.serialize(&mut &mut tokenized_nft_account.data.borrow_mut()[..])?;
    pda_account_data.serialize(&mut &mut pda.data.borrow_mut()[..])?;
   
    //buy out ucretini odeyip nftyi cuzdanina al

    Ok(())
  }

//////////////////////////////////////////////////////////////////////////////////

fn tokenize_your_nft(
  accounts: &[AccountInfo],
  create_account:InitAccount,
  data:NFTTerms
) -> ProgramResult {


  let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();

  let owner: &AccountInfo<'_> = next_account_info(accounts_iter)?;
  let owner_ata: &AccountInfo<'_> = next_account_info(accounts_iter)?;
  let pda: &AccountInfo<'_> = next_account_info(accounts_iter)?;//nft owner in the program
  let pda_ata: &AccountInfo<'_> = next_account_info(accounts_iter)?;//nft ata of the pda
  let nft_mint: &AccountInfo<'_> = next_account_info(accounts_iter)?;
  let tokenized_nft_token_mint: &AccountInfo<'_> = next_account_info(accounts_iter)?;
  let token_program: &AccountInfo<'_> = next_account_info(accounts_iter)?;
  let sysvar: &AccountInfo<'_> = next_account_info(accounts_iter)?;

  if !owner.is_signer{panic!()}
  if owner_ata.owner!=&spl_token::id(){panic!()}//token2022 degil
  if nft_mint.owner!=&spl_token::id(){panic!()}//token2022 degil

  let seller_ata_unpacked: Account = Account::unpack_from_slice(&owner_ata.data.borrow())?;
  if nft_mint.key != &seller_ata_unpacked.mint {panic!()}
  if &seller_ata_unpacked.owner != owner.key{panic!()}

  let mut pda_account_data: NFTTerms = NFTTerms::try_from_slice(&pda.data.borrow())?;

  if pda_account_data.for_sale != 0{panic!()}
  if pda_account_data.tokenized_for_sale != 0{panic!()}

  let nft_from_bytes: Pubkey = Pubkey::new_from_array(pda_account_data.nft_mint);

  if &nft_from_bytes != nft_mint.key {panic!()}

  //creating mint account
  let ix = &system_instruction::create_account(  
    &owner.key, 
    &tokenized_nft_token_mint.key,
    create_account.lamports,
    create_account.size,
    &token_program.key
  );

  //minting tokens to nft owner
  let ix_2 = initialize_mint(
   token_program.key,
   tokenized_nft_token_mint.key,
   pda.key,
   Some(pda.key),
   0)?;

   let ix_3 = create_associated_token_account(
    owner.key,
    owner.key,
    tokenized_nft_token_mint.key,
    token_program.key,
    );

    let ix_4 = mint_to(
      token_program.key,
      tokenized_nft_token_mint.key,
      owner_ata.key,
      pda.key,
      &[&pda.key],
      data.number_of_tokens
      )?;

  invoke(ix,  &[owner.clone(),tokenized_nft_token_mint.clone(),token_program.clone(),])?;
  invoke(&ix_2,  &[pda.clone(),tokenized_nft_token_mint.clone(),token_program.clone(),sysvar.clone()])?;
  invoke(&ix_3,  &[owner.clone(),tokenized_nft_token_mint.clone(),token_program.clone(),sysvar.clone(),owner_ata.clone()])?;
  invoke(&ix_4,  &[pda.clone(),tokenized_nft_token_mint.clone(),token_program.clone(),sysvar.clone(),owner_ata.clone()])?;



  //transfering nft to program
  let transfer_nft_to_pda_ata =spl_token::instruction::transfer( &token_program.key,
      &owner_ata.key, 
      &pda_ata.key, 
      &owner.key, 
      &[&owner.key], 
      1)?;

  invoke(&transfer_nft_to_pda_ata,&[token_program.clone(),owner_ata.clone(),pda_ata.clone(),owner.clone()])?; 


    
  let number_of_tokens = data.price/1000000;

  if data.price < 1000000{panic!()}
  if data.buy_out_price < 1000000{panic!()}

  let price: u64 = number_of_tokens*1000000;

  let lamports_per_token: u64 = 1000000;

  let buy_out_token_number: u64 = data.buy_out_price/1000000;

  let buy_out_price: u64 = buy_out_token_number*1000000;


     pda_account_data.owner= owner.key.to_bytes();
     pda_account_data.nft_mint= nft_mint.key.to_bytes();
     pda_account_data.tokenization_mint= tokenized_nft_token_mint.key.to_bytes();
     pda_account_data.for_sale=0;
     pda_account_data.buy_out_allowed=1;
     pda_account_data.owned_by_pda=1;
     pda_account_data.tokenized_for_sale=0;
     pda_account_data.price= price;
     pda_account_data.buy_out_price= buy_out_price;
     pda_account_data.list_in_main_page= "X".to_string();
     pda_account_data.number_of_tokens= number_of_tokens;
     pda_account_data.lamports_per_token = lamports_per_token;
     pda_account_data.tokens_sold= 0;
     pda_account_data.bump=create_account.bump;



     pda_account_data.serialize(&mut &mut pda.data.borrow_mut()[..])?;

  //nft saticisi nftsini program gonderip tokenlara

  Ok(())
}

//////////////////////////////////////////////////////////////////////////////////

  fn init_voting_to_set_new_buy_out_price(
    accounts: &[AccountInfo],
    program_id: &Pubkey,
    data:InitVoting) -> ProgramResult {


    let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();

    let proposer: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let proposer_ata: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let proposer_pda: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let vote_account_pda: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let pda: &AccountInfo<'_> = next_account_info(accounts_iter)?;


    if !proposer.is_signer{panic!()}

    let mut pda_account_data: NFTTerms = NFTTerms::try_from_slice(&pda.data.borrow())?;

    let tokenization_mint_key_from_bytes: Pubkey = Pubkey::new_from_array(pda_account_data.tokenization_mint);

    if proposer_ata.owner!=&spl_token::id(){panic!()}
    if pda_account_data.for_sale != 0{panic!()}
    if pda_account_data.tokenized_for_sale != 0{panic!()}
    if pda_account_data.vote_open != 0{panic!()}

    let owner: Pubkey = Pubkey::new_from_array(pda_account_data.owner);
    if &owner != pda.key {panic!()}

    let proposer_ata_unpacked: Account = Account::unpack_from_slice(&proposer_ata.data.borrow())?;
    if tokenization_mint_key_from_bytes != proposer_ata_unpacked.mint {panic!()}
    if &proposer_ata_unpacked.owner != proposer.key{panic!()}

    let seed:&[u8]= &proposer.key.to_bytes();
    let seed2:&[u8]= &pda_account_data.tokenization_mint;

    let derived_pda: Pubkey = Pubkey::create_program_address(&[seed,seed2, &[data.proposer_pda]], program_id)?;
    if &derived_pda != proposer_pda.key{panic!()}


    invoke_signed(
      &system_instruction::create_account(  
          &proposer.key, 
          &vote_account_pda.key,
          1000000,
          65,
          &program_id
      ),
      &[
        proposer.clone(),
        vote_account_pda.clone(), 
      ],
      &[&[seed, &[data.vote_account_pda]]],
    )?;

    let clock: Clock= Clock::get()?;
    let current_time: u64 = clock.unix_timestamp as u64;

    let vote: VoteAccount = VoteAccount{
       tokenized_nft_mint: pda_account_data.nft_mint, 
       new_buy_out_price_accept_votes: proposer_ata_unpacked.amount, 
       new_buy_out_price_refuse_votes: 0, 
       voting_ends: current_time + 86400, 
       new_buy_out_offer: data.offer, 
       voting_no:1
      };

    let voter_account: InitPDA = InitPDA{
        init_pda: 1,
    };

    pda_account_data.vote_open = 1;
  
    vote.serialize(&mut &mut vote_account_pda.data.borrow_mut()[..])?;
    voter_account.serialize(&mut &mut proposer_pda.data.borrow_mut()[..])?;
    pda_account_data.serialize(&mut &mut pda.data.borrow_mut()[..])?;

    //nft icin yeni bir satis fiyati belirlemek icin oylama baslatir


    Ok(())
  }

  fn repeat_voting_to_set_new_buy_out_price(
    accounts: &[AccountInfo],
    program_id: &Pubkey,
    data:InitVoting) -> ProgramResult {

      let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();

      let proposer: &AccountInfo<'_> = next_account_info(accounts_iter)?;
      let propser_ata: &AccountInfo<'_> = next_account_info(accounts_iter)?;
      let proposer_pda: &AccountInfo<'_> = next_account_info(accounts_iter)?;
      let vote_account_pda: &AccountInfo<'_> = next_account_info(accounts_iter)?;
      let pda: &AccountInfo<'_> = next_account_info(accounts_iter)?;
  
  
      if !proposer.is_signer{panic!()}
      if vote_account_pda.owner != program_id{panic!()}
      if proposer_pda.owner != program_id{panic!()}
      if pda.owner != program_id{panic!()}
  
      if propser_ata.owner!=&spl_token::id(){panic!()}

      let mut votes: VoteAccount = VoteAccount::try_from_slice(&vote_account_pda.data.borrow())?;
      let mut proposer_account: InitPDA = InitPDA::try_from_slice(&proposer_pda.data.borrow())?;
      let mut pda_account_data: NFTTerms = NFTTerms::try_from_slice(&pda.data.borrow())?;

      let owner: Pubkey = Pubkey::new_from_array(pda_account_data.owner);
      if &owner != pda.key {panic!()}

      if pda_account_data.for_sale != 0{panic!()}
      if pda_account_data.tokenized_for_sale != 0{panic!()}
      if pda_account_data.vote_open != 0{panic!()}

      let tokenization_mint_key_from_bytes: Pubkey = Pubkey::new_from_array(votes.tokenized_nft_mint);
      let tokenization_mint_key_from_bytes_2: Pubkey = Pubkey::new_from_array(pda_account_data.tokenization_mint);

      let proposer_ata_unpacked: Account = Account::unpack_from_slice(&propser_ata.data.borrow())?;
      if tokenization_mint_key_from_bytes != proposer_ata_unpacked.mint {panic!()}
      if tokenization_mint_key_from_bytes != tokenization_mint_key_from_bytes_2 {panic!()}
      if &proposer_ata_unpacked.owner != proposer.key{panic!()}

      if proposer_account.init_pda >= votes.voting_no{panic!()}

      let clock: Clock= Clock::get()?;
      let current_time: u64 = clock.unix_timestamp as u64;

      if votes.voting_ends > current_time {panic!()}
  
      let seed:&[u8]= &proposer.key.to_bytes();
      let seed2:&[u8]= &votes.tokenized_nft_mint;
  
  
      let derived_pda: Pubkey = Pubkey::create_program_address(&[seed,seed2, &[data.proposer_pda]], program_id)?;
      if &derived_pda != proposer_pda.key{panic!()}


      votes.new_buy_out_offer = data.offer;
      votes.voting_no += 1;
      votes.voting_ends = current_time + 86400;
      votes.new_buy_out_price_accept_votes += proposer_ata_unpacked.amount;
      votes.new_buy_out_price_refuse_votes += 0;
      proposer_account.init_pda = votes.voting_no;
      pda_account_data.vote_open =1;
      
    
      votes.serialize(&mut &mut vote_account_pda.data.borrow_mut()[..])?;
      proposer_account.serialize(&mut &mut proposer_pda.data.borrow_mut()[..])?;
      pda_account_data.serialize(&mut &mut pda.data.borrow_mut()[..])?;

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
  
      if voter_ata.owner!=&spl_token::id(){panic!()}

      let mut votes: VoteAccount = VoteAccount::try_from_slice(&vote_account_pda.data.borrow())?;
      let mut voter_account: InitPDA = InitPDA::try_from_slice(&voter_pda.data.borrow())?;

      let tokenization_mint_key_from_bytes: Pubkey = Pubkey::new_from_array(votes.tokenized_nft_mint);

      let voter_ata_unpacked: Account = Account::unpack_from_slice(&voter_ata.data.borrow())?;
      if tokenization_mint_key_from_bytes != voter_ata_unpacked.mint {panic!()}
      if &voter_ata_unpacked.owner != voter.key{panic!()}

      if voter_account.init_pda >= votes.voting_no{panic!()}



      let clock: Clock= Clock::get()?;
      let current_time: u64 = clock.unix_timestamp as u64;

      if votes.voting_ends < current_time {panic!()}
  
      let seed:&[u8]= &voter.key.to_bytes();
      let seed2:&[u8]= &votes.tokenized_nft_mint;
  
  
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
      let pda: &AccountInfo<'_> = next_account_info(accounts_iter)?;
  
  
      if vote_account_pda.owner != program_id{panic!()}
      if pda.owner != program_id{panic!()}

      let mut votes: VoteAccount = VoteAccount::try_from_slice(&vote_account_pda.data.borrow())?;
      let mut pda_account_data: NFTTerms = NFTTerms::try_from_slice(&pda.data.borrow())?;
      let owner: Pubkey = Pubkey::new_from_array(pda_account_data.owner);

      if pda_account_data.for_sale != 0{panic!()}
      if pda_account_data.tokenized_for_sale != 0{panic!()}
      if &owner != pda.key {panic!()}
      if pda_account_data.vote_open != 1 {panic!()}


      let tokenization_mint_key_from_bytes: Pubkey = Pubkey::new_from_array(votes.tokenized_nft_mint);
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
      pda_account_data.serialize(&mut &mut pda.data.borrow_mut()[..])?;

    //nftnin likide olmasi icin oylama baslatir


    Ok(())
  }

  fn init_voter_account(
    accounts: &[AccountInfo],
    program_id: &Pubkey,
    data:InitVoting) -> ProgramResult {

      let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();

      let voter: &AccountInfo<'_> = next_account_info(accounts_iter)?;
      let voter_ata: &AccountInfo<'_> = next_account_info(accounts_iter)?;
      let voter_pda: &AccountInfo<'_> = next_account_info(accounts_iter)?;

      if !voter.is_signer{panic!()}
  
      if voter_ata.owner!=&spl_token::id(){panic!()}


      let voter_ata_unpacked: Account = Account::unpack_from_slice(&voter_ata.data.borrow())?;
      if &voter_ata_unpacked.owner != voter.key{panic!()}

  
      let seed:&[u8]= &voter.key.to_bytes();
      let seed2:&[u8]= &voter_ata_unpacked.mint.to_bytes();
  
      invoke_signed(
        &system_instruction::create_account(  
            &voter.key, 
            &voter_pda.key,
            1000000,
            1,
            &program_id
        ),
        &[
          voter.clone(),
          voter_pda.clone(), 
        ],
        &[&[seed,seed2, &[data.proposer_pda]]],
      )?;
    
    //likide oylamasinda oy kullanir
 
    Ok(())
  }

  fn sell_nft_owned_by_program_to_investor(
    accounts: &[AccountInfo],
    program_id: &Pubkey) -> ProgramResult {


      let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();

      let proposer: &AccountInfo<'_> = next_account_info(accounts_iter)?;
      let proposer_ata: &AccountInfo<'_> = next_account_info(accounts_iter)?;
      let proposer_investor_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;
      let proposal_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;
      let pda: &AccountInfo<'_> = next_account_info(accounts_iter)?;//transfer nft back
      let pda_ata: &AccountInfo<'_> = next_account_info(accounts_iter)?;//nft ata of the pda
      let nft_mint: &AccountInfo<'_> = next_account_info(accounts_iter)?;//check
      let token_program: &AccountInfo<'_> = next_account_info(accounts_iter)?;
      let tokenized_nft_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;
  
      if pda.owner != program_id {panic!()}
      if proposer_investor_account.owner != program_id {panic!()}
      if proposal_account.owner != program_id {panic!()}


      if proposer_ata.owner!=&spl_token::id(){panic!()}//token2022 degil
      if nft_mint.owner!=&spl_token::id(){panic!()}//token2022 degil
  
      let proposer_ata_unpacked: Account = Account::unpack_from_slice(&proposer_ata.data.borrow())?;
      if nft_mint.key != &proposer_ata_unpacked.mint {panic!()}
      if &proposer_ata_unpacked.owner != proposer.key{panic!()}
  
      let pda_ata_unpacked: Account = Account::unpack_from_slice(&pda_ata.data.borrow())?;
      if nft_mint.key != &pda_ata_unpacked.mint {panic!()}
      if &pda_ata_unpacked.owner != pda.key{panic!()}
  
      let mut pda_account_data: NFTTerms = NFTTerms::try_from_slice(&pda.data.borrow())?;
      let mut investor_acc: InvestorAccount = InvestorAccount::try_from_slice(&proposer_investor_account.data.borrow())?;
      let proposal: Proposal = Proposal::try_from_slice(&proposal_account.data.borrow())?;
  
  
      let owner_key_from_bytes: Pubkey = Pubkey::new_from_array(pda_account_data.owner);
      let nft_mint_key_from_bytes: Pubkey = Pubkey::new_from_array(pda_account_data.nft_mint);
      let nft_mint_key_from_bytes2: Pubkey = Pubkey::new_from_array(proposal.nft_mint);
      let proposer_key_from_bytes: Pubkey = Pubkey::new_from_array(investor_acc.investor);
      let proposer_key_from_bytes2: Pubkey = Pubkey::new_from_array(proposal.proposer);
  
      if &owner_key_from_bytes != pda.key{panic!()}
      if &nft_mint_key_from_bytes != nft_mint.key{panic!()}
      if &nft_mint_key_from_bytes2 != nft_mint.key{panic!()}
      if &proposer_key_from_bytes != proposer.key{panic!()}
      if &proposer_key_from_bytes2 != proposer.key{panic!()}

      if proposal.offer < pda_account_data.buy_out_price{panic!()}
      if proposal.offer > investor_acc.lamports{panic!()}

      let transfer_nft_to_buyer=spl_token::instruction::transfer( &token_program.key,
        &pda_ata.key, 
        &proposer_ata.key, 
        &pda.key, 
        &[&pda.key], 
        1)?;
  
      let seed: &[u8] = &nft_mint.key.to_bytes();
  
      invoke_signed(
          &transfer_nft_to_buyer,
          &[
            proposer_ata.clone(),
          pda_ata.clone(),
          pda.clone(), 
          token_program.clone()
        ],
          &[&[seed, &[pda_account_data.bump]]],
      )?;
  
      let lamports_per_token = pda_account_data.buy_out_price/pda_account_data.number_of_tokens;
      //TODO hesaplama uzerine calisilmasi gerek
  
      let ix: &solana_program::instruction::Instruction = &system_instruction::create_account(  
        &proposer.key, 
        &tokenized_nft_account.key,
        1000000,
        50,
        &program_id
      );
  
      invoke(&ix,  &[proposer.clone(),token_program.clone(),tokenized_nft_account.clone(),])?;
  
      let tokenization_account_data: NFTToken = NFTToken{
          tokenized_nft_mint: pda_account_data.tokenization_mint,
          number_of_tokens: pda_account_data.number_of_tokens,
          lamports_per_token,
          tokens_sold: pda_account_data.tokens_sold,
      };
  
      pda_account_data.owner = proposer.key.to_bytes();
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
      pda_account_data.serialize(&mut &mut pda.data.borrow_mut()[..])?;
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

      if proposer_ata.owner!=&spl_token::id(){panic!()}//token2022 degil
      if nft_mint.owner!=&spl_token::id(){panic!()}//token2022 degil
  
      let proposer_ata_unpacked: Account = Account::unpack_from_slice(&proposer_ata.data.borrow())?;
      if nft_mint.key != &proposer_ata_unpacked.mint {panic!()}
      if &proposer_ata_unpacked.owner != proposer.key{panic!()}
  
      let seller_ata_unpacked: Account = Account::unpack_from_slice(&seller_ata.data.borrow())?;
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


      let transfer_nft_to_proposer=spl_token::instruction::transfer( &token_program.key,
        &seller_ata.key, 
        &proposer_ata.key, 
        &proposer.key, 
        &[&proposer.key], 
        1)?;
  
  
      invoke(
          &transfer_nft_to_proposer,
          &[
            proposer_ata.clone(),
            seller_ata.clone(),
            proposer.clone(), 
          token_program.clone()
        ],
      )?;

      investor_acc.lamports -= proposal.offer;


      **proposer_investor_account.lamports.borrow_mut()-= proposal.offer;
      **seller.lamports.borrow_mut()+= proposal.offer;

      investor_acc.serialize(&mut &mut proposer_investor_account.data.borrow_mut()[..])?;

    Ok(())
  }


////////////////////////////////////////////////////////////////////////////////////

fn make_an_offer_for_nft(
  accounts: &[AccountInfo],
  program_id: &Pubkey,
  data:Proposal
) -> ProgramResult {


  let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();

  let proposer: &AccountInfo<'_> = next_account_info(accounts_iter)?;
  let proposer_investor_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;
  let proposal_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;
  let nft_mint: &AccountInfo<'_> = next_account_info(accounts_iter)?;

  if !proposer.is_signer{panic!()}

  let investor_acc: InvestorAccount = InvestorAccount::try_from_slice(&proposer_investor_account.data.borrow())?;


  if investor_acc.lamports < data.offer{panic!()}

  let ix: &solana_program::instruction::Instruction = &system_instruction::create_account(  
    &proposer.key, 
    &proposal_account.key,
    data.offer,
    72,
    &program_id
  );

  let proposal = Proposal{
    proposer:proposer.key.to_bytes(),
    nft_mint:nft_mint.key.to_bytes(),
    offer:data.offer
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
    data:InvestorAccount) -> ProgramResult {


    let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();

    let investor: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let investor_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;

    if !investor.is_signer{panic!()}


    if **investor.lamports.borrow() < data.lamports{panic!()}

    let ix: &solana_program::instruction::Instruction = &system_instruction::create_account(  
      &investor.key, 
      &investor_account.key,
      data.lamports,
      40,
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
    data:InvestorAccount) -> ProgramResult {

    let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();

    let investor: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let investor_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;

    if !investor.is_signer{panic!()}
    if investor_account.owner != program_id{panic!()}

    let mut investor_acc: InvestorAccount = InvestorAccount::try_from_slice(&investor_account.data.borrow())?;

    if **investor.lamports.borrow() < data.lamports{panic!()}
    
    **investor.lamports.borrow_mut()-= data.lamports;
    **investor_account.lamports.borrow_mut()+= data.lamports;

    let inverstor_address_from_bytes: Pubkey = Pubkey::new_from_array(investor_acc.investor);

    if &inverstor_address_from_bytes != investor.key {panic!()}
    
    investor_acc.lamports = data.lamports;

    investor_acc.serialize(&mut &mut investor_account.data.borrow_mut()[..])?;


    Ok(())
  }

  //fn create_metadata_for_tokenized_nft_tokens(    accounts: &[AccountInfo],    program_id: &Pubkey  ){}
  fn create_metadata(
      accounts: &[AccountInfo],
      program_id: &Pubkey,
      data:InitPDA
  ) -> ProgramResult {
  /* 
      let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();
  
      let nft_metadata_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;
      let tokenized_nft_ata: &AccountInfo<'_> = next_account_info(accounts_iter)?;
      let tokenized_nft_token_metadata_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;
  
      deserialize nft metadata. check mint in the metadata. create metadata for fungible assets(tokens of the tokenized nft)
  
      let metadata: DataV2= DataV2{
        name: data.name,
        symbol: data.symbol,
        uri: data.uri,
        seller_fee_basis_points: 0,
        creators: None,
        collection: None,
        uses: None,
      };
  
      let args = instructions::CreateMetadataAccountV3InstructionArgs { 
        data: metadata,
        is_mutable: true,
        collection_details: None};
  
      let accounts = instructions::CreateMetadataAccountV3CpiAccounts{    
      metadata: &metadata_account,
      mint: &mint,
      mint_authority: &pda,
      payer: &player,
      update_authority: (pda,true),
      system_program: &system_program,
      rent: None,};
  
      let  ix = instructions::CreateMetadataAccountV3Cpi::new(metaplex_program_id,accounts,args);
  
      ix.invoke_signed(&[&[seed.as_bytes(), &[create_account.bump]]])?;
  */
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
    let pda_ata: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let nft_mint: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let token_program: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let terms_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;

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

    //creating pda_ata
    let create_pda_ata: solana_program::instruction::Instruction = create_associated_token_account(
      initializer.key,
      pda.key, 
      nft_mint.key, 
      token_program.key);

    invoke(&create_pda_ata,  &[initializer.clone(),pda.clone(),pda_ata.clone(),nft_mint.clone(),])?;

    let nft_terms: NFTTerms = NFTTerms{
      
      owner: [0;32],
      nft_mint: nft_mint.key.to_bytes(),
      tokenization_mint: [0;32],
      for_sale:0,
      buy_out_allowed:0,
      owned_by_pda:0,
      tokenized_for_sale:0,
      price: 0, 
      buy_out_price: 0, 
      list_in_main_page: "X".to_string(), 
      number_of_tokens: 0, 
      lamports_per_token:0,
      tokens_sold: 0,
      bump:init.init_pda,
      vote_open:0,

     };

     nft_terms.serialize(&mut &mut pda.data.borrow_mut()[..])?;


    Ok(())
  }

}

/*
  fn join_fund_raising_for_sale_of_an_nft_as_whole_from_this_program(
    accounts: &[AccountInfo],
    program_id: &Pubkey,
    data: Lamports
  ) -> ProgramResult {

    let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();

    let seller: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let buyer: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let buyer_ata: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let pda: &AccountInfo<'_> = next_account_info(accounts_iter)?;//nft owner in the program
    let tokenized_nft_token_mint: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let nft_mint: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let token_program: &AccountInfo<'_> = next_account_info(accounts_iter)?;

    

    if pda.owner != program_id {panic!()}
    if !buyer.is_signer {panic!()}

    if buyer_ata.owner!=&spl_token::id(){panic!()}//token2022 degil
    if tokenized_nft_token_mint.owner!=&spl_token::id(){panic!()}//token2022 degil
    if nft_mint.owner!=&spl_token::id(){panic!()}//token2022 degil

    let buyer_ata_unpacked: Account = Account::unpack_from_slice(&buyer_ata.data.borrow())?;
    if tokenized_nft_token_mint.key != &buyer_ata_unpacked.mint {panic!()}
    if &buyer_ata_unpacked.owner != buyer.key{panic!()}

    let mut pda_account_data: NFTTerms = NFTTerms::try_from_slice(&pda.data.borrow())?;

    if pda_account_data.for_sale != 1{panic!()}
    if pda_account_data.tokenized_for_sale != 0{panic!()}

    let seller_key_from_bytes: Pubkey = Pubkey::new_from_array(pda_account_data.owner);
    let tokenization_mint_key_from_bytes: Pubkey = Pubkey::new_from_array(pda_account_data.tokenization_mint);
    let nft_mint_from_bytes: Pubkey = Pubkey::new_from_array(pda_account_data.nft_mint);

    if &seller_key_from_bytes != seller.key{panic!()}
    if &tokenization_mint_key_from_bytes != tokenized_nft_token_mint.key{panic!()}
    if &nft_mint_from_bytes != nft_mint.key{panic!()}


    let value = **buyer.lamports.borrow();
    let total_value = data.lamports*pda_account_data.lamports_per_token;

    if value < total_value{panic!()}

    let tokens_left = pda_account_data.number_of_tokens - pda_account_data.tokens_sold;
    if tokens_left < data.lamports {panic!()}


    **buyer.lamports.borrow_mut()-= total_value;
    **pda.lamports.borrow_mut()+= total_value;


    let ix = mint_to(
      token_program.key,
      tokenized_nft_token_mint.key, 
      buyer_ata.key, 
       pda.key,//ata_owner???
       &[pda.key],
       data.lamports)?;

       let seed: &[u8] = &nft_mint.key.to_bytes();


      invoke_signed(
        &ix,
        &[
          buyer_ata.clone(),
          tokenized_nft_token_mint.clone(),
          pda.clone(), 
          token_program.clone()
        ],
        &[&[seed, &[pda_account_data.bump]]],
      )?;

    pda_account_data.tokens_sold += data.lamports;

    //if all required amount is collected transfer the lamports to seller and ownership to pda
    if pda_account_data.number_of_tokens == pda_account_data.tokens_sold{

      **pda.lamports.borrow_mut()-= pda_account_data.price;
      **seller.lamports.borrow_mut()+= pda_account_data.price;

      pda_account_data.owner = pda.key.to_bytes();
      pda_account_data.for_sale = 0;

    }

    pda_account_data.serialize(&mut &mut pda.data.borrow_mut()[..])?;


    //sell_nft_as_whole_in_this_program ile acilan hesaba yatirim gonderir.
    //cuzdanina fungible asset gonderilir

    Ok(())
  }

*/

/*
  fn return_profits_from_sold_out_nft(
    accounts: &[AccountInfo],
    program_id: &Pubkey) -> ProgramResult {


      let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();

      let buyer: &AccountInfo<'_> = next_account_info(accounts_iter)?;
      let buyer_ata: &AccountInfo<'_> = next_account_info(accounts_iter)?;
      let tokenized_nft_token_mint: &AccountInfo<'_> = next_account_info(accounts_iter)?;
      let tokenized_nft_token_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;
      let token_program: &AccountInfo<'_> = next_account_info(accounts_iter)?;
  
      if tokenized_nft_token_account.owner != program_id {panic!()}
      if !buyer.is_signer {panic!()}
  
      if buyer_ata.owner!=&spl_token::id(){panic!()}//token2022 degil
      if tokenized_nft_token_mint.owner!=&spl_token::id(){panic!()}//token2022 degil
  
      let buyer_ata_unpacked: Account = Account::unpack_from_slice(&buyer_ata.data.borrow())?;
      if tokenized_nft_token_mint.key != &buyer_ata_unpacked.mint {panic!()}
      if &buyer_ata_unpacked.owner != buyer.key{panic!()}
  
      let mut tokenized_nft_data: NFTToken = NFTToken::try_from_slice(&tokenized_nft_token_account.data.borrow())?;
  
      let tokenization_mint_key_from_bytes: Pubkey = Pubkey::new_from_array(tokenized_nft_data.tokenized_nft_mint);
  
      if &tokenization_mint_key_from_bytes != tokenized_nft_token_mint.key{panic!()}
  
      if buyer_ata_unpacked.amount == 0 {panic!()}
  
      let total_value: u64 = tokenized_nft_data.lamports_per_token*buyer_ata_unpacked.amount;
  
      tokenized_nft_data.tokens_sold -= buyer_ata_unpacked.amount;
  
      if tokenized_nft_data.tokens_sold == 0{panic!()}
  
      **tokenized_nft_token_account.lamports.borrow_mut()-= total_value;
      **buyer.lamports.borrow_mut()+= total_value;
  
      let ix = burn(
        token_program.key,
        buyer_ata.key, 
        tokenized_nft_token_mint.key, 
         buyer.key,
         &[buyer.key],
         buyer_ata_unpacked.amount)?;
  
      let ix2 = close_account(
            token_program.key,
            buyer_ata.key, 
            buyer.key, 
             buyer.key,
             &[buyer.key],
        )?;
  
  
      invoke(&ix, &[token_program.clone(),buyer_ata.clone(),tokenized_nft_token_mint.clone(),buyer.clone()])?;
      invoke(&ix2, &[token_program.clone(),buyer_ata.clone(),buyer.clone()])?;
  
  
      tokenized_nft_data.serialize(&mut &mut tokenized_nft_token_account.data.borrow_mut()[..])?;
   
    //nft pesin para verilip satin alinmis ise hisse sahiplari tokenlarini bozdurup karlarini alirlar

    Ok(())
  }
*/

use crate::{check, service::create_nft_transfer_instruction, state::{ InvestorAccount, Lamports, NFTState, Proposal, Terms, TokenToSol }};
use borsh::{BorshDeserialize, BorshSerialize};


use spl_associated_token_account::instruction::create_associated_token_account;

use solana_program::{
  account_info::{next_account_info, AccountInfo},
  entrypoint::ProgramResult,
  pubkey::Pubkey,
  system_instruction,
  program::{invoke_signed,invoke},
};

use check::check_mint_and_owner;


  pub fn  list_nft_forsale_as_whole_in_this_program(
    accounts: &[AccountInfo],
    program_id: &Pubkey,
    data: NFTState,
    ) -> ProgramResult {

      //nft saticisi pazar yerimizde nftsini satisa cikarir.
      //pesin odeme talep eder. Birisi pesin odeme ile yada bir fon toplama girisimi alabilir 

    let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();

    let seller: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let seller_ata: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let registered_nft_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;//nft owner in the program
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

    if registered_nft_account.owner != program_id{panic!()}

    check_mint_and_owner(nft_mint.key,seller.key, seller_ata);

    let mut pda_account_data: NFTState = NFTState::try_from_slice(&registered_nft_account.data.borrow())?;

    if pda_account_data.tokenized_for_sale != 0{panic!()}
    if pda_account_data.for_sale != 0{panic!()}

    let nft_from_bytes: Pubkey = Pubkey::new_from_array(pda_account_data.nft_mint);

    if &nft_from_bytes != nft_mint.key {panic!()}


    let transfer_nft_to_registered_nft_account_ata = create_nft_transfer_instruction(token_program.key, seller_ata.key, nft_mint.key, registered_nft_account_ata.key, seller.key);

    invoke(&transfer_nft_to_registered_nft_account_ata,&[token_program.clone(),seller_ata.clone(),registered_nft_account_ata.clone(),seller.clone(),nft_mint.clone()])?; 


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


    pda_account_data.serialize(&mut &mut registered_nft_account.data.borrow_mut()[..])?;

    Ok(())
  }

  pub fn cancel_sale_of_nft_as_whole_in_this_program(
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

    check_mint_and_owner(nft_mint.key, seller.key, seller_ata);

    check_mint_and_owner(nft_mint.key, registered_nft_account.key, registered_nft_account_ata);


    let mut pda_account_data: NFTState = NFTState::try_from_slice(&registered_nft_account.data.borrow())?;

    if pda_account_data.for_sale != 1{panic!()}
    if pda_account_data.tokenized_for_sale != 0{panic!()}

    let seller_key_from_bytes: Pubkey = Pubkey::new_from_array(pda_account_data.owner);
    let nft_mint_key_from_bytes: Pubkey = Pubkey::new_from_array(pda_account_data.nft_mint);

    if &seller_key_from_bytes != seller.key{panic!()}
    if &nft_mint_key_from_bytes != nft_mint.key{panic!()}



    let transfer_nft_to_seller = create_nft_transfer_instruction(token_program.key,registered_nft_account_ata.key,nft_mint.key,seller_ata.key,registered_nft_account.key);


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

  pub fn buy_whole_nft_from_this_program(
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
    
    check_mint_and_owner(nft_mint.key, buyer.key, buyer_ata);
    
    check_mint_and_owner(nft_mint.key, registered_nft_account.key, registered_nft_account_ata);
    

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


    let transfer_nft_to_buyer = create_nft_transfer_instruction(token_program.key, registered_nft_account_ata.key, nft_mint.key, buyer_ata.key, registered_nft_account.key);
    
    let seed: &[u8] = &nft_mint.key.to_bytes();

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

  pub fn sell_nft_owned_by_program_to_investor(
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
  
      check_mint_and_owner(nft_mint.key, proposer.key, proposer_ata);

      check_mint_and_owner(nft_mint.key, registered_nft_account.key, registered_nft_account_ata);

  
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

      let transfer_nft_to_buyer = create_nft_transfer_instruction(token_program.key, registered_nft_account_ata.key, nft_mint.key, proposer_ata.key, registered_nft_account.key);

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

  pub  fn sell_nft_owned_by_individual_to_investor(
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
  
      check_mint_and_owner(nft_mint.key, proposer.key, proposer_ata);

      check_mint_and_owner(nft_mint.key, seller.key, seller_ata);

  
      let mut investor_acc: InvestorAccount = InvestorAccount::try_from_slice(&proposer_investor_account.data.borrow())?;
      let proposal: Proposal = Proposal::try_from_slice(&proposal_account.data.borrow())?;
  
  
      let nft_mint_key_from_bytes2: Pubkey = Pubkey::new_from_array(proposal.nft_mint);
      let proposer_key_from_bytes: Pubkey = Pubkey::new_from_array(investor_acc.investor);
      let proposer_key_from_bytes2: Pubkey = Pubkey::new_from_array(proposal.proposer);
  

      if &nft_mint_key_from_bytes2 != nft_mint.key{panic!()}
      if &proposer_key_from_bytes != proposer.key{panic!()}
      if &proposer_key_from_bytes2 != proposer.key{panic!()}

      if proposal.offer > investor_acc.lamports{panic!()}

  
    let transfer_nft_to_proposer = create_nft_transfer_instruction(token_program.key , seller_ata.key, nft_mint.key, proposer_ata.key, seller.key);

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

  pub fn make_an_offer_for_nft(
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

  pub fn create_investor_account(
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

  pub fn fund_investor_account(
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

  pub fn remove_funds_from_investor_account(
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

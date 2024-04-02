
use crate::{check::check_mint_and_owner, service::create_nft_transfer_instruction, state::{InitPDA, Lamports, NFTState, Terms, TokenToSol, UserAddresTokenMint}};
use borsh::{BorshDeserialize, BorshSerialize};


use solana_program::program_pack::Pack;
use spl_associated_token_account::instruction::create_associated_token_account;

use solana_program::{
  account_info::{next_account_info, AccountInfo},
  entrypoint::ProgramResult,
  pubkey::Pubkey,
  system_instruction,
  program::{invoke_signed,invoke},

};


use spl_token::state::Account;


  pub fn change_tokens_to_sol(
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

  pub  fn tokenize_nft_and_sell_in_this_program(
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
  let dex: &AccountInfo<'_> = next_account_info(accounts_iter)?;
  let dex_ata: &AccountInfo<'_> = next_account_info(accounts_iter)?;


  if terms_account.owner != program_id{panic!()}
  if dex.owner != program_id{panic!()}
  if terms_account.is_writable{panic!()}
  if dex.is_writable{panic!()}

  let terms: Terms = Terms::try_from_slice(&terms_account.data.borrow())?;
  if terms.is_init != 1 {panic!()}

  let dex_data: InitPDA = InitPDA::try_from_slice(&dex.data.borrow())?;

  if dex_data.init_pda != 5 {panic!()}
  

  if pda.owner != program_id {panic!()}
  if seller_ata.owner!=&spl_token::id() && seller_ata.owner!=&spl_token_2022::id(){panic!()}
  if nft_mint.owner!=&spl_token::id() && nft_mint.owner!=&spl_token_2022::id(){panic!()}
  
  check_mint_and_owner(nft_mint.key, seller.key, seller_ata);
  
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

  
  let transfer_nft_to_registered_nft_account_ata = create_nft_transfer_instruction(token_program.key, seller_ata.key, nft_mint.key, registered_nft_account_ata.key, seller.key);
  invoke(&transfer_nft_to_registered_nft_account_ata,&[token_program.clone(),nft_mint.clone(),seller_ata.clone(),registered_nft_account_ata.clone(),seller.clone()])?; 


  let create_dex_ata: solana_program::instruction::Instruction = create_associated_token_account(
    seller.key,
    dex.key, 
    tokenization_mint.key, 
    token_program.key);

  invoke(&create_dex_ata,
      &[seller.clone(),dex_ata.clone(),dex.clone(),tokenization_mint.clone(),token_program.clone(),sysvar.clone()])?;

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

  pub  fn buy_part_of_tokenized_nft_from_this_program(
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

    check_mint_and_owner(tokenization_mint.key, buyer.key, buyer_tokenization_ata);


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

      let create_useraddres_token_mint: &solana_program::instruction::Instruction = &system_instruction::create_account(  
        &buyer.key, 
        &useradresstokenmint.key,
        terms.usertokenmint_account,
        terms.usertokenmint_account_size,
        &program_id
      );

      invoke(&create_useraddres_token_mint,  &[buyer.clone(),useradresstokenmint.clone(),])?;


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

  pub  fn stop_sale_of_tokenized_nft_and_return_tokens(
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
    let useradresstokenmint: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let terms_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;


    if terms_account.owner != program_id{panic!()}
    if registered_nft_account.owner != program_id{panic!()}
  
    if terms_account.is_writable{panic!()}
    let terms: Terms = Terms::try_from_slice(&terms_account.data.borrow())?;
    if terms.is_init != 1 {panic!()}


    if registered_nft_account.owner != program_id {panic!()}
    if !owner.is_signer {panic!()}

    if owner_tokenization_ata.owner!=&spl_token::id() && owner_tokenization_ata.owner!=&spl_token_2022::id(){panic!()}
    if tokenization_mint.owner!=&spl_token::id() && tokenization_mint.owner!=&spl_token_2022::id(){panic!()}

    check_mint_and_owner(tokenization_mint.key, owner.key, owner_tokenization_ata);


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

    let create_useraddres_token_mint: &solana_program::instruction::Instruction = &system_instruction::create_account(  
      &owner.key, 
      &useradresstokenmint.key,
      terms.usertokenmint_account,
      terms.usertokenmint_account_size,
      &program_id
    );
    
    invoke(&create_useraddres_token_mint,  &[owner.clone(),useradresstokenmint.clone(),])?;
    
    let usertoken = UserAddresTokenMint{
      user:owner.key.to_bytes(),
      mint:tokenization_mint.key.to_bytes()
    };

    pda_account_data.owner = registered_nft_account.key.to_bytes();
    pda_account_data.tokenized_for_sale = 0;
    
    usertoken.serialize(&mut &mut useradresstokenmint.data.borrow_mut()[..])?;
    pda_account_data.serialize(&mut &mut registered_nft_account.data.borrow_mut()[..])?;
   
    //nft saticisi tokenize_nft_and_sell_in_this_program fonksiyonu ile satisa cikardigi nftsinin satisini iptal eder
    //bedel token olarak cuzdanina gelir, nft tokenize olarak programda kalir

    Ok(())
  }

  pub fn buy_out_tokenized_nft(
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

    check_mint_and_owner(nft_mint.key, buyer.key, buyer_ata);

    check_mint_and_owner(nft_mint.key, registered_nft_account.key, &registered_nft_account_ata);

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


    let transfer_nft_to_buyer = create_nft_transfer_instruction(token_program.key , registered_nft_account_ata.key, nft_mint.key, buyer_ata.key, registered_nft_account.key);

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

  pub fn tokenize_your_nft(
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
  let useradresstokenmint: &AccountInfo<'_> = next_account_info(accounts_iter)?;

  if terms_account.owner != program_id{panic!()}
  if registered_nft_account.owner != program_id{panic!()}

  if terms_account.is_writable{panic!()}
  let terms: Terms = Terms::try_from_slice(&terms_account.data.borrow())?;
  if terms.is_init != 1 {panic!()}


  if !owner.is_signer{panic!()}
  if owner_ata.owner!=&spl_token::id() && owner_ata.owner!=&spl_token_2022::id(){panic!()}
  if nft_mint.owner!=&spl_token::id() && nft_mint.owner!=&spl_token_2022::id(){panic!()}

  check_mint_and_owner(nft_mint.key, owner.key, &owner_ata);


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

  let create_useraddres_token_mint: &solana_program::instruction::Instruction = &system_instruction::create_account(  
    &owner.key, 
    &useradresstokenmint.key,
    terms.usertokenmint_account,
    terms.usertokenmint_account_size,
    &program_id
  );

  invoke(&create_useraddres_token_mint,  &[owner.clone(),useradresstokenmint.clone(),])?;

  if data.lamports_per_token < terms.minimum_lamports_per_token{panic!()}
  if data.lamports_per_token_buyout < data.lamports_per_token{panic!()}

  if data.lamports_per_token % terms.minimum_lamports_per_token != 0{panic!()}
  if data.lamports_per_token_buyout % terms.minimum_lamports_per_token != 0{panic!()}

  let price = data.number_of_tokens*data.lamports_per_token;
  let buy_out_price = data.number_of_tokens*data.lamports_per_token_buyout;

  registered_nft_account_data.owner= registered_nft_account.key.to_bytes();
  registered_nft_account_data.nft_mint= nft_mint.key.to_bytes();
  registered_nft_account_data.tokenization_mint= tokenization_mint.key.to_bytes();
  registered_nft_account_data.for_sale=0;
  registered_nft_account_data.buy_out_allowed=data.buy_out_allowed;
  registered_nft_account_data.owned_by_pda=1;
  registered_nft_account_data.tokenized_for_sale=0;
  registered_nft_account_data.price= price;
  registered_nft_account_data.buy_out_price= buy_out_price;
  registered_nft_account_data.lamports_per_token_buyout= data.lamports_per_token_buyout;
  registered_nft_account_data.number_of_tokens= data.number_of_tokens;
  registered_nft_account_data.lamports_per_token = data.lamports_per_token;
  registered_nft_account_data.tokens_sold= 0;
  registered_nft_account_data.bump=data.bump;

  let usertoken = UserAddresTokenMint{
    user:owner.key.to_bytes(),
    mint:tokenization_mint.key.to_bytes()
  };


  registered_nft_account_data.serialize(&mut &mut registered_nft_account.data.borrow_mut()[..])?;
  usertoken.serialize(&mut &mut useradresstokenmint.data.borrow_mut()[..])?;

  //nft saticisi nftsini program gonderip tokenlara

  Ok(())
}

//tokenize your nft
//stop_sale_of_tokenized_nft_and_return_tokens

//terms state
//tokenize nft and sell
//start_fund_raising_to_buy_nft
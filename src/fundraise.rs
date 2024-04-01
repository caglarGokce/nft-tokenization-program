


use crate::{check::check_mint_and_owner, service::create_nft_transfer_instruction, state::{DistData, FundRaising, FunderAccount, InitAccount, InitPDA, Lamports, NFTState, Terms, UserAddresTokenMint  }};
use borsh::{BorshDeserialize, BorshSerialize};


use spl_associated_token_account::instruction::create_associated_token_account;

use solana_program::{
  account_info::{next_account_info, AccountInfo}, entrypoint::ProgramResult, msg, program::{invoke, invoke_signed}, pubkey::Pubkey, system_instruction

};

  //NFT satin almak icin fon toplama baslatilir.
  //Ayni NFT icin sadece bir tane fon toplama girisimi olabilir
  //Eger fon toplama girisimi devam ediyorsa ayrica bir girisim baslatilamaz
  //Eger NFT tokenize edilmis ise fon toplama girisimi baslatilamaz. Yatirimcinin mevcut hisse sahiplerinden hisse yani token satin almasi beklenir.
  //Kisacasi sahiplik bir topluluktan otekine gecemez
  //Fon toplama girisimi basarili olursa yatirimci tokenlarini talep eder.
  //yatirimci yatirim ekleyebilir veya yatirimini cekebilir.
  pub fn start_fund_raising_to_buy_nft(
    accounts: &[AccountInfo],
    program_id: &Pubkey,
    create_account: InitAccount,
  ) -> ProgramResult {

    let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();

    let initializer: &AccountInfo<'_> = next_account_info(accounts_iter)?; //fon toplama girisimini baslatan hesap writable signer
    let tokenization_mint: &AccountInfo<'_> = next_account_info(accounts_iter)?; //NFT'nin tokenize olmus halinin adresi - fungible assets
    let nft_mint: &AccountInfo<'_> = next_account_info(accounts_iter)?; //NFT'nin tokenize olmus halinin adresi - fungible assets
    let fundrasing_account: &AccountInfo<'_> = next_account_info(accounts_iter)?; //fon toplama girisim hesabi. her NFT icin bir tane bulunur.
    let token_dist_data: &AccountInfo<'_> = next_account_info(accounts_iter)?; //Fon toplama girisimi basarili olursa yatirimci tokenlarini almak icin bu hesabi kullanir
    let registered_nft_account: &AccountInfo<'_> = next_account_info(accounts_iter)?; //NFT ile ilgili butun bilgilerin tutuldugu hesaptir
    let token_2022_program: &AccountInfo<'_> = next_account_info(accounts_iter)?; 
    let terms_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;//Hesaplarin size ve rent datasi burda
    let dex: &AccountInfo<'_> = next_account_info(accounts_iter)?; 
    let dex_ata: &AccountInfo<'_> = next_account_info(accounts_iter)?; 
    let sysvar: &AccountInfo<'_> = next_account_info(accounts_iter)?;

    if dex.owner != program_id{panic!()}
    if registered_nft_account.owner != program_id{panic!()}
    if terms_account.owner != program_id{panic!()}
    if dex.is_writable{panic!()}
    if terms_account.is_writable{panic!()}

    let registered_nft_account_data: NFTState = NFTState::try_from_slice(&registered_nft_account.data.borrow())?;
    let nft_mint_from_bytes2: Pubkey = Pubkey::new_from_array(registered_nft_account_data.nft_mint);

    if nft_mint.owner != &spl_token::id() && nft_mint.owner != &spl_token_2022::id(){panic!()}

    let dex_data: Terms = Terms::try_from_slice(&dex.data.borrow())?;
    let terms: Terms = Terms::try_from_slice(&terms_account.data.borrow())?;
    if terms.is_init != 1 {panic!()}
    if dex_data.is_init != 5 {panic!()}

    if !initializer.is_signer{panic!()}

    let seed: &[u8] = &nft_mint.key.to_bytes();


    let bump: u8 = Pubkey::find_program_address(&[b"fund",seed], program_id).1;

    if fundrasing_account.owner != program_id{

        invoke_signed(
            &system_instruction::create_account(
              &initializer.key, 
              &fundrasing_account.key,
              terms.fundrasing_account,
                terms.fundrasing_account_size, 
                program_id),
            &[
              initializer.clone(),
              fundrasing_account.clone(),
    
            ],
            &[&[b"fund",seed, &[bump]]],
         )?;
    
    }else{
        let fundraising: FundRaising = FundRaising::try_from_slice(&fundrasing_account.data.borrow())?;
        if fundraising.fund_raising != 0 {panic!()} //if there is an active fundrasing panic
        let nft_mint_from_bytes3: Pubkey = Pubkey::new_from_array(fundraising.nft_mint);
        if nft_mint_from_bytes3 != nft_mint_from_bytes2 {panic!()}
    }

    if registered_nft_account_data.tokenized_for_sale != 0 {panic!()} //if the nft is tokenized cant start a fundrasing. go buy tokens
    if registered_nft_account_data.owned_by_pda != 0 {panic!()} //if nft already owned by a community cant start a funsraise

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

    let create_dex_ata: solana_program::instruction::Instruction = create_associated_token_account(
      initializer.key,
      dex.key, 
      tokenization_mint.key, 
      token_2022_program.key);
  
    invoke(&create_dex_ata,
        &[initializer.clone(),dex_ata.clone(),dex.clone(),tokenization_mint.clone(),token_2022_program.clone(),sysvar.clone()])?;

    let fundraising: FundRaising = FundRaising{ 
        fund_raising:1,
        nft_mint: nft_mint.key.to_bytes(),
        tokens_mint:tokenization_mint.key.to_bytes(),
        funds_collected: 0,
        number_of_tokens: 0,
        lamports_per_token: terms.lamports_per_token_fundraising,
        bump:bump
    };

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

  pub fn join_fund_raising_to_buy_nft(
    accounts: &[AccountInfo],
    program_id: &Pubkey,
    data:Lamports
  ) -> ProgramResult {


    let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();

    let funder: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let funders_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let fundraising_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;//nft owner in the program
    let temp: &AccountInfo<'_> = next_account_info(accounts_iter)?;//nft owner in the program
    let funder_ata: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let tokenization_mint: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let token_program: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let terms_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;

    if !funder.is_signer{panic!()}
    msg!("1");
      let mut fundraising: FundRaising = FundRaising::try_from_slice(&fundraising_account.data.borrow())?;
  
      if fundraising_account.owner != program_id{panic!()}
    msg!("2");
    if fundraising.fund_raising != 1 {panic!()}
    msg!("3");
  

      let nft_mint_from_bytes2: Pubkey = Pubkey::new_from_array(fundraising.nft_mint);
      let tokenization_mint_from_bytes2: Pubkey = Pubkey::new_from_array(fundraising.tokens_mint);
      msg!("{}",tokenization_mint_from_bytes2.to_string());
      msg!("{}",tokenization_mint.key.to_string());
      if &tokenization_mint_from_bytes2 != tokenization_mint.key {panic!()}
      msg!("5",);


      let mut fund_invested:u64 = 0;
  
      if funders_account.owner != program_id{
        msg!("4");

          if terms_account.owner != program_id{panic!()}
          let terms: Terms = Terms::try_from_slice(&terms_account.data.borrow())?;
          if terms.is_init != 1{panic!()}
       msg!("5");
      
          invoke(
            &system_instruction::create_account(  
                &funder.key, 
                &funders_account.key,
                terms.funder_account,
                terms.funder_account_size,
                &program_id
            ),
            &[
              funder.clone(),
              funders_account.clone(), 
            ],
          )?;
        
          let create_funder_ata: solana_program::instruction::Instruction = create_associated_token_account(
            funder.key,
            funder.key, 
            tokenization_mint.key, 
            token_program.key);
      
          invoke(&create_funder_ata,  &[funder.clone(),tokenization_mint.clone(),funder_ata.clone(),token_program.clone()])?;

      }else{
        msg!("9");

          let funders_account_data: FunderAccount = FunderAccount::try_from_slice(&funders_account.data.borrow())?;
    
          let funder_address_from_bytes: Pubkey = Pubkey::new_from_array(funders_account_data.funder);
          if &funder_address_from_bytes != funder.key {panic!()}
           msg!("6");
      
          let nft_mint_from_bytes: Pubkey = Pubkey::new_from_array(funders_account_data.nft_mint);
          if nft_mint_from_bytes != nft_mint_from_bytes2 {panic!()}
          msg!("7");
    
          let tokenization_mint_from_bytes: Pubkey = Pubkey::new_from_array(funders_account_data.tokens_mint);
          if tokenization_mint_from_bytes != tokenization_mint_from_bytes2{panic!()}
           msg!("8");
      
          fund_invested = funders_account_data.fund_invested;
      }

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
    msg!("9");

    fund_invested += fund_raise;
    fundraising.funds_collected += fund_raise;
    fundraising.number_of_tokens += number_of_tokens;

    **temp.lamports.borrow_mut()-= fund_raise;
    **fundraising_account.lamports.borrow_mut()+= fund_raise;

    let funders: FunderAccount = FunderAccount{
      funder:funder.key.to_bytes(),
      nft_mint:fundraising.nft_mint,
      tokens_mint:fundraising.tokens_mint,
      fund_invested,
      lamports_per_token:fundraising.lamports_per_token
    };

    funders.serialize(&mut &mut funders_account.data.borrow_mut()[..])?;
    fundraising.serialize(&mut &mut fundraising_account.data.borrow_mut()[..])?;



    Ok(())
  }

  pub fn remove_funds_from_the_fundraising(
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

  pub fn get_tokenized_assets_from_successfull_fundraising(
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

    check_mint_and_owner(tokenization_mint.key, funder.key, funder_ata);


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

      
      let createuseradrestokenmint: &solana_program::instruction::Instruction = &system_instruction::create_account(  
        &funder.key, 
        &useradresstokenmint.key,
        terms.usertokenmint_account,
        terms.usertokenmint_account_size,
        &program_id
      );

      invoke(&createuseradrestokenmint,  &[funder.clone(),useradresstokenmint.clone(),])?;


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

  pub fn buy_nft_with_the_funds_cpi_to(
    _accounts: &[AccountInfo],
    _program_id: &Pubkey
  ) -> ProgramResult {


    //let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();

    //Cross program invocation to another market place

    Ok(())
  }

  pub fn sell_nft_to_the_fundraising(
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

 
    check_mint_and_owner(nft_mint.key, seller.key, seller_ata);

    check_mint_and_owner(nft_mint.key, registered_nft_account.key, registered_nft_account_ata);

    let nft_mint_from_bytes: Pubkey = Pubkey::new_from_array(fundraising.nft_mint);
    if &nft_mint_from_bytes != nft_mint.key {panic!()}

    let nft_mint_from_bytes2: Pubkey = Pubkey::new_from_array(pda_account.nft_mint);
    if &nft_mint_from_bytes2 != nft_mint.key {panic!()}



    let  transfer_nft_to_registered_nft_account_ata_2022 = create_nft_transfer_instruction(token_program.key, seller_ata.key, nft_mint.key, registered_nft_account_ata.key, seller.key);

    invoke(&transfer_nft_to_registered_nft_account_ata_2022,&[token_program.clone(),seller_ata.clone(),nft_mint.clone(),registered_nft_account_ata.clone(),seller.clone()])?; 


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

  pub fn buy_nft_listed_in_program_with_the_funds(
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


    check_mint_and_owner(nft_mint.key, pda.key, registered_nft_account_ata);

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

  pub fn create_funding_account(
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

  pub fn create_fundraising_account(
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


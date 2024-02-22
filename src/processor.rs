use crate::instruction::GameInstruction;
use crate::state::{};
use borsh::{BorshDeserialize, BorshSerialize};


use spl_associated_token_account::instruction::create_associated_token_account;
use std::str::FromStr;
use solana_program::{
  account_info::{next_account_info, AccountInfo},
  entrypoint::ProgramResult,
  pubkey::Pubkey,
  sysvar::{clock::Clock, Sysvar,},
  system_instruction,
  program::{invoke_signed,invoke},
  program_pack::Pack,
  msg,
};

use spl_token::instruction::{initialize_mint, mint_to, transfer,freeze_account,close_account};


use spl_token::state::Account;

use mpl_token_metadata::{
  instructions,
  types::DataV2
  };

pub struct Processor;
impl Processor {
  pub fn process(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
  ) -> ProgramResult {
    let instruction: GameInstruction = GameInstruction::unpack(instruction_data)?;

    match instruction {
      
      GameInstruction::StartFundRaising {} => {
        Self::start_fund_raising_to_buy_nft_from_another_marketplace(accounts,program_id)
      }

      GameInstruction::JoinFundRaising {} => {
        Self::join_fund_raising_to_buy_nft_from_another_marketplace(accounts,program_id)
      }
      GameInstruction::ReturnAssets {} => {
        Self::return_assets_from_failed_fund_raising_to_buy_nft_from_another_marketplace(accounts,program_id)
      }
      GameInstruction::FailFundRaising {} => {
        Self::fail_fund_raising_to_buy_nft_from_another_marketplace(accounts,program_id)
      }
      GameInstruction::SellWholeNFT {} => {
        Self::sell_nft_as_whole_in_this_program(accounts,program_id)
      }
      GameInstruction::CancelWholeNFTSale {} => {
        Self::cancel_sale_of_nft_as_whole_in_this_program(accounts,program_id)
      }
      GameInstruction::JoinFundRaisingProgram {} => {
        Self::join_fund_raising_for_sale_of_an_nft_as_whole_from_this_program(accounts,program_id)
      }
      GameInstruction::BuyTokenizedNFT {} => {
        Self::buy_part_of_tokenized_nft_from_this_program(accounts,program_id)
      }
      GameInstruction::TokenizeNFT {} => {
        Self::tokenize_nft_and_sell_in_this_program(accounts,program_id)
      }
      GameInstruction::BuyWholeNFTProgram {} => {
        Self::buy_whole_nft_from_this_program(accounts,program_id)
      }
      GameInstruction::StopTokenizedNFTSale {} => {
        Self::stop_sale_of_tokenized_nft_and_return_tokens(accounts,program_id)
      }
      GameInstruction::MakeOffer {} => {
        Self::make_an_offer_for_nft(accounts,program_id)
      }
      GameInstruction::MakeOfferProgram {} => {
        Self::make_an_offer_for_nft_in_this_program(accounts,program_id)
      }
      GameInstruction::StartVoting {} => {
        Self::start_voting_for_liquidation(accounts,program_id)
      }
      GameInstruction::Liquidate{} => {
        Self::liquidate_nft(accounts,program_id)
      }
      GameInstruction::Vote {} => {
        Self::vote(accounts,program_id)
      }
      GameInstruction::ClaimSOL {} => {
        Self::claim_sol_from_sold_nft(accounts,program_id)
      }
      GameInstruction::ClaimUSDC {} => {
        Self::claim_usdc_from_sold_nft(accounts,program_id)
      }
      
    }
  }

  
  
  

//should fungible assets send to participants instantly or after a succesful purchase they claim themselves??

  fn start_fund_raising_to_buy_nft_from_another_marketplace(
    accounts: &[AccountInfo],
    program_id: &Pubkey) -> ProgramResult {


    let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();
    
    //fonsiyon baska bir pazaryerindeki nft'yi almak icin bir hesap acar. 
    //hedeflenen miktar hedeflenen surede toplanirsa nftyi almak icin CPI yapar.
    //farkli pazaryerleri icin farkli CPIler yapmak gerekecektir.

    Ok(())
  }

  fn join_fund_raising_to_buy_nft_from_another_marketplace(
    accounts: &[AccountInfo],
    program_id: &Pubkey) -> ProgramResult {


    let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();

    //start_fund_raising_to_buy_nft_from_another_marketplace fonsiyonu ile acilan hesaba yatirim gonderir.
    


    Ok(())
  }

  fn return_assets_from_failed_fund_raising_to_buy_nft_from_another_marketplace(
    accounts: &[AccountInfo],
    program_id: &Pubkey) -> ProgramResult {


    let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();

    //start_fund_raising_to_buy_nft_from_another_marketplace fonksiyonu ile baslatilan 
    //girisim hedefine ulasamaz ise kullanici yatirimini geri alir

    Ok(())
  }
  
  fn fail_fund_raising_to_buy_nft_from_another_marketplace(
    accounts: &[AccountInfo],
    program_id: &Pubkey) -> ProgramResult {


    let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();

    //fund raising ends if nft is purchased by anybody else. not sold anymore, price rised, burned, etc...

    //start_fund_raising_to_buy_nft_from_another_marketplace fonksiyonu ile baslatilan girisimin
    //basarisiz oldugunu onaylamak icin kullanilan fonksiyondur. farkli degiskenleri denetleyecektir
    //bu fonksiyon cagrilarak yatirimcilarin fundraising deadline zamanini beklemeden yatirimlarini geri almasi saglanir


    Ok(())
  }
  
  //can fund raising be canceled?
  //can assets be removed from a fundraising before it fails?

//////////////////////////////////////////////////////////////////////////////////

  fn sell_nft_as_whole_in_this_program(
    accounts: &[AccountInfo],
    program_id: &Pubkey) -> ProgramResult {


    let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();
    
    //fonksiyon bu program uzerinde yani bizim pazaryerimizde bir nftyi butun olarak satisa cikarir.
    //Yatirimcilar bu fonksiyonun actigi hesaba varlikalrini gonderir. hedef rakama ulasildiginda
    //bir yatirimci ucretin hepsini odeyip nftyi alabilir. oteki yatirimcilar varliklarini geri iade alir
    //nft saticisina satis ucreti toplu gonderilir

    Ok(())
  }

  fn cancel_sale_of_nft_as_whole_in_this_program(
    accounts: &[AccountInfo],
    program_id: &Pubkey) -> ProgramResult {


    let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();
    
    //nft saticisi sell_nft_as_whole_in_this_program fonksiyonu ile baslattigi satisi iptal eder.
    //yatirimcilar varlikalrini geri talep eder

    Ok(())
  }

  fn buy_whole_nft_from_this_program(
    accounts: &[AccountInfo],
    program_id: &Pubkey) -> ProgramResult {


    let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();

    //fonksiyonu cagiran bu program da satilan bir nft'nin hepsini satin alir ve cuzdaninda sahip olur.
    //oteki yatirimcilar varliklarini geri iade alir.

    Ok(())
  }


  fn join_fund_raising_for_sale_of_an_nft_as_whole_from_this_program(
    accounts: &[AccountInfo],
    program_id: &Pubkey) -> ProgramResult {


    let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();

    //sell_nft_as_whole_in_this_program ile acilan hesaba yatirim gonderir.

    Ok(())
  }

//////////////////////////////////////////////////////////////////////////////////

fn tokenize_nft_and_sell_in_this_program(
  accounts: &[AccountInfo],
  program_id: &Pubkey) -> ProgramResult {


  let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();

  //nft saticisi nftsini bu programda hisselendirip satisa cikarir.
  //nft hisseleri satildikca nft saticisinin cuzdanina parcalar halinde varlik gelir
  //alici ise cuzdanina token yani hisse alir

  Ok(())
}

  fn buy_part_of_tokenized_nft_from_this_program(
    accounts: &[AccountInfo],
    program_id: &Pubkey) -> ProgramResult {


    let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();
    
    //fonsiyonu cagiran tokenize_nft_and_sell_in_this_program fonksiyonu ile tokenize edilen nftden parcalar alir

    Ok(())
  }



  fn stop_sale_of_tokenized_nft_and_return_tokens(
    accounts: &[AccountInfo],
    program_id: &Pubkey) -> ProgramResult {


    let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();
   
    //nft saticisi tokenize_nft_and_sell_in_this_program fonksiyonu ile satisa cikardigi nftsinin satisini iptal eder
    //eger hic bir hisse satilmamissa nftyi butun olarak cuzdanina iade alir.
    //eger hisse satilmissa geri kalan bedel token olarak cuzdanina gelir, nft tokenize olarak programda kalir

    Ok(())
  }

  //////////////////////////////////////////////////////////

  fn make_an_offer_for_nft(
    accounts: &[AccountInfo],
    program_id: &Pubkey) -> ProgramResult {


    let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();
     
     //fonksiyon belirlenen mintteki nftnin alimi icin teklif acar.
     //satis gerceklesirse teklifteki bedel alicinin yatirim hesabindan cekilir.
     //teklif bir kisiye aittir.
     //nft sahibi teklifi kabul ederse nft alici cuzdanina gonderilir 
     //nft sahibinin cuzdanina teklifteki tutar gonderilir

    Ok(())
  }

  fn make_an_offer_for_nft_in_this_program(
    accounts: &[AccountInfo],
    program_id: &Pubkey) -> ProgramResult {


    let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();

    //make_an_offer_for_nft fonksiyonunu aynisi
    //sadece bu programda satilan nft icin


    Ok(())
  }

  fn start_voting_for_liquidation(
    accounts: &[AccountInfo],
    program_id: &Pubkey) -> ProgramResult {


    let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();

    //nftnin likide olmasi icin oylama baslatir


    Ok(())
  }

  fn liquidate_nft(
    accounts: &[AccountInfo],
    program_id: &Pubkey) -> ProgramResult {


    let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();

    //start_voting_for_liquidation fonksiyonu ile baslatilan likide etme oylamasinda likide karari cikarsa
    //nftye nerede teklif verilmis ise orada likide eder

    Ok(())
  }

  fn vote(
    accounts: &[AccountInfo],
    program_id: &Pubkey) -> ProgramResult {


    let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();
    
    //likide oylamasinda oy kullanir
 
    Ok(())
  }

  

  fn claim_sol_from_sold_nft(
    accounts: &[AccountInfo],
    program_id: &Pubkey) -> ProgramResult {


    let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();

    //satilan bir nftden elde edilen geliri hissesi oraninda talep eder.
    //hisseyi temsil eden tokenlar burn olur


    Ok(())
  }

  fn claim_usdc_from_sold_nft(
    accounts: &[AccountInfo],
    program_id: &Pubkey) -> ProgramResult {


    let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();

    //satilan bir nftden elde edilen geliri hissesi oraninda talep eder.
    //hisseyi temsil eden tokenlar burn olur

    Ok(())
  }

  fn create_investor_account(
    accounts: &[AccountInfo],
    program_id: &Pubkey) -> ProgramResult {


    let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();

    //yatirimci hesabi yatirimcinin birden fazla nft icin teklif acabilmesine olanak saglar.
    //yatirimci hesabinda yeterli yatirim kalmamissa nft icin verdigi teklifler goruntulenmez

    Ok(())
  }

  fn create_dealer_account(
    accounts: &[AccountInfo],
    program_id: &Pubkey) -> ProgramResult {


    let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();

    //dealer kendisine gonderilen yatirimlari degerlendirir


    Ok(())
  }

}


use solana_program::program_pack::Pack;

use solana_program::{
  account_info:: AccountInfo,

  pubkey::Pubkey,


};



use spl_token::state::Account;

pub fn check_mint_and_owner(mint: &Pubkey,owner: &Pubkey,ata:&AccountInfo)  {

    let result: Result<Account, solana_program::program_error::ProgramError>  = Account::unpack_from_slice(&ata.data.borrow());

    let ata_unpacked: spl_token::state::Account = match result {
        Ok(account) => account,
        Err(error) => {panic!("{}",error);}};

    if mint != &ata_unpacked.mint {panic!()}
    if owner != &ata_unpacked.owner {panic!()}

}
pub fn check_amount(amount: u64, ata:&AccountInfo) {

    let result: Result<Account, solana_program::program_error::ProgramError>  = Account::unpack_from_slice(&ata.data.borrow());

    let ata_unpacked: spl_token::state::Account = match result {
        Ok(account) => account,
        Err(error) => {panic!("{}",error);}};

    if amount > ata_unpacked.amount{panic!()}

}
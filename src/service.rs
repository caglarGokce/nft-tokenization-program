use solana_program::{instruction::Instruction, pubkey::Pubkey};



pub fn create_nft_transfer_instruction(token_program_id:&Pubkey,source_pubkey:&Pubkey,mint_pubkey:&Pubkey,destination_pubkey:&Pubkey,authority_pubkey:&Pubkey)
-> Instruction { 

    let ix: solana_program::instruction::Instruction; 

        let result: Result<solana_program::instruction::Instruction, solana_program::program_error::ProgramError> = spl_token_2022::instruction::transfer_checked(
              &token_program_id,
              &source_pubkey, 
              &mint_pubkey, 
              &destination_pubkey, 
              &authority_pubkey, 
              &[],1,0);
  
              ix =  match result {
                Ok(instruction) => instruction,
                Err(error) => {panic!("{}",error);}};
  

      return ix;

 }


 pub fn create_token_transfer_instruction(token_program_id:&Pubkey,source_pubkey:&Pubkey,mint_pubkey:&Pubkey,destination_pubkey:&Pubkey,authority_pubkey:&Pubkey,amount:u64,decimals:u8)
 -> Instruction { 
 
  let ix: solana_program::instruction::Instruction; 

      let result:Result<solana_program::instruction::Instruction, solana_program::program_error::ProgramError>  = spl_token::instruction::transfer_checked(
        &token_program_id,
        &source_pubkey, 
        &mint_pubkey, 
        &destination_pubkey, 
        &authority_pubkey, 
        &[],amount,decimals);

        ix =  match result {
          Ok(instruction) => instruction,
          Err(error) => {panic!("{}",error);}};

 
       return ix;
 
  }
 
 
 
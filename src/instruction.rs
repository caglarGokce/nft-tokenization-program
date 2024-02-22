use crate::error::MailError::InvalidInstruction;
use crate::state::{};
use borsh::BorshDeserialize;
use solana_program::program_error::ProgramError;

#[derive(Debug, PartialEq)]
pub enum GameInstruction {

  StartFundRaising,
  JoinFundRaising,
  ReturnAssets,
  FailFundRaising,
  SellWholeNFT,
  CancelWholeNFTSale,
  JoinFundRaisingProgram,
  BuyTokenizedNFT,
  TokenizeNFT,
  BuyWholeNFTProgram,
  StopTokenizedNFTSale,
  MakeOffer,
  MakeOfferProgram,
  StartVoting,
  Liquidate,
  Vote,
  ClaimSOL,
  ClaimUSDC,

}

impl GameInstruction {
  pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
    let (tag, rest) = input.split_first().ok_or(InvalidInstruction)?;
    
    Ok(match tag {
     0 => Self::StartFundRaising,
     1 => Self::JoinFundRaising,
     2 => Self::ReturnAssets,
     3 => Self::FailFundRaising,
     4 => Self::SellWholeNFT,
     5 => Self::CancelWholeNFTSale,
     6 => Self::JoinFundRaisingProgram,
     7 => Self::BuyTokenizedNFT,
     8 => Self::TokenizeNFT,
     9 => Self::BuyWholeNFTProgram,
     10 => Self::StopTokenizedNFTSale,
     11 => Self::MakeOffer,
     12 => Self::MakeOfferProgram,
     13 => Self::StartVoting,
     14 => Self::Liquidate,
     15 => Self::Vote,
     16 => Self::ClaimSOL,
     17 => Self::ClaimUSDC,
      

      _ => return Err(InvalidInstruction.into()),
    })
  }
}

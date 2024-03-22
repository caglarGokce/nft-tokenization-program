use crate::error::MailError::InvalidInstruction;
use crate::state::{ InitPDA, StartVoting,  Lamports, InitAccount, NFTState,  VoteData, Terms};
use borsh::BorshDeserialize;
use solana_program::program_error::ProgramError;

#[derive(Debug, PartialEq)]
pub enum NFTInstruction {
  StartFundRaising{data:InitAccount},
  JoinFundRaising{data:Lamports},
  RemoveFunds{data:Lamports},
  GetTokenizedAsset,
  BuyNFTFunds,
  SellNFTtoFund,
  CreateFunder,
  CreateFundRaisingAcc{data:InitPDA},
  SellWholeNFT{data:NFTState},
  CancelWholeNFTSale,
  BuyWholeNFTProgram,
  TokenSol,
  TokenizeNFTSell{data:NFTState},
  BuyTokenizedNFT{data:Lamports},
  StopTokenizedNFTSale,
  BuyOutNFT,
  TokenizeNFT{data:NFTState},
  InitVoting{data:StartVoting},
  RepeatVoting{data:StartVoting},
  SetVoteResult,
  Vote{data:VoteData},
  InitVoteAccount{data:InitAccount},
  LiquidateProg,
  LiquidateIndv,
  MakeOffer{data:Lamports},
  CreateInvestorAccount{data:Lamports},
  FundInvestorAccount{data:Lamports},
  CreateMeta,
  Register{data:InitPDA},
  UpdateTerms{data:Terms},
  BuyNFTFundsProgrm,
  RemoveFundsInvestorAcc{data:Lamports},
}


impl NFTInstruction {
  pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
    let (tag, rest) = input.split_first().ok_or(InvalidInstruction)?;
    
    Ok(match tag {
     0 => Self::StartFundRaising {
      data: InitAccount::try_from_slice(&rest)?,
     },
     1 => Self::JoinFundRaising{
      data: Lamports::try_from_slice(&rest)?,
     },
     2 => Self::RemoveFunds{
      data: Lamports::try_from_slice(&rest)?,
     },
     3 => Self::GetTokenizedAsset,
     4 => Self::BuyNFTFunds,
     5 => Self::SellNFTtoFund,
     6 => Self::SellWholeNFT{
      data: NFTState::try_from_slice(&rest)?,
     },
     7 => Self::CancelWholeNFTSale,
     8 => Self::TokenSol,
     9 => Self::BuyTokenizedNFT{
      data: Lamports::try_from_slice(&rest)?,
     },
     10 => Self::TokenizeNFTSell {
      data: NFTState::try_from_slice(&rest)?,
    },
    11 => Self::TokenizeNFT {
      data: NFTState::try_from_slice(&rest)?,
    },
     12 => Self::BuyWholeNFTProgram,
     13 => Self::StopTokenizedNFTSale,
     14 => Self::BuyOutNFT,
     15 => Self::MakeOffer{
      data: Lamports::try_from_slice(&rest)?,
     },
     16 => Self::InitVoting{
      data: StartVoting::try_from_slice(&rest)?,
     },
     17 => Self::LiquidateProg,
     18 => Self::LiquidateIndv,
     19 => Self::InitVoteAccount{
      data: InitAccount::try_from_slice(&rest)?,
     },
     20 => Self::SetVoteResult,

     21 => Self::RepeatVoting{
      data: StartVoting::try_from_slice(&rest)?,
     },
     22 => Self::Vote{
      data: VoteData::try_from_slice(&rest)?,
     },
     23 => Self::CreateInvestorAccount{
      data: Lamports::try_from_slice(&rest)?,
     },
     24 => Self::CreateFunder,
     25 => Self::FundInvestorAccount{
      data: Lamports::try_from_slice(&rest)?,
     },
     26 => Self::CreateFundRaisingAcc{
      data: InitPDA::try_from_slice(&rest)?,
     },
     27 => Self::Register {
      data: InitPDA::try_from_slice(&rest)?,
     },
     28 => Self::CreateMeta,
     29 => Self::UpdateTerms{
      data: Terms::try_from_slice(&rest)?
     },
     30 => Self::BuyNFTFundsProgrm,
     31 => Self::RemoveFundsInvestorAcc{
      data: Lamports::try_from_slice(&rest)?,
     },

      _ => return Err(InvalidInstruction.into()),
    })
  }
}


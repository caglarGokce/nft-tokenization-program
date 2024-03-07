use crate::error::MailError::InvalidInstruction;
use crate::state::{ InitPDA, InitVoting, InvestorAccount, Lamports, NFTAccount, NFTTerms, Proposal, VoteData};
use borsh::BorshDeserialize;
use solana_program::program_error::ProgramError;

#[derive(Debug, PartialEq)]
pub enum NFTInstruction {
  StartFundRaising{data:NFTAccount},
  JoinFundRaising{data:Lamports},
  RemoveFunds{data:Lamports},
  GetTokenizedAsset,
  BuyNFTFunds,
  SellNFTtoFund,
  CreateFunder,
  CreateFundRaisingAcc{data:InitPDA},
  SellWholeNFT{data:NFTAccount,data2:NFTTerms},
  CancelWholeNFTSale,
  BuyWholeNFTProgram,
  TokenSol,
  TokenizeNFTSell{data:NFTAccount,data2:NFTTerms},
  BuyTokenizedNFT{data:Lamports},
  StopTokenizedNFTSale,
  BuyOutNFT,
  TokenizeNFT{data:NFTAccount,data2:NFTTerms},
  InitVoting{data:InitVoting},
  RepeatVoting{data:InitVoting},
  SetVoteResult,
  Vote{data:VoteData},
  InitVoteAccount{data:InitVoting},
  LiquidateProg,
  LiquidateIndv,
  MakeOffer{data:Proposal},
  CreateInvestorAccount{data:InvestorAccount},
  FundInvestorAccount{data:InvestorAccount},
  CreateMeta{data:InitPDA},
  Register{data:InitPDA},

}

impl NFTInstruction {
  pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
    let (tag, rest) = input.split_first().ok_or(InvalidInstruction)?;
    
    Ok(match tag {
     0 => Self::StartFundRaising {
      data: NFTAccount::try_from_slice(&rest)?,
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

     6 => {
      let (data_bytes, rest2) = rest.split_at(33);
      let data: NFTAccount = NFTAccount::try_from_slice(data_bytes)?;

      // Extract data bytes (assuming the rest)
      let data2: NFTTerms = NFTTerms::try_from_slice(rest2)?;

      Self::SellWholeNFT { data, data2 }
    },
     7 => Self::CancelWholeNFTSale,
     8 => Self::TokenSol,
     9 => Self::BuyTokenizedNFT{
      data: Lamports::try_from_slice(&rest)?,
     },
     10 => {
      let (data_bytes, rest2) = rest.split_at(33);
      let data: NFTAccount = NFTAccount::try_from_slice(data_bytes)?;

      // Extract data bytes (assuming the rest)
      let data2: NFTTerms = NFTTerms::try_from_slice(rest2)?;

      Self::TokenizeNFTSell { data, data2 }
    },
    11 => {
      let (data_bytes, rest2) = rest.split_at(33);
      let data: NFTAccount = NFTAccount::try_from_slice(data_bytes)?;

      // Extract data bytes (assuming the rest)
      let data2: NFTTerms = NFTTerms::try_from_slice(rest2)?;

      Self::TokenizeNFT { data, data2 }
    },
     12 => Self::BuyWholeNFTProgram,
     13 => Self::StopTokenizedNFTSale,
     14 => Self::BuyOutNFT,
     15 => Self::MakeOffer{
      data: Proposal::try_from_slice(&rest)?,
     },
     16 => Self::InitVoting{
      data: InitVoting::try_from_slice(&rest)?,
     },
     17 => Self::LiquidateProg,
     18 => Self::LiquidateIndv,
     19 => Self::InitVoteAccount{
      data: InitVoting::try_from_slice(&rest)?,
     },
     20 => Self::SetVoteResult,

     21 => Self::RepeatVoting{
      data: InitVoting::try_from_slice(&rest)?,
     },
     22 => Self::Vote{
      data: VoteData::try_from_slice(&rest)?,
     },
     23 => Self::CreateInvestorAccount{
      data: InvestorAccount::try_from_slice(&rest)?,
     },
     24 => Self::CreateFunder,
     25 => Self::FundInvestorAccount{
      data: InvestorAccount::try_from_slice(&rest)?,
     },
     26 => Self::CreateFundRaisingAcc{
      data: InitPDA::try_from_slice(&rest)?,
     },
     27 => Self::Register {
      data: InitPDA::try_from_slice(&rest)?,
     },
     28 => Self::CreateMeta{
      data: InitPDA::try_from_slice(&rest)?,
     },
      _ => return Err(InvalidInstruction.into()),
    })
  }
}



import { WalletContextState } from '@solana/wallet-adapter-react';
import { deserialize } from 'borsh';
import { ASSOCIATED_TOKEN_PROGRAM_ID,  getAssociatedTokenAddressSync, } from '@solana/spl-token';
import { Connection, PublicKey, TransactionInstruction, TransactionMessage, VersionedTransaction, Keypair, AccountInfo,  } from '@solana/web3.js';
import { FundRaising, FundRaisingSchema, NFT, NFTTerms,  NFTTermsSchema,   Proposal,  ProposalSchema, UserAddresTokenMint, UserAddresTokenMintSchema } from './model';
import { metaplex_program, programID } from './key';
var BASE58 = '123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz';
var bs58 = require('base-x')(BASE58);



//const connection = new Connection(clusterApiUrl("testnet"))
const connection= new Connection("http://localhost:8899","confirmed");


export const sendTransaction = async (wallet:WalletContextState,ix:TransactionInstruction[],signers:Keypair[]) => {

  const message = new TransactionMessage({
      instructions: ix,
        payerKey: wallet.publicKey!,
        recentBlockhash : (await connection.getLatestBlockhash()).blockhash
      }).compileToV0Message();
  
      const tx = new VersionedTransaction(message);

      const latestBlockHash = await connection.getLatestBlockhash();

      if (signers.length != 0){
        tx.sign(signers);

      }

      const sig = await wallet.sendTransaction(tx,connection);

}
export const deserialize_user_token = async (acc_info:AccountInfo<Buffer>) => {

  const account_data:UserAddresTokenMint = deserialize(UserAddresTokenMintSchema,UserAddresTokenMint,acc_info.data);
  
  
  return account_data;
  }
  export const deserialize_proposal = async (acc_info:AccountInfo<Buffer>) => {
  
  const account_data:Proposal = deserialize(ProposalSchema,Proposal,acc_info.data);
  
  const proposer = new PublicKey(bs58.encode(account_data.proposer).toString());
  
  return proposer;
  }
export const deserialize_fundraising_data = async (data:Buffer) => {
  const account_data:FundRaising = deserialize(FundRaisingSchema,FundRaising,data);

  return account_data;
}
export const get_publickey_from_bytes = async (data:number[]) => {

  const key = new PublicKey(bs58.encode(data).toString());

  return key;
}
export const deserialize_nftterms_data = async (data:Buffer) => {
const account_data:NFTTerms = deserialize(NFTTermsSchema,NFTTerms,data);
const token_mint = new PublicKey(bs58.encode(account_data.tokenization_mint).toString());
const nft_mint = new PublicKey(bs58.encode(account_data.nft_mint).toString());
const owner = new PublicKey(bs58.encode(account_data.owner).toString());

console.log("owner "+owner.toBase58());
console.log("buy_out_allowed "+account_data.buy_out_allowed);
console.log("buy_out_price "+account_data.buy_out_price);
console.log("number_of_tokens "+account_data.number_of_tokens);
console.log("tokens_sold "+account_data.tokens_sold);
console.log("tokenized_for_sale "+account_data.tokenized_for_sale);
console.log("for_sale "+account_data.for_sale);
console.log("buy_out_allowed "+account_data.buy_out_allowed);
console.log("owned_by_pda "+account_data.owned_by_pda);
console.log("price "+account_data.price);
console.log("buy_out_price "+account_data.buy_out_price);
console.log("lamports_per_token "+account_data.lamports_per_token);
console.log("lamports_per_token_buyout "+account_data.lamports_per_token_buyout);

return account_data;
}
export const get_ata = async (owner:PublicKey,mint:PublicKey,onCurve:boolean,token_program:PublicKey) => {

  const ata = getAssociatedTokenAddressSync(mint,owner,onCurve,token_program,ASSOCIATED_TOKEN_PROGRAM_ID);

  return ata;
    
}
export const get_fundraising_account = async (nft_mint:PublicKey) => {

const pda = PublicKey.findProgramAddressSync([Buffer.from("fund"),nft_mint.toBytes()],programID);


return pda;

}
export const get_distribution_account = async (token_mint:PublicKey) => {

  const pda = PublicKey.findProgramAddressSync([token_mint.toBytes()],programID);

  return pda;

}
export const get_vote_account_pda = async (tokenezation:PublicKey) => {

  const pda = PublicKey.findProgramAddressSync([Buffer.from("vote"),tokenezation.toBytes()],programID);
  
  return pda;
  
}
 export const get_voter_pda = async (wallet:WalletContextState,token_mint:PublicKey) => {
  
  const pda = PublicKey.findProgramAddressSync([wallet.publicKey!.toBytes(),token_mint.toBytes()],programID);
  
  return pda;
}
export const get_investor_account = async (investor:PublicKey) => {
  
  const accounts = await connection.getProgramAccounts(
    programID,
    {
      filters: [
        {
          dataSize: 40,
        },
        {
          memcmp: {
            offset: 0, 
            bytes: investor.toString(),
          },
        },
      ],
    }
  );
  
  return accounts[0].pubkey;
  
}
export const get_registered_nft_account_address = async (nft_mint:PublicKey) => {

  const pda = PublicKey.findProgramAddressSync([nft_mint.toBytes()],programID);


  return pda;

}
export const deserialize_nft_metadata = async (data:Buffer) => {

  //TODO deserialize metadata
  const nft = new NFT()
  
  return nft;
  }


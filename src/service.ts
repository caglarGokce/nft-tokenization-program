
import { ASSOCIATED_TOKEN_PROGRAM_ID, TOKEN_2022_PROGRAM_ID  } from '@solana/spl-token';
import { Connection, PublicKey, TransactionInstruction, SystemProgram,Keypair, LAMPORTS_PER_SOL,SYSVAR_RENT_PUBKEY} from '@solana/web3.js';
import {dex, programID, terms_account} from './key';
import { get_ata, get_distribution_account,  get_fundraising_account,  get_voter_pda, get_registered_nft_account_address, 
      get_vote_account_pda, sendTransaction, get_investor_account,  } from './utils';
import { InitAccount, InitAccountSchema, InitPDA, InitPDASchema, Lamports, LamportsSchema,
     NFTTerms, NFTTermsSchema,  StartVoting, StartVotingSchema, VoteData, VoteDataSchema} from './model';
import { serialize } from 'borsh';
import { WalletContextState } from '@solana/wallet-adapter-react';
import { get_funders_account, get_token_program, get_owner, get_TokentoSol_account, get_proposal_account } from './getAccounts';

//const connection = new Connection(clusterApiUrl("testnet"))
const connection= new Connection("http://localhost:8899","confirmed");



export const start_fund_raising_to_buy_nft = async (nft_mint:PublicKey,wallet:WalletContextState,fund:number) => {

  const tokenization_mint = Keypair.generate();

  const fundraising_account = await get_fundraising_account(nft_mint);
  const token_dist_data = await get_distribution_account(tokenization_mint.publicKey);
  const registered_nft_account = await get_registered_nft_account_address(nft_mint);
  const dex_ata = await get_ata(dex,tokenization_mint.publicKey,true,TOKEN_2022_PROGRAM_ID);

  const initialize = new InitAccount();

  initialize.bump = token_dist_data[1];
  initialize.size = BigInt(182);
  initialize.lamports = BigInt(LAMPORTS_PER_SOL* fund); //funds to send to the fundraising 

  let encoded = serialize(InitAccountSchema,initialize);

  let concated = Uint8Array.of(0,...encoded);
    
    const ix = new TransactionInstruction({
        programId:programID,
        keys:[
            {isSigner:true,isWritable:true,pubkey:wallet.publicKey!},
            {isSigner:true,isWritable:true,pubkey:tokenization_mint.publicKey},
            {isSigner:false,isWritable:true,pubkey:nft_mint},
            {isSigner:false,isWritable:true,pubkey:fundraising_account[0]},
            {isSigner:false,isWritable:true,pubkey:token_dist_data[0]},
            {isSigner:false,isWritable:true,pubkey:registered_nft_account[0]},
            {isSigner:false,isWritable:true,pubkey:TOKEN_2022_PROGRAM_ID},
            {isSigner:false,isWritable:false,pubkey:terms_account},
            {isSigner:false,isWritable:false,pubkey:dex},
            {isSigner:false,isWritable:true,pubkey:dex_ata},
            {isSigner:false,isWritable:true,pubkey:SYSVAR_RENT_PUBKEY},
            {isSigner:false,isWritable:true,pubkey:ASSOCIATED_TOKEN_PROGRAM_ID},
            {isSigner:false,isWritable:true,pubkey:SystemProgram.programId},
        ],
        data:Buffer.from(concated)
    });

  const signers:Keypair[]=[tokenization_mint];
  
    try{
      await sendTransaction(wallet,[ix],signers);
    }catch(e){
       console.log(e);
    }

}///////////
export const join_fund_raising_to_buy_nft = async (tokenization_mint:PublicKey,nft_mint:PublicKey,wallet:WalletContextState,lamports:number) => {


    const fundraising_account = await get_fundraising_account(nft_mint);
    const temp = Keypair.generate();
  
    const amount = new Lamports();
  
    amount.lamports = BigInt(LAMPORTS_PER_SOL* lamports);
  
    let encoded = serialize(LamportsSchema,amount);
  
    let concated = Uint8Array.of(1,...encoded);
  
    const funder_tokenization_ata = await get_ata(wallet.publicKey!,tokenization_mint,false,TOKEN_2022_PROGRAM_ID);  
  
      try {
      const funders_account = await get_funders_account(wallet,tokenization_mint,nft_mint);
  
      const ix = new TransactionInstruction({
          programId:programID,
          keys:[
              {isSigner:true,isWritable:true,pubkey:wallet.publicKey!},
              {isSigner:false,isWritable:true,pubkey:funders_account},
              {isSigner:false,isWritable:true,pubkey:fundraising_account[0]},
              {isSigner:true,isWritable:true,pubkey:temp.publicKey},
              {isSigner:false,isWritable:true,pubkey:funder_tokenization_ata},
              {isSigner:false,isWritable:true,pubkey:tokenization_mint},
              {isSigner:false,isWritable:true,pubkey:TOKEN_2022_PROGRAM_ID},
              {isSigner:false,isWritable:false,pubkey:terms_account},
              {isSigner:false,isWritable:true,pubkey:SystemProgram.programId},
              {isSigner:false,isWritable:true,pubkey:ASSOCIATED_TOKEN_PROGRAM_ID},
          ],
          data:Buffer.from(concated)
        });
        
        const signers:Keypair[]=[temp];
        
        
        try{await sendTransaction(wallet,[ix],signers);}catch(e){ console.log(e);}
          
      } catch (error) {
  
      const funders_account =  Keypair.generate();
  
      const ix = new TransactionInstruction({
          programId:programID,
          keys:[
              {isSigner:true,isWritable:true,pubkey:wallet.publicKey!},
              {isSigner:true,isWritable:true,pubkey:funders_account.publicKey},
              {isSigner:false,isWritable:true,pubkey:fundraising_account[0]},
              {isSigner:true,isWritable:true,pubkey:temp.publicKey},
              {isSigner:false,isWritable:true,pubkey:funder_tokenization_ata},
              {isSigner:false,isWritable:true,pubkey:tokenization_mint},
              {isSigner:false,isWritable:true,pubkey:TOKEN_2022_PROGRAM_ID},
              {isSigner:false,isWritable:false,pubkey:terms_account},
              {isSigner:false,isWritable:true,pubkey:SystemProgram.programId},
              {isSigner:false,isWritable:true,pubkey:ASSOCIATED_TOKEN_PROGRAM_ID},
          ],
          data:Buffer.from(concated)
        });
        
        const signers:Keypair[]=[temp,funders_account];
        
        
        try{await sendTransaction(wallet,[ix],signers);}catch(e){ console.log(e);}
          
      }
  
  
}///////////
export const add_funds_to_fudnraising= async (tokenization_mint:PublicKey,nft_mint:PublicKey,funders_account:PublicKey,wallet:WalletContextState,lamports:number) => {

    const fundraising_account = await get_fundraising_account(nft_mint);
    const temp = Keypair.generate();
  
    const amount = new Lamports();
  
    amount.lamports = BigInt(LAMPORTS_PER_SOL* lamports);
  
    let encoded = serialize(LamportsSchema,amount);
  
    let concated = Uint8Array.of(1,...encoded);
  
    const funder_tokenization_ata = await get_ata(wallet.publicKey!,tokenization_mint,false,TOKEN_2022_PROGRAM_ID);  
  
  
      const ix = new TransactionInstruction({
          programId:programID,
          keys:[
              {isSigner:true,isWritable:true,pubkey:wallet.publicKey!},
              {isSigner:false,isWritable:true,pubkey:funders_account},
              {isSigner:false,isWritable:true,pubkey:fundraising_account[0]},
              {isSigner:true,isWritable:true,pubkey:temp.publicKey},
              {isSigner:false,isWritable:true,pubkey:funder_tokenization_ata},
              {isSigner:false,isWritable:true,pubkey:tokenization_mint},
              {isSigner:false,isWritable:true,pubkey:TOKEN_2022_PROGRAM_ID},
              {isSigner:false,isWritable:false,pubkey:terms_account},
              {isSigner:false,isWritable:true,pubkey:SystemProgram.programId},
              {isSigner:false,isWritable:true,pubkey:ASSOCIATED_TOKEN_PROGRAM_ID},
          ],
          data:Buffer.from(concated)
        });
        
        const signers:Keypair[]=[temp];
        
        
        try{await sendTransaction(wallet,[ix],signers);}catch(e){ console.log(e);}
  
}///////////
export const remove_funds_from_the_fundraising = async (nft_mint:PublicKey,funders_account:PublicKey,wallet:WalletContextState,lamports:number) => {

    const fundraising_account = await get_fundraising_account(nft_mint);

    const amount = new Lamports();

    amount.lamports = BigInt(LAMPORTS_PER_SOL* lamports);
  
    let encoded = serialize(LamportsSchema,amount);
  
    let concated = Uint8Array.of(2,...encoded);
  
  const ix = new TransactionInstruction({
    programId:programID,
    keys:[
        {isSigner:true,isWritable:true,pubkey:wallet.publicKey!},
        {isSigner:false,isWritable:true,pubkey:funders_account},
        {isSigner:false,isWritable:true,pubkey:fundraising_account[0]},
    ],
    data:Buffer.from(concated)
  });


  
    const signers:Keypair[]=[];


try{await sendTransaction(wallet,[ix],signers);}catch(e){ console.log(e);}

}///////////
export const get_tokenized_assets_from_successfull_fundraising = async (token_mint:PublicKey,funders_account:PublicKey,wallet:WalletContextState ) => {

  const token_dist_data = await get_distribution_account(token_mint);
  const funder_tokenization_ata = await get_ata(wallet.publicKey!,token_mint,false,TOKEN_2022_PROGRAM_ID);
  const userAddresTokenMint = Keypair.generate();

    const ix = new TransactionInstruction({
        programId:programID,
        keys:[
            {isSigner:false,isWritable:true,pubkey:wallet.publicKey!},
            {isSigner:false,isWritable:true,pubkey:funder_tokenization_ata},
            {isSigner:false,isWritable:true,pubkey:funders_account},
            {isSigner:false,isWritable:true,pubkey:token_dist_data[0]},
            {isSigner:false,isWritable:true,pubkey:token_mint},
            {isSigner:false,isWritable:true,pubkey:TOKEN_2022_PROGRAM_ID},
            {isSigner:true,isWritable:true,pubkey:userAddresTokenMint.publicKey},
            {isSigner:false,isWritable:false,pubkey:terms_account},
            {isSigner:false,isWritable:false,pubkey:SystemProgram.programId},
        ],
        data:Buffer.from([3])
    });
  
    const signers:Keypair[]=[userAddresTokenMint];

try{await sendTransaction(wallet,[ix],signers);}catch(e){ console.log(e);}

}///////////
export const sell_nft_to_the_fundraising = async (nft_mint:PublicKey,token_mint:PublicKey,wallet:WalletContextState ) => {


    const fundraising_account = await get_fundraising_account(nft_mint);
    const token_dist_data = await get_distribution_account(token_mint);
    const registered_nft_account = await get_registered_nft_account_address(nft_mint);
    const token_program = await get_token_program(nft_mint);
    const seller_ata = await get_ata(wallet.publicKey!,nft_mint,false,token_program);
    const registered_nft_account_ata = await get_ata(registered_nft_account[0],nft_mint,true,token_program);
    

    const ix = new TransactionInstruction({
        programId:programID,
        keys:[
            {isSigner:false,isWritable:true,pubkey:wallet.publicKey!},
            {isSigner:false,isWritable:true,pubkey:seller_ata},
            {isSigner:false,isWritable:true,pubkey:token_dist_data[0]},
            {isSigner:false,isWritable:true,pubkey:fundraising_account[0]},
            {isSigner:false,isWritable:true,pubkey:registered_nft_account[0]},
            {isSigner:false,isWritable:true,pubkey:registered_nft_account_ata},
            {isSigner:false,isWritable:true,pubkey:nft_mint},
            {isSigner:false,isWritable:true,pubkey:token_program},
        ],
        data:Buffer.from([5])
    });
    
    const signers:Keypair[]=[];

    try{await sendTransaction(wallet,[ix],signers);}catch(e){ console.log(e);}

}///////////
export const buy_nft_listed_in_program_with_the_funds = async (nft_mint:PublicKey,token_mint:PublicKey,wallet:WalletContextState ) => {

  const fundraising_account = await get_fundraising_account(nft_mint);
  const token_dist_data = await get_distribution_account(token_mint);
  const registered_nft_account = await get_registered_nft_account_address(nft_mint);
  const token_program = await get_token_program(nft_mint);
  const registered_nft_account_ata = await get_ata(registered_nft_account[0],nft_mint,true,token_program);
  const seller = await get_owner(registered_nft_account[0]);

    const ix = new TransactionInstruction({
        programId:programID,
        keys:[
            {isSigner:false,isWritable:true,pubkey:seller},
            {isSigner:false,isWritable:true,pubkey:token_dist_data[0]},
            {isSigner:false,isWritable:true,pubkey:fundraising_account[0]},
            {isSigner:false,isWritable:true,pubkey:registered_nft_account[0]},
            {isSigner:false,isWritable:true,pubkey:registered_nft_account_ata},
            {isSigner:false,isWritable:true,pubkey:nft_mint},
        ],
        data:Buffer.from([30])
    });
  
      const signers:Keypair[]=[];

try{await sendTransaction(wallet,[ix],signers);}catch(e){ console.log(e);}

}///////////
export const list_nft_forsale_as_whole_in_this_program = async (nft_mint:PublicKey,wallet:WalletContextState,price:number,buy_out_price:number ) => {//6

      const registered_nft_account = await get_registered_nft_account_address(nft_mint);
      const token_program = await get_token_program(nft_mint);
      const seller_ata = await get_ata(wallet.publicKey!,nft_mint,false,token_program);
      const  registered_nft_account_ata = await get_ata(registered_nft_account[0],nft_mint,true,token_program);


      const initialize = new NFTTerms();

      initialize.bump = registered_nft_account[1];
      initialize.buy_out_price = BigInt(buy_out_price*LAMPORTS_PER_SOL);
      initialize.price = BigInt(price*LAMPORTS_PER_SOL);
    
      let encoded = serialize(NFTTermsSchema,initialize);
    
      let concated = Uint8Array.of(6,...encoded);
  
        const ix = new TransactionInstruction({
            programId:programID,
            keys:[
                {isSigner:false,isWritable:true,pubkey:wallet.publicKey!},
                {isSigner:false,isWritable:true,pubkey:seller_ata},
                {isSigner:false,isWritable:true,pubkey:registered_nft_account[0]},
                {isSigner:false,isWritable:true,pubkey:registered_nft_account_ata},
                {isSigner:false,isWritable:true,pubkey:nft_mint},
                {isSigner:false,isWritable:true,pubkey:token_program},
                {isSigner:false,isWritable:false,pubkey:terms_account},
            ],
            data:Buffer.from(concated)
        });
      
          const signers:Keypair[]=[];

          console.log(seller_ata.toBase58());
          console.log(registered_nft_account_ata.toBase58());


try{await sendTransaction(wallet,[ix],signers);}catch(e){ console.log(e);}

}//////////
export const cancel_sale_of_nft_as_whole_in_this_program = async (nft_mint:PublicKey,wallet:WalletContextState ) => {//7

    const registered_nft_account = await get_registered_nft_account_address(nft_mint);
    const token_program = await get_token_program(nft_mint);
    const seller_ata = await get_ata(wallet.publicKey!,nft_mint,false,token_program);
    const  registered_nft_account_ata = await get_ata(registered_nft_account[0],nft_mint,true,token_program);

      const ix = new TransactionInstruction({
          programId:programID,
          keys:[
              {isSigner:false,isWritable:true,pubkey:wallet.publicKey!},
              {isSigner:false,isWritable:true,pubkey:seller_ata},
              {isSigner:false,isWritable:true,pubkey:registered_nft_account[0]},
              {isSigner:false,isWritable:true,pubkey:registered_nft_account_ata},
              {isSigner:false,isWritable:true,pubkey:nft_mint},
              {isSigner:false,isWritable:true,pubkey:token_program},
              {isSigner:false,isWritable:true,pubkey:terms_account},
          ],
          data:Buffer.from([7])
      });
    
        const signers:Keypair[]=[];


try{await sendTransaction(wallet,[ix],signers);}catch(e){ console.log(e);}
    
}///////////
export const buy_whole_nft_from_this_program = async (nft_mint:PublicKey,wallet:WalletContextState ) => {//12

const registered_nft_account = await get_registered_nft_account_address(nft_mint);
const seller = await get_owner(registered_nft_account[0]);
const token_program = await get_token_program(nft_mint);
const buyer_ata = await get_ata(wallet.publicKey!,nft_mint,false,token_program);
const registered_nft_account_ata = await get_ata(registered_nft_account[0],nft_mint,true,token_program);
const temp = Keypair.generate();

  const ix = new TransactionInstruction({
      programId:programID,
      keys:[
          {isSigner:false,isWritable:true,pubkey:seller},
          {isSigner:true,isWritable:true,pubkey:wallet.publicKey!},
          {isSigner:true,isWritable:true,pubkey:temp.publicKey},
          {isSigner:false,isWritable:true,pubkey:buyer_ata},
          {isSigner:false,isWritable:true,pubkey:registered_nft_account[0]},
          {isSigner:false,isWritable:true,pubkey:registered_nft_account_ata},
          {isSigner:false,isWritable:true,pubkey:nft_mint},
          {isSigner:false,isWritable:true,pubkey:token_program},
          {isSigner:false,isWritable:true,pubkey:SYSVAR_RENT_PUBKEY},
          {isSigner:false,isWritable:true,pubkey:ASSOCIATED_TOKEN_PROGRAM_ID},
          {isSigner:false,isWritable:true,pubkey:SystemProgram.programId},
      ],
      data:Buffer.from([12])
  });

    const signers:Keypair[]=[temp];

    console.log(buyer_ata.toBase58())

try{await sendTransaction(wallet,[ix],signers);}catch(e){ console.log(e);}


}///////////
export const change_tokens_to_sol = async (token_mint:PublicKey,wallet:WalletContextState ) => {//8


    const buyer_tokenization_ata = await get_ata(wallet.publicKey!,token_mint,false,TOKEN_2022_PROGRAM_ID);
    const tokenization_account = await get_TokentoSol_account(token_mint);

    const ix = new TransactionInstruction({
        programId:programID,
        keys:[
            {isSigner:false,isWritable:true,pubkey:wallet.publicKey!},
            {isSigner:false,isWritable:true,pubkey:buyer_tokenization_ata},
            {isSigner:false,isWritable:true,pubkey:token_mint},
            {isSigner:false,isWritable:true,pubkey:tokenization_account},
            {isSigner:false,isWritable:true,pubkey:TOKEN_2022_PROGRAM_ID},
            {isSigner:false,isWritable:true,pubkey:SYSVAR_RENT_PUBKEY},
            {isSigner:false,isWritable:true,pubkey:ASSOCIATED_TOKEN_PROGRAM_ID},
        ],
        data:Buffer.from([8])
    });
    
    const signers:Keypair[]=[];

    console.log(buyer_tokenization_ata.toString())


try{await sendTransaction(wallet,[ix],signers);}catch(e){ console.log(e);}

}
export const tokenize_nft_and_sell_in_this_program = async (nft_mint:PublicKey,wallet:WalletContextState, price_per_share:number, divide_into:number,lamports_per_token_buyout:number ) => {//10

  const token_mint = Keypair.generate();
  const registered_nft_account = await get_registered_nft_account_address(nft_mint);
  const token_program = await get_token_program(nft_mint);
  const seller_tokenization_ata = await get_ata(wallet.publicKey!,token_mint.publicKey,false,TOKEN_2022_PROGRAM_ID);
  const seller_ata = await get_ata(wallet.publicKey!,nft_mint,false,token_program);
  const  registered_nft_account_ata = await get_ata(registered_nft_account[0],nft_mint,true,token_program);
  const dex_ata = await get_ata(dex,token_mint.publicKey,true,TOKEN_2022_PROGRAM_ID);



  const initialize = new NFTTerms();

  initialize.bump = registered_nft_account[1];
  initialize.lamports_per_token = BigInt(price_per_share*LAMPORTS_PER_SOL);
  initialize.number_of_tokens = BigInt(divide_into);
  initialize.lamports_per_token_buyout = BigInt(lamports_per_token_buyout*LAMPORTS_PER_SOL);
  initialize.price = BigInt(price_per_share*LAMPORTS_PER_SOL*divide_into);
  initialize.buy_out_price = BigInt(lamports_per_token_buyout*LAMPORTS_PER_SOL*divide_into);

  let encoded = serialize(NFTTermsSchema,initialize);

  let concated = Uint8Array.of(10,...encoded);

    const ix = new TransactionInstruction({
        programId:programID,
        keys:[
            {isSigner:true,isWritable:true,pubkey:wallet.publicKey!},
            {isSigner:false,isWritable:true,pubkey:seller_ata},
            {isSigner:false,isWritable:true,pubkey:seller_tokenization_ata},
            {isSigner:false,isWritable:true,pubkey:registered_nft_account[0]},
            {isSigner:false,isWritable:true,pubkey:registered_nft_account_ata},
            {isSigner:false,isWritable:true,pubkey:nft_mint},
            {isSigner:true,isWritable:true,pubkey:token_mint.publicKey},
            {isSigner:false,isWritable:true,pubkey:token_program},
            {isSigner:false,isWritable:true,pubkey:TOKEN_2022_PROGRAM_ID},
            {isSigner:false,isWritable:true,pubkey:SYSVAR_RENT_PUBKEY},
            {isSigner:false,isWritable:false,pubkey:terms_account},
            {isSigner:false,isWritable:false,pubkey:dex},
            {isSigner:false,isWritable:true,pubkey:dex_ata},
            {isSigner:false,isWritable:false,pubkey:SystemProgram.programId},
            {isSigner:false,isWritable:false,pubkey:ASSOCIATED_TOKEN_PROGRAM_ID},
        ],
        data:Buffer.from(concated)
    });
  
      const signers:Keypair[]=[token_mint];


try{await sendTransaction(wallet,[ix],signers);}catch(e){ console.log(e);}

}//////////
export const buy_part_of_tokenized_nft_from_this_program = async (nft_mint:PublicKey,token_mint:PublicKey,wallet:WalletContextState, number_of_tokens_to_buy:number ) => {//9

const registered_nft_account = await get_registered_nft_account_address(nft_mint);
const seller = await get_owner(registered_nft_account[0]);
const buyer_tokenization_ata = await get_ata(wallet.publicKey!,token_mint,true,TOKEN_2022_PROGRAM_ID);

const useradresstokenmint = Keypair.generate();
const temp = Keypair.generate();

const amount = new Lamports();
amount.lamports = BigInt(number_of_tokens_to_buy);

let encoded = serialize(LamportsSchema,amount);

let concated = Uint8Array.of(9,...encoded);

  const ix = new TransactionInstruction({
      programId:programID,
      keys:[
          {isSigner:false,isWritable:true,pubkey:seller},
          {isSigner:true,isWritable:true,pubkey:wallet.publicKey!},
          {isSigner:true,isWritable:true,pubkey:temp.publicKey},
          {isSigner:false,isWritable:true,pubkey:buyer_tokenization_ata},
          {isSigner:false,isWritable:true,pubkey:registered_nft_account[0]},
          {isSigner:false,isWritable:true,pubkey:nft_mint},
          {isSigner:false,isWritable:true,pubkey:token_mint},
          {isSigner:false,isWritable:true,pubkey:TOKEN_2022_PROGRAM_ID},
          {isSigner:false,isWritable:true,pubkey:SYSVAR_RENT_PUBKEY},
          {isSigner:true,isWritable:true,pubkey:useradresstokenmint.publicKey},
          {isSigner:false,isWritable:false,pubkey:terms_account},
          {isSigner:false,isWritable:false,pubkey:SystemProgram.programId},
          {isSigner:false,isWritable:false,pubkey:ASSOCIATED_TOKEN_PROGRAM_ID},
      ],
      data:Buffer.from(concated)
  });

    const signers:Keypair[]=[temp,useradresstokenmint];

    console.log(buyer_tokenization_ata.toString())

try{await sendTransaction(wallet,[ix],signers);}catch(e){ console.log(e);}

}///////////
export const stop_sale_of_tokenized_nft_and_return_tokens = async (nft_mint:PublicKey,token_mint:PublicKey,wallet:WalletContextState ) => {//13

const registered_nft_account = await get_registered_nft_account_address(nft_mint);
const owner_tokenization_mint_ata = await get_ata(wallet.publicKey!,token_mint,false,TOKEN_2022_PROGRAM_ID);
    
   const useradresstokenmint = Keypair.generate();

  const ix = new TransactionInstruction({
      programId:programID,
      keys:[
          {isSigner:false,isWritable:true,pubkey:wallet.publicKey!},
          {isSigner:false,isWritable:true,pubkey:owner_tokenization_mint_ata},
          {isSigner:false,isWritable:true,pubkey:nft_mint},
          {isSigner:false,isWritable:true,pubkey:token_mint},
          {isSigner:false,isWritable:true,pubkey:registered_nft_account[0]},
          {isSigner:false,isWritable:true,pubkey:TOKEN_2022_PROGRAM_ID},
          {isSigner:false,isWritable:true,pubkey:useradresstokenmint.publicKey},
          {isSigner:false,isWritable:true,pubkey:terms_account},
          {isSigner:false,isWritable:true,pubkey:SystemProgram.programId},
      ],
      data:Buffer.from([13])
  });

    const signers:Keypair[]=[useradresstokenmint];


try{await sendTransaction(wallet,[ix],signers);}catch(e){ console.log(e);}
}///////////
export const buy_out_tokenized_nft = async (nft_mint:PublicKey,token_mint:PublicKey,wallet:WalletContextState ) => {//14

    const registered_nft_account = await get_registered_nft_account_address(nft_mint);
    const owner = await get_owner(registered_nft_account[0]);
    const token_program = await get_token_program(nft_mint);
    const buyer_ata = await get_ata(wallet.publicKey!,nft_mint,false,token_program);
    const registered_nft_account_ata = await get_ata(registered_nft_account[0],nft_mint,true,token_program);
    const temp = Keypair.generate();
    const tokenized_nft_account = Keypair.generate();

    const ix = new TransactionInstruction({
        programId:programID,
        keys:[
            {isSigner:false,isWritable:true,pubkey:owner},
            {isSigner:true,isWritable:true,pubkey:wallet.publicKey!},
            {isSigner:true,isWritable:true,pubkey:temp.publicKey},
            {isSigner:false,isWritable:true,pubkey:buyer_ata},
            {isSigner:false,isWritable:true,pubkey:registered_nft_account[0]},
            {isSigner:false,isWritable:true,pubkey:registered_nft_account_ata},
            {isSigner:false,isWritable:true,pubkey:nft_mint},
            {isSigner:false,isWritable:true,pubkey:token_program},
            {isSigner:true,isWritable:true,pubkey:tokenized_nft_account.publicKey},
            {isSigner:false,isWritable:true,pubkey:terms_account},
            {isSigner:false,isWritable:true,pubkey:SYSVAR_RENT_PUBKEY},
            {isSigner:false,isWritable:true,pubkey:SystemProgram.programId},
            {isSigner:false,isWritable:true,pubkey:ASSOCIATED_TOKEN_PROGRAM_ID},
        ],
        data:Buffer.from([14])
    });

    const signers:Keypair[]=[temp,tokenized_nft_account];

    console.log("registered nft account "+registered_nft_account_ata.toBase58());
    console.log("buyer ata "+buyer_ata.toBase58());


    try{await sendTransaction(wallet,[ix],signers);}catch(e){ console.log(e);}

}///////////
export const tokenize_your_nft = async (nft_mint:PublicKey,wallet:WalletContextState,lamports_per_token:number,lamports_per_token_buyout:number,number_of_tokens:number ) => {//11

  const token_mint = Keypair.generate();
  const useraddresstokenmint = Keypair.generate();

  const registered_nft_account = await get_registered_nft_account_address(nft_mint);
  const token_program = await get_token_program(nft_mint);
  const owner_ata = await get_ata(wallet.publicKey!,nft_mint,false,token_program);
  const owner_tokenization_ata = await get_ata(wallet.publicKey!,token_mint.publicKey,false,TOKEN_2022_PROGRAM_ID);

  const  registered_nft_account_ata = await get_ata(registered_nft_account[0],nft_mint,true,token_program);

  const initialize = new NFTTerms();

  console.log(owner_tokenization_ata.toBase58())

  initialize.bump = registered_nft_account[1];
  initialize.lamports_per_token = BigInt(lamports_per_token*LAMPORTS_PER_SOL);
  initialize.lamports_per_token_buyout = BigInt(lamports_per_token_buyout*LAMPORTS_PER_SOL);
  initialize.number_of_tokens = BigInt(number_of_tokens);


  let encoded = serialize(NFTTermsSchema,initialize);

  let concated = Uint8Array.of(11,...encoded);

    const ix = new TransactionInstruction({
        programId:programID,
        keys:[
            {isSigner:true,isWritable:true,pubkey:wallet.publicKey!},
            {isSigner:false,isWritable:true,pubkey:owner_ata},
            {isSigner:false,isWritable:true,pubkey:owner_tokenization_ata},
            {isSigner:false,isWritable:true,pubkey:registered_nft_account[0]},
            {isSigner:false,isWritable:true,pubkey:registered_nft_account_ata},
            {isSigner:false,isWritable:true,pubkey:nft_mint},
            {isSigner:true,isWritable:true,pubkey:token_mint.publicKey},
            {isSigner:false,isWritable:true,pubkey:token_program},
            {isSigner:false,isWritable:true,pubkey:TOKEN_2022_PROGRAM_ID},
            {isSigner:false,isWritable:false,pubkey:terms_account},
            {isSigner:false,isWritable:true,pubkey:SYSVAR_RENT_PUBKEY},
            {isSigner:false,isWritable:true,pubkey:useraddresstokenmint.publicKey},
            {isSigner:false,isWritable:true,pubkey:SystemProgram.programId},
            {isSigner:false,isWritable:true,pubkey:ASSOCIATED_TOKEN_PROGRAM_ID},
        ],
        data:Buffer.from(concated)
    });
  
      const signers:Keypair[]=[token_mint,useraddresstokenmint];


try{await sendTransaction(wallet,[ix],signers);}catch(e){ console.log(e);}


}///////////
export const init_voting_to_set_new_buy_out_price = async (token_mint:PublicKey,nft_mint:PublicKey,wallet:WalletContextState,offer:number) => {//16

    const registered_nft_account = await get_registered_nft_account_address(nft_mint);
    const proposer_tokenization_ata = await get_ata(wallet.publicKey!,token_mint,false,TOKEN_2022_PROGRAM_ID);
    const proposer_pda = await get_voter_pda(wallet,token_mint);
    const vote_account_pda = await get_vote_account_pda(token_mint);

    const initialize = new StartVoting();

    initialize.offer = BigInt(offer*LAMPORTS_PER_SOL);
    initialize.proposer_pda = proposer_pda[1];
    initialize.vote_account_pda = vote_account_pda[1];

    let encoded = serialize(StartVotingSchema,initialize);

    let concated = Uint8Array.of(16,...encoded);
  
      const ix = new TransactionInstruction({
          programId:programID,
          keys:[
              {isSigner:false,isWritable:true,pubkey:wallet.publicKey!},
              {isSigner:false,isWritable:true,pubkey:proposer_tokenization_ata},
              {isSigner:false,isWritable:true,pubkey:proposer_pda[0]},
              {isSigner:false,isWritable:true,pubkey:vote_account_pda[0]},
              {isSigner:false,isWritable:true,pubkey:registered_nft_account[0]},
              {isSigner:false,isWritable:false,pubkey:terms_account},
              {isSigner:false,isWritable:true,pubkey:SystemProgram.programId},
          ],
          data:Buffer.from(concated)
      });
    
        const signers:Keypair[]=[];


try{await sendTransaction(wallet,[ix],signers);}catch(e){ console.log(e);}

}
export const repeat_voting_to_set_new_buy_out_price = async (token_mint:PublicKey,nft_mint:PublicKey,wallet:WalletContextState,offer:number) => {//21

      const registered_nft_account = await get_registered_nft_account_address(nft_mint);
      const owner_tokenization_ata = await get_ata(wallet.publicKey!,token_mint,false,TOKEN_2022_PROGRAM_ID);
      const proposer_pda = await get_voter_pda(wallet,token_mint);
      const vote_account_pda = await get_vote_account_pda(token_mint);

      const initialize = new StartVoting();

      initialize.offer = BigInt(offer*LAMPORTS_PER_SOL);
      initialize.proposer_pda = proposer_pda[1];
      initialize.vote_account_pda = vote_account_pda[1];
  
      let encoded = serialize(StartVotingSchema,initialize);
  
      let concated = Uint8Array.of(21,...encoded);
    
        const ix = new TransactionInstruction({
            programId:programID,
            keys:[
                {isSigner:false,isWritable:true,pubkey:wallet.publicKey!},
                {isSigner:false,isWritable:true,pubkey:owner_tokenization_ata},
                {isSigner:false,isWritable:true,pubkey:proposer_pda[0]},
                {isSigner:false,isWritable:true,pubkey:vote_account_pda[0]},
                {isSigner:false,isWritable:true,pubkey:registered_nft_account[0]},
            ],
            data:Buffer.from(concated)
        });
      
          const signers:Keypair[]=[];


try{await sendTransaction(wallet,[ix],signers);}catch(e){ console.log(e);}
}
export const vote = async (token_mint:PublicKey,wallet:WalletContextState,desicion:number) => {//22

      const voter_tokenization_ata = await get_ata(wallet.publicKey!,token_mint,false,TOKEN_2022_PROGRAM_ID);
      const voter_pda = await get_voter_pda(wallet,token_mint);
      const vote_account_pda = await get_vote_account_pda(token_mint);

      const vote = new VoteData();

      vote.refuse_accept = desicion;
      vote.vote_account_pda_bump = voter_pda[1];

      let encoded = serialize(VoteDataSchema,vote);

      let concated = Uint8Array.of(22,...encoded);
    
      const ix = new TransactionInstruction({
          programId:programID,
          keys:[
              {isSigner:false,isWritable:true,pubkey:wallet.publicKey!},
              {isSigner:false,isWritable:true,pubkey:voter_tokenization_ata},
              {isSigner:false,isWritable:true,pubkey:voter_pda[0]},
              {isSigner:false,isWritable:true,pubkey:vote_account_pda[0]},
          ],
          data:Buffer.from(concated)
      });
      
      const signers:Keypair[]=[];


try{await sendTransaction(wallet,[ix],signers);}catch(e){ console.log(e);}

}
export const set_new_buyout_price_after_voting = async (nft_mint:PublicKey,token_mint:PublicKey,wallet:WalletContextState ) => {//20

      const registered_nft_account = await get_registered_nft_account_address(nft_mint);
      const vote_account_pda = await get_vote_account_pda(token_mint);
    
        const ix = new TransactionInstruction({
            programId:programID,
            keys:[
                {isSigner:false,isWritable:true,pubkey:vote_account_pda[0]},
                {isSigner:false,isWritable:true,pubkey:registered_nft_account[0]},
            ],
            data:Buffer.from([20])
        });
      
          const signers:Keypair[]=[];


try{await sendTransaction(wallet,[ix],signers);}catch(e){ console.log(e);}

}
export const init_voter_account = async (token_mint:PublicKey,wallet:WalletContextState ) => {//19

      const owner_tokenization_ata = await get_ata(wallet.publicKey!,token_mint,false,TOKEN_2022_PROGRAM_ID);
      const voter_pda = await get_voter_pda(wallet,token_mint);

      const initialize = new InitAccount();
      
      const exemption = await connection.getMinimumBalanceForRentExemption(1);
      initialize.bump = voter_pda[1];
      initialize.lamports = BigInt(exemption);
    
      let encoded = serialize(InitAccountSchema,initialize);
    
      let concated = Uint8Array.of(19,...encoded);
    
        const ix = new TransactionInstruction({
            programId:programID,
            keys:[
                {isSigner:false,isWritable:true,pubkey:wallet.publicKey!},
                {isSigner:false,isWritable:true,pubkey:owner_tokenization_ata},
                {isSigner:false,isWritable:true,pubkey:voter_pda[0]},
                {isSigner:false,isWritable:true,pubkey:SystemProgram.programId},
            ],
            data:Buffer.from(concated)
        });
      
          const signers:Keypair[]=[];


try{await sendTransaction(wallet,[ix],signers);}catch(e){ console.log(e);}
      

}
export const sell_nft_owned_by_program_to_investor = async (nft_mint:PublicKey,wallet:WalletContextState ,investor:PublicKey) => {//17

    const tokenized_nft_account = Keypair.generate();
    const registered_nft_account = await get_registered_nft_account_address(nft_mint);
    const token_program = await get_token_program(nft_mint);
    const investor_ata = await get_ata(investor,nft_mint,false,token_program);
    const registered_nft_account_ata = await get_ata(registered_nft_account[0],nft_mint,true,token_program);
    const investor_account = await get_investor_account(investor);
    const proposal_account = await get_proposal_account(investor,nft_mint);
  
    const ix = new TransactionInstruction({
        programId:programID,
        keys:[
            {isSigner:false,isWritable:true,pubkey:investor},
            {isSigner:false,isWritable:true,pubkey:investor_ata},
            {isSigner:false,isWritable:true,pubkey:investor_account},
            {isSigner:false,isWritable:true,pubkey:proposal_account},
            {isSigner:false,isWritable:true,pubkey:registered_nft_account[0]},
            {isSigner:false,isWritable:true,pubkey:registered_nft_account_ata},
            {isSigner:false,isWritable:true,pubkey:nft_mint},
            {isSigner:true,isWritable:true,pubkey:tokenized_nft_account.publicKey},
            {isSigner:false,isWritable:true,pubkey:token_program},
            {isSigner:false,isWritable:false,pubkey:terms_account},
            {isSigner:false,isWritable:true,pubkey:SystemProgram.programId},
        ],
        data:Buffer.from([17])
    });

    const signers:Keypair[]=[tokenized_nft_account];


try{await sendTransaction(wallet,[ix],signers);}catch(e){ console.log(e);}

}
export const sell_nft_owned_by_individual_to_investor = async (nft_mint:PublicKey,investor:PublicKey,seller:WalletContextState) => {//18

      const token_program = await get_token_program(nft_mint);
      const investor_ata = await get_ata(investor,nft_mint,false,token_program);
      const seller_ata = await get_ata(seller.publicKey!,nft_mint,false,token_program);
      const investor_account = await get_investor_account(investor);
      const proposal_account = await get_proposal_account(investor,nft_mint);

    const ix = new TransactionInstruction({
        programId:programID,
        keys:[
            {isSigner:false,isWritable:true,pubkey:seller.publicKey!},
            {isSigner:false,isWritable:true,pubkey:seller_ata},
            {isSigner:false,isWritable:true,pubkey:investor},
            {isSigner:false,isWritable:true,pubkey:investor_ata},
            {isSigner:false,isWritable:true,pubkey:investor_account},
            {isSigner:false,isWritable:true,pubkey:proposal_account},
            {isSigner:false,isWritable:true,pubkey:nft_mint},
            {isSigner:false,isWritable:true,pubkey:token_program},
        ],
        data:Buffer.from([18])
    });

    const signers:Keypair[]=[];


    try{await sendTransaction(seller,[ix],signers);}catch(e){ console.log(e);}

}
export const make_an_offer_for_nft = async (nft_mint:PublicKey,investor:WalletContextState,amount:number) => {//15

  const investor_account = await get_investor_account(investor.publicKey!);
  const proposal_account = Keypair.generate();
  const token_program = await get_token_program(nft_mint);
  const investor_ata = await get_ata(investor.publicKey!,nft_mint,false,token_program);

  const proposal = new Lamports();

  proposal.lamports = BigInt(amount*LAMPORTS_PER_SOL)

  let encoded = serialize(LamportsSchema,proposal);

  let concated = Uint8Array.of(15,...encoded);

    const ix = new TransactionInstruction({
        programId:programID,
        keys:[
            {isSigner:true,isWritable:true,pubkey:investor.publicKey!},
            {isSigner:false,isWritable:true,pubkey:investor_ata},
            {isSigner:false,isWritable:true,pubkey:investor_account},
            {isSigner:true,isWritable:true,pubkey:proposal_account.publicKey},
            {isSigner:false,isWritable:true,pubkey:nft_mint},
            {isSigner:false,isWritable:true,pubkey:token_program},
            {isSigner:false,isWritable:false,pubkey:terms_account},
            {isSigner:false,isWritable:true,pubkey:SYSVAR_RENT_PUBKEY},
            {isSigner:false,isWritable:true,pubkey:ASSOCIATED_TOKEN_PROGRAM_ID},
            {isSigner:false,isWritable:true,pubkey:SystemProgram.programId},
        ],
        data:Buffer.from(concated)
    });
  
      const signers:Keypair[]=[proposal_account];


try{await sendTransaction(investor,[ix],signers);}catch(e){ console.log(e);}
}
export const create_investor_account = async (investor:WalletContextState,amount:number) => {//23

    const investor_account = Keypair.generate();

    const lamports = new Lamports();
    lamports.lamports = BigInt(amount*LAMPORTS_PER_SOL);

  
    let encoded = serialize(LamportsSchema,lamports);
  
    let concated = Uint8Array.of(23,...encoded);
  
      const ix = new TransactionInstruction({
          programId:programID,
          keys:[
              {isSigner:true,isWritable:true,pubkey:investor.publicKey!},
              {isSigner:true,isWritable:true,pubkey:investor_account.publicKey},
              {isSigner:false,isWritable:false,pubkey:terms_account},
              {isSigner:false,isWritable:false,pubkey:SystemProgram.programId},

          ],
          data:Buffer.from(concated)
      });
    
        const signers:Keypair[]=[investor_account];


try{await sendTransaction(investor,[ix],signers);}catch(e){ console.log(e);}
}
export const fund_investor_account = async (investor:WalletContextState,amount:number) => {//25

    const investor_account = await get_investor_account(investor.publicKey!);

    const temp = Keypair.generate();

    const lamports = new Lamports();

    lamports.lamports = BigInt(amount*LAMPORTS_PER_SOL);
  
    let encoded = serialize(LamportsSchema,lamports);
  
    let concated = Uint8Array.of(25,...encoded);
  
    const ix = new TransactionInstruction({
        programId:programID,
        keys:[
            {isSigner:true,isWritable:true,pubkey:investor.publicKey!},
            {isSigner:false,isWritable:true,pubkey:investor_account},
            {isSigner:true,isWritable:true,pubkey:temp.publicKey},
            {isSigner:false,isWritable:true,pubkey:SystemProgram.programId},
        ],
        data:Buffer.from(concated)
    });

    console.log(investor_account.toBase58())
  
      const signers:Keypair[]=[temp];


try{await sendTransaction(investor,[ix],signers);}catch(e){ console.log(e);}
}
export const remove_funds_from_investor_account = async (investor:WalletContextState,amount:number) => {//25

    const investor_account = await get_investor_account(investor.publicKey!);

    const lamports = new Lamports();

    lamports.lamports = BigInt(amount*LAMPORTS_PER_SOL);
  
    let encoded = serialize(LamportsSchema,lamports);
  
    let concated = Uint8Array.of(31,...encoded);
  
    const ix = new TransactionInstruction({
        programId:programID,
        keys:[
            {isSigner:false,isWritable:true,pubkey:investor.publicKey!},
            {isSigner:false,isWritable:true,pubkey:investor_account},

        ],
        data:Buffer.from(concated)
    });
  
      const signers:Keypair[]=[];


try{await sendTransaction(investor,[ix],signers);}catch(e){ console.log(e);}
}
export const register_nft_and_sell_to_fundraising = async (nft_mint:PublicKey,token_mint:PublicKey,wallet:WalletContextState) => {

    const registered_nft_account = await get_registered_nft_account_address(nft_mint);
    const token_program = await get_token_program(nft_mint)
    const registered_nft_account_ata = await get_ata(registered_nft_account[0],nft_mint,true,token_program);

    const initialize = new InitPDA();

    initialize.init_pda = registered_nft_account[1];

  
    let encoded = serialize(InitPDASchema,initialize);
  
    let concated = Uint8Array.of(27,...encoded);
      const ix = new TransactionInstruction({
          programId:programID,
          keys:[
              {isSigner:true,isWritable:true,pubkey:wallet.publicKey!},
              {isSigner:false,isWritable:true,pubkey:registered_nft_account[0]},
              {isSigner:false,isWritable:true,pubkey:registered_nft_account_ata},
              {isSigner:false,isWritable:false,pubkey:nft_mint},
              {isSigner:false,isWritable:false,pubkey:TOKEN_2022_PROGRAM_ID},
              {isSigner:false,isWritable:false,pubkey:terms_account},
              {isSigner:false,isWritable:true,pubkey:SystemProgram.programId},
              {isSigner:false,isWritable:true,pubkey:SYSVAR_RENT_PUBKEY},
              {isSigner:false,isWritable:true,pubkey:ASSOCIATED_TOKEN_PROGRAM_ID},
          ],
          data:Buffer.from(concated)
      });

      const fundraising_account = await get_fundraising_account(nft_mint);
      const token_dist_data = await get_distribution_account(token_mint);
      const seller_ata = await get_ata(wallet.publicKey!,nft_mint,false,token_program);
      
  
      const ix2 = new TransactionInstruction({
          programId:programID,
          keys:[
              {isSigner:false,isWritable:true,pubkey:wallet.publicKey!},
              {isSigner:false,isWritable:true,pubkey:seller_ata},
              {isSigner:false,isWritable:true,pubkey:token_dist_data[0]},
              {isSigner:false,isWritable:true,pubkey:fundraising_account[0]},
              {isSigner:false,isWritable:true,pubkey:registered_nft_account[0]},
              {isSigner:false,isWritable:true,pubkey:registered_nft_account_ata},
              {isSigner:false,isWritable:true,pubkey:nft_mint},
              {isSigner:false,isWritable:true,pubkey:token_program},
          ],
          data:Buffer.from([5])
      });
    
      const signers:Keypair[]=[];


try{await sendTransaction(wallet,[ix,ix2],signers);}catch(e){ console.log(e);}

}///////////

export const create_metadata = async () => {}

export const get_nft_mint = async () => {
//TODO verilen linke gidip nft mintini alan bir fonsiyon yaz
    const nft_mint:PublicKey = new PublicKey("");
     
    return nft_mint;
}

////////////////////////////////////////
export const create_funding_account = async (nft_mint:PublicKey,token_mint:PublicKey,wallet:WalletContextState ) => {

    const funders_account = Keypair.generate();
    const funder_tokenization_ata = await get_ata(wallet.publicKey!,token_mint,false,TOKEN_2022_PROGRAM_ID);
    const fundraising_account = await get_fundraising_account(nft_mint);

      
      const ix = new TransactionInstruction({
          programId:programID,
          keys:[
              {isSigner:true,isWritable:true,pubkey:wallet.publicKey!},
              {isSigner:false,isWritable:true,pubkey:funder_tokenization_ata},
              {isSigner:true,isWritable:true,pubkey:funders_account.publicKey},
              {isSigner:false,isWritable:true,pubkey:token_mint},
              {isSigner:false,isWritable:true,pubkey:fundraising_account[0]},
              {isSigner:false,isWritable:true,pubkey:TOKEN_2022_PROGRAM_ID},
              {isSigner:false,isWritable:true,pubkey:terms_account},
              {isSigner:false,isWritable:true,pubkey:SystemProgram.programId},
              {isSigner:false,isWritable:true,pubkey:ASSOCIATED_TOKEN_PROGRAM_ID},
          ],
          data:Buffer.from([24])
      });
    
        const signers:Keypair[]=[funders_account];


try{await sendTransaction(wallet,[ix],signers);}catch(e){ console.log(e);}

}
export const create_fundraising_account = async (nft_mint:PublicKey,wallet:WalletContextState) => {

    const fundraising_account = await get_fundraising_account(nft_mint);

    const initialize = new InitPDA();
  
    initialize.init_pda = fundraising_account[1];

    let encoded = serialize(InitPDASchema,initialize);
  
    let concated = Uint8Array.of(26,...encoded);
      
      const ix = new TransactionInstruction({
          programId:programID,
          keys:[
              {isSigner:true,isWritable:true,pubkey:wallet.publicKey!},
              {isSigner:false,isWritable:true,pubkey:fundraising_account[0]},
              {isSigner:false,isWritable:true,pubkey:nft_mint},
              {isSigner:false,isWritable:false,pubkey:terms_account},
              {isSigner:false,isWritable:false,pubkey:SystemProgram.programId},
          ],
          data:Buffer.from(concated)
      });
    
        const signers:Keypair[]=[];


try{await sendTransaction(wallet,[ix],signers);}catch(e){ console.log(e);}

}
export const register_nft_in_program = async (nft_mint:PublicKey,wallet:WalletContextState ) => {//27

    const registered_nft_account = await get_registered_nft_account_address(nft_mint);
    const token_program = await get_token_program(nft_mint)
    const registered_nft_account_ata = await get_ata(registered_nft_account[0],nft_mint,true,token_program);

    const initialize = new InitPDA();

    initialize.init_pda = registered_nft_account[1];

  
    let encoded = serialize(InitPDASchema,initialize);
  
    let concated = Uint8Array.of(27,...encoded);
      const ix = new TransactionInstruction({
          programId:programID,
          keys:[
              {isSigner:true,isWritable:true,pubkey:wallet.publicKey!},
              {isSigner:false,isWritable:true,pubkey:registered_nft_account[0]},
              {isSigner:false,isWritable:true,pubkey:registered_nft_account_ata},
              {isSigner:false,isWritable:false,pubkey:nft_mint},
              {isSigner:false,isWritable:false,pubkey:TOKEN_2022_PROGRAM_ID},
              {isSigner:false,isWritable:false,pubkey:terms_account},
              {isSigner:false,isWritable:true,pubkey:SystemProgram.programId},
              {isSigner:false,isWritable:true,pubkey:SYSVAR_RENT_PUBKEY},
              {isSigner:false,isWritable:true,pubkey:ASSOCIATED_TOKEN_PROGRAM_ID},
          ],
          data:Buffer.from(concated)
      });
    
        const signers:Keypair[]=[];


try{await sendTransaction(wallet,[ix],signers);}catch(e){ console.log(e);}
}
export const buy_nft_with_the_funds_cpi_to = async () => {

  

}
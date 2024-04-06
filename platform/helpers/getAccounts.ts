
import { unpack } from '@solana/spl-token-metadata';
import { WalletContextState } from '@solana/wallet-adapter-react';
import { deserialize } from 'borsh';
import { TOKEN_PROGRAM_ID, TOKEN_2022_PROGRAM_ID, getMint, Mint } from '@solana/spl-token';
import { Connection, PublicKey, AccountInfo, TokenAccountsFilter, Commitment } from '@solana/web3.js';
import {
  ActiveFundRaising, DistData, DistDataSchema, FundRaising, FundRaisingSchema, FunderAccount, FunderAccountSchema, InitPDA, InitPDASchema,
  MyFundRaising,
  NFT,
  NFTTerms, NFTTermsSchema, NFTonSale, SolToken, SolTokenSchema, UserAddresTokenMint, UserAddresTokenMintSchema
} from './model';
import { metaplex_program, programID } from './key';
import { deserialize_fundraising_data, deserialize_nft_metadata, deserialize_nftterms_data } from './utils';
var BASE58 = '123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz';
var bs58 = require('base-x')(BASE58);



//const connection = new Connection(clusterApiUrl("testnet"))
const connection = new Connection("http://localhost:8899", "confirmed");

export const get_registered_nft_account_data = async (nft_mint: PublicKey) => {

  const pda = PublicKey.findProgramAddressSync([nft_mint.toBytes()], programID);

  const pda_acc_info = await connection.getAccountInfo(pda[0]);

  const decoded_data = deserialize(NFTTermsSchema, NFTTerms, pda_acc_info?.data!);

  return decoded_data;

}
export const get_token_program = async (nft_mint: PublicKey) => {

  const mint = await connection.getAccountInfo(nft_mint)
  const owner = mint?.owner!;

  return owner;
}
export const get_owner = async (registered_nft_account: PublicKey) => {

  const registered_nft_account_info = await connection.getAccountInfo(registered_nft_account)
  const data: NFTTerms = deserialize(NFTTermsSchema, NFTTerms, registered_nft_account_info!.data);
  const seller_from_bytes = bs58.encode(data.owner);
  const seller = new PublicKey(seller_from_bytes);

  return seller;
}
export const get_funders_account = async (wallet: WalletContextState, token_mint: PublicKey, nft_mint: PublicKey) => {

  const account = await connection.getProgramAccounts(
    programID,
    {
      filters: [
        {
          dataSize: 112,
        },
        {
          memcmp: {
            offset: 0,
            bytes: wallet.publicKey!.toString(),
          },
        },
        {
          memcmp: {
            offset: 32,
            bytes: nft_mint.toString(),
          },
        },
        {
          memcmp: {
            offset: 64,
            bytes: token_mint.toString(),
          },
        },
      ],
    }
  );


  return account[0].pubkey;

}
export const get_TokentoSol_account = async (token_mint: PublicKey) => {

  const accounts = await connection.getProgramAccounts(
    programID,
    {
      filters: [
        {
          dataSize: 56,
        },
        {
          memcmp: {
            offset: 0,
            bytes: token_mint.toString(),
          },
        },
      ],
    }
  );
  const data: SolToken = deserialize(SolTokenSchema, SolToken, accounts[0].account.data!);

  console.log("lamports_per_token " + data.lamports_per_token);
  console.log("number_of_tokens " + data.number_of_tokens);
  console.log("tokens_sold " + data.tokens_sold);

  return accounts[0].pubkey;
}
export const get_proposal_account = async (investor: PublicKey, nft_mint: PublicKey) => {

  const accounts = await connection.getProgramAccounts(
    programID,
    {
      filters: [
        {
          dataSize: 72,
        },
        {
          memcmp: {
            offset: 0,
            bytes: investor.toString(),
          },
        },
        {
          memcmp: {
            offset: 32,
            bytes: nft_mint.toString(),
          },
        },
      ],
    }
  );

  return accounts[0].pubkey;

}
export const get_active_fundraisings = async () => {

  const one = bs58.encode([1]);

  const fundraisings: ActiveFundRaising[] = [];

  const accounts = await connection.getProgramAccounts(
    programID,
    {
      filters: [
        {
          dataSize: 90,
        },
        {
          memcmp: {
            offset: 0,
            bytes: one,
          },
        },
      ],
    }
  );

  for (let index = 0; index < accounts.length; index++) {
    const fundr = await deserialize_fundraising_data(accounts[index].account.data);
    const fundraising = new ActiveFundRaising();
    fundraising.nft_mint = new PublicKey(bs58.encode(fundr.nft_mint).toString());
    fundraising.tokens_mint = new PublicKey(bs58.encode(fundr.tokens_mint).toString());
    fundraising.bump = fundr.bump;
    fundraising.funds_collected = fundr.funds_collected;
    fundraising.number_of_tokens = fundr.number_of_tokens;
    fundraising.lamports_per_token = fundr.lamports_per_token;
    fundraisings.push(fundraising);
  }

  return fundraisings;
}
export const get_tokenized_nfts_on_sale = async () => {

  const accounts: NFTonSale[] = [];

  const one = bs58.encode([1]);

  const accounts1 = await connection.getProgramAccounts(
    programID,
    {
      filters: [
        {
          dataSize: 150,
        },
        {
          memcmp: {
            offset: 97,
            bytes: one,
          },
        },
      ],
    }
  );

  for (let index = 0; index < accounts1.length; index++) {
    const nft = await deserialize_nftterms_data(accounts1[index].account.data);
    const nftOnsale = new NFTonSale();
    nftOnsale.price = nft.buy_out_price;
    nftOnsale.owner = new PublicKey(bs58.encode(nft.owner).toString());
    nftOnsale.nft_mint = new PublicKey(bs58.encode(nft.nft_mint).toString());
    nftOnsale.tokenization_mint = new PublicKey(bs58.encode(nft.tokenization_mint).toString());
    nftOnsale.for_sale = nft.for_sale;
    nftOnsale.buy_out_allowed = nft.buy_out_allowed;
    nftOnsale.bump = nft.bump;
    nftOnsale.lamports_per_token_buyout = nft.lamports_per_token_buyout;
    nftOnsale.number_of_tokens = nft.number_of_tokens;
    nftOnsale.lamports_per_token = nft.lamports_per_token;
    nftOnsale.tokens_sold = nft.tokens_sold;
    accounts.push(nftOnsale);
  }

  return accounts;

}
export const get_nfts_on_sale = async () => {

  const one = bs58.encode([1]);
  const accounts: NFTonSale[] = [];


  const accounts1 = await connection.getProgramAccounts(
    programID,
    {
      filters: [
        {
          dataSize: 150,
        },
        {
          memcmp: {
            offset: 98,
            bytes: one,
          },
        },
      ],
    }
  );

  const accounts2 = await connection.getProgramAccounts(
    programID,
    {
      filters: [
        {
          dataSize: 150,
        },
        {
          memcmp: {
            offset: 96,
            bytes: one,
          },
        },
      ],
    }
  );

  for (let index = 0; index < accounts1.length; index++) {
    const nft = await deserialize_nftterms_data(accounts1[index].account.data);
    const nftOnsale = new NFTonSale();
    nftOnsale.price = nft.price;
    nftOnsale.owner = new PublicKey(bs58.encode(nft.owner).toString());
    nftOnsale.nft_mint = new PublicKey(bs58.encode(nft.nft_mint).toString());
    nftOnsale.tokenization_mint = new PublicKey(bs58.encode(nft.tokenization_mint).toString());
    nftOnsale.for_sale = nft.for_sale;
    nftOnsale.buy_out_allowed = nft.buy_out_allowed;
    nftOnsale.bump = nft.bump;
    accounts.push(nftOnsale)

  }
  for (let index = 0; index < accounts2.length; index++) {
    const nft = await deserialize_nftterms_data(accounts2[index].account.data);
    const nftOnsale = new NFTonSale();
    nftOnsale.price = nft.price;
    nftOnsale.owner = new PublicKey(bs58.encode(nft.owner).toString());
    nftOnsale.nft_mint = new PublicKey(bs58.encode(nft.nft_mint).toString());
    nftOnsale.tokenization_mint = new PublicKey(bs58.encode(nft.tokenization_mint).toString());
    nftOnsale.for_sale = nft.for_sale;
    nftOnsale.buy_out_allowed = nft.buy_out_allowed;
    nftOnsale.bump = nft.bump;
    accounts.push(nftOnsale)
  }

  return accounts;
}
export const get_user_token = async (wallet: WalletContextState) => {

  const user_tokens = await connection.getProgramAccounts(
    programID,
    {
      filters: [
        {
          dataSize: 64,
        },
        {
          memcmp: {
            offset: 0,
            bytes: wallet.publicKey!.toString(),
          },
        },
      ],
    }
  );

  return user_tokens[0].account;
}
export const get_offers_for_nfts = async () => {

  const accounts = await connection.getProgramAccounts(
    programID,
    {
      filters: [
        {
          dataSize: 72,
        },
      ],
    }
  );

  return accounts[0].account;
}
export const get_my_tokenized_nfts_on_sale = async (wallet: WalletContextState) => {

  const one = bs58.encode([1]);

  const accounts = await connection.getProgramAccounts(
    programID,
    {
      filters: [
        {
          dataSize: 150,
        },
        {
          memcmp: {
            offset: 0,
            bytes: wallet.publicKey!.toString(),
          },
        },
        {
          memcmp: {
            offset: 97,
            bytes: one,
          },
        },
      ],
    }
  );
  return accounts;
}//tokenize halde satilan nftyi gosterir
//kullaniciya satisi iptal et butonu gosterilir 
//nftnini yuzde kaci satildi gibi bilgiler gosterilir
export const get_my_nfts_on_sale = async (wallet: WalletContextState) => {

  const one = bs58.encode([1]);

  const accounts = await connection.getProgramAccounts(
    programID,
    {
      filters: [
        {
          dataSize: 150,
        },
        {
          memcmp: {
            offset: 0,
            bytes: wallet.publicKey!.toString(),
          },
        },
        {
          memcmp: {
            offset: 96,
            bytes: one,
          },
        },
      ],
    }
  );
  return accounts;
}//kullanicinin pesin sattigi nftyi gosterir
export const get_my_fundraisings = async (wallet: WalletContextState) => {

  const funder_accounts = await connection.getProgramAccounts(
    programID,
    {
      filters: [
        {
          dataSize: 112,
        },
        {
          memcmp: {
            offset: 0,
            bytes: wallet.publicKey!.toString(),
          },
        },
      ],
    }
  );

  let myfundraisings: MyFundRaising[] = [];

  for (let index = 0; index < funder_accounts.length; index++) {

    const funder: FunderAccount = deserialize(FunderAccountSchema, FunderAccount, funder_accounts[index].account.data);

    const token_mint = new PublicKey(bs58.encode(funder.tokens_mint).toString());

    const funders_account_pubkey = funder_accounts[index].pubkey;

    const tokens_to_receive = funder.fund_invested / funder.lamports_per_token;

    try {

      const fundraising = await connection.getProgramAccounts(
        programID,
        {
          filters: [
            {
              dataSize: 90,
            },
            {
              memcmp: {
                offset: 33,
                bytes: token_mint.toString(),
              },
            },
          ],
        }
      );

      const distribution = await connection.getProgramAccounts(
        programID,
        {
          filters: [
            {
              dataSize: 42,
            },
            {
              memcmp: {
                offset: 0,
                bytes: token_mint.toString(),
              },
            },
          ],
        }
      );

      const fund: FundRaising = deserialize(FundRaisingSchema, FundRaising, fundraising[0].account.data);
      const dist: DistData = deserialize(DistDataSchema, DistData, distribution[0].account.data);

      if (dist.distribution_open == 1) {

        const fundraising = new MyFundRaising();
        fundraising.nft_mint = new PublicKey(bs58.encode(fund.nft_mint).toString());
        fundraising.tokens_mint = new PublicKey(bs58.encode(fund.tokens_mint).toString());
        fundraising.funders_account = funders_account_pubkey;
        fundraising.funds_invested = funder.fund_invested;
        fundraising.tokens_to_receive = tokens_to_receive;
        fundraising.bump = fund.bump;
        fundraising.funds_collected = fund.funds_collected;
        fundraising.number_of_tokens = fund.number_of_tokens;
        fundraising.lamports_per_token = fund.lamports_per_token;
        fundraising.fund_raising_succesful = true;
        myfundraisings.push(fundraising);
      } else {
        const fundraising = new MyFundRaising();
        fundraising.nft_mint = new PublicKey(bs58.encode(fund.nft_mint).toString());
        fundraising.tokens_mint = new PublicKey(bs58.encode(fund.tokens_mint).toString());
        fundraising.funders_account = funders_account_pubkey;
        fundraising.funds_invested = funder.fund_invested;
        fundraising.tokens_to_receive = tokens_to_receive;
        fundraising.bump = fund.bump;
        fundraising.funds_collected = fund.funds_collected;
        fundraising.number_of_tokens = fund.number_of_tokens;
        fundraising.lamports_per_token = fund.lamports_per_token;
        fundraising.fund_raising_succesful = false;
        myfundraisings.push(fundraising);
      }
    } catch (e) { }

  }


  return myfundraisings[0];
}//returns ongoing and succesful fundraising
//from succesful fundraising user can claim tokenized assets
//on ongoing fundraises return add or remove Sol button
export const get_my_tokenized_nfts = async (wallet: WalletContextState) => {

  const user_tokens = await connection.getProgramAccounts(
    programID,
    {
      filters: [
        {
          dataSize: 64,
        },
        {
          memcmp: {
            offset: 0,
            bytes: wallet.publicKey!.toString(),
          },
        },
      ],
    }
  );
  let token_mints: PublicKey[] = [];
  for (let index = 0; index < user_tokens.length; index++) {
    const usertoken: UserAddresTokenMint = deserialize(UserAddresTokenMintSchema, UserAddresTokenMint, user_tokens[index].account.data);
    const token_mint = new PublicKey(bs58.encode(usertoken.token_mint).toString());
    token_mints.push(token_mint);
  }

  let token_mints_uniqe: PublicKey[] = Array.from(new Set(token_mints));

  let nfts: NFTTerms[] = [];

  for (let index = 0; index < token_mints_uniqe.length; index++) {

    const accounts = await connection.getProgramAccounts(
      programID,
      {
        filters: [
          {
            dataSize: 150,
          },
          {
            memcmp: {
              offset: 64,
              bytes: token_mints_uniqe[index].toString(),
            },
          },
        ],
      }
    );

    const nft: NFTTerms = deserialize(NFTTermsSchema, NFTTerms, accounts[0].account.data);

    nfts.push(nft);

  }

  const token = new PublicKey(bs58.encode(nfts[0].tokenization_mint).toString());
  const nft = new PublicKey(bs58.encode(nfts[0].nft_mint).toString());


  return [token, nft];
}//user gets tokenized nfts of which he has shares 
export const get_active_votings = async (token_mint: PublicKey) => {

  const accounts = await connection.getProgramAccounts(
    programID,
    {
      filters: [
        {
          dataSize: 65,
        },
        {
          memcmp: {
            offset: 0,
            bytes: token_mint.toBase58(),
          },
        },
      ],
    }
  );

  return accounts;

}//first get_my_tokenized_nfts function is called and returns tokenized nfts of which user has shares
// if there are active votings  and user clicks on it  get_active_votings function is called
export const get_my_vote_info = async (token_mint: PublicKey, wallet: WalletContextState) => {

  const vote_account = PublicKey.findProgramAddressSync([token_mint.toBytes(), wallet.publicKey!.toBytes()!], programID);

  const vote_account_info = await connection.getAccountInfo(vote_account[0]);

  let vote_no = 0;

  if (vote_account_info?.owner.toString() == programID.toString()) {

  } else {
    const vote_account_data: InitPDA = deserialize(InitPDASchema, InitPDA, vote_account_info?.data!);

    vote_no = vote_account_data.init_pda;
  }
  return vote_no;
}// there are no vote accounts return vote button(accept refuse) - if vote account vote no is not equal to vote no return vote button(accept refuse).
// if vote account vote no is equal to vote no already voted
export const get_my_investor_account = async (wallet: WalletContextState) => {

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
            bytes: wallet.publicKey!.toString(),
          },
        },
      ],
    }
  );

  return accounts;

}
export const get_my_offers = async (wallet: WalletContextState) => {

  const accounts = await connection.getProgramAccounts(
    programID,
    {
      filters: [
        {
          dataSize: 72,
        },
        {
          memcmp: {
            offset: 0,
            bytes: wallet.publicKey!.toString(),
          },
        },
      ],
    }
  );
  return accounts;
}//if user has investor account get_my_offers function is called
export const get_my_nfts = async (wallet: PublicKey, token_program_id: PublicKey) => {

  const mints: PublicKey[] = [];
  const nft_mint_infos: Mint[] = [];

  let filter: TokenAccountsFilter = { programId: token_program_id }
  let token_accounts = await connection.getParsedTokenAccountsByOwner(wallet, filter)
  for (let index = 0; index < token_accounts.value.length; index++) {
    const parsedAccountInfo: any = token_accounts.value[index].account.data;
    const mintAddress: string = parsedAccountInfo["parsed"]["info"]["mint"];
    const tokenBalance: number = parsedAccountInfo["parsed"]["info"]["tokenAmount"]["uiAmount"];
    if (tokenBalance == 1) {
      const mint = new PublicKey(mintAddress);
      mints.push(mint);
    }
  }

  const commitment: Commitment = "confirmed";

  for (let index = 0; index < mints.length; index++) {
    const m = await getMint(connection, mints[index], commitment, token_program_id);
    if (m.decimals == 0 && m.supply == BigInt(1) && m.isInitialized) { nft_mint_infos.push(m); }
  }



  return nft_mint_infos;
}//returns nft mintinfo for given token program
export const get_metaplex_metadata = async (wallet: WalletContextState) => {

  const mints = await get_my_nfts(wallet.publicKey!, TOKEN_PROGRAM_ID);

  const nfts: NFT[] = [];

  for (let index = 0; index < mints.length; index++) {
    const accounts = await connection.getProgramAccounts(
      metaplex_program,
      {
        filters: [
          {
            memcmp: {
              offset: 33,
              bytes: mints[index].address.toString(),
            },
          },
        ],
      }
    );
    if (accounts.length == 1) {
      const nft = await deserialize_nft_metadata(accounts[0].account.data);
      nfts.push(nft);
    }
  }


  return nfts;
}

export const get_token2022_metadata = async (wallet: WalletContextState) => {

  const mints = await get_my_nfts(wallet.publicKey!, TOKEN_2022_PROGRAM_ID);

  const nfts: NFT[] = [];

  //TODO get nft metadata


  return nfts;
}
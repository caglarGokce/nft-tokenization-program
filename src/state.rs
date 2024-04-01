use borsh::{BorshDeserialize, BorshSerialize};



#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq)]
pub struct NFTState{
    pub owner:[u8;32],//Satici addresi - Satildiktan sonra pda adresi olur
    pub nft_mint:[u8;32],// nftnin mint addresi
    pub tokenization_mint:[u8;32], //tokenize olmus nftnin mint addresi
    pub for_sale:u8,
    pub tokenized_for_sale:u8, // nft tokenize edilerek satiliyor. Tokenlar satildikca ownera satis tutari gonderilecek.
    pub buy_out_allowed:u8,
    pub owned_by_pda:u8,
    pub price:u64, //kaca satiliyor
    pub buy_out_price:u64, //kaca satilacak / hemen satin al rakami.
    pub lamports_per_token_buyout:u64, // belli bir fee odeyenleri ana sayfada gosterebiliriz
    pub number_of_tokens:u64, //nft kac tokena bolundu
    pub lamports_per_token:u64, //bir hisse kaca satiliyor = price_bought/tokenization_share
    pub tokens_sold:u64, // kac token satildi if share_sold == tokenization_share transfer tokens to pda
    pub bump:u8,
    pub vote_open:u8
}//147

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct UserAddresTokenMint{
    pub user:[u8;32],
    pub mint:[u8;32],
}//64

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq)]
pub struct FundRaising{
    pub fund_raising:u8,
    pub nft_mint:[u8;32],// nftnin mint addresi
    pub tokens_mint:[u8;32],
    pub funds_collected:u64,
    pub number_of_tokens:u64, //nft kac tokena bolundu
    pub lamports_per_token:u64, //bir hisse kaca satiliyor = price_bought/tokenization_share
    pub bump:u8
}//90

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct DistData{
    pub token_mint:[u8;32],
    pub distribution_open:u8,
    pub tokens_left:u64,
    pub bump:u8
}//42

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct FunderAccount{
    pub funder:[u8;32],
    pub nft_mint:[u8;32],
    pub tokens_mint:[u8;32],
    pub fund_invested:u64,
    pub lamports_per_token:u64,
}//104

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq)]
pub struct InitAccount{
    pub bump:u8,
    pub lamports:u64,
    pub size:u64,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct TokenToSol{
    pub tokenization_mint:[u8;32],
    pub number_of_tokens:u64, 
    pub lamports_per_token:u64,
    pub tokens_sold:u64,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct VoteAccount{
    pub tokenization_mint:[u8;32],
    pub new_buy_out_price_accept_votes:u64,
    pub new_buy_out_price_refuse_votes:u64,
    pub voting_ends:u64,
    pub new_buy_out_offer:u64,
    pub voting_no:u8,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq)]
pub struct InitPDA{
    pub init_pda:u8, 
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Proposal{
    pub proposer:[u8;32],
    pub nft_mint:[u8;32],
    pub offer:u64,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq)]
pub struct Lamports{
    pub lamports:u64,//&number of tokens to buy for instruction 9
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq)]
pub struct StartVoting{
    pub offer:u64,
    pub proposer_pda:u8,
    pub vote_account_pda:u8,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq)]
pub struct VoteData{
    pub refuse_accept:u8,
    pub vote_account_pda_bump:u8,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct InvestorAccount{
    pub investor:[u8;32],
    pub lamports:u64,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq)]
pub struct Terms{
    pub is_init:u8,
    pub funder_account:u64,
    pub funder_account_size:u64,
    pub fundrasing_account:u64,
    pub fundrasing_account_size:u64,
    pub nft_pda_account:u64,
    pub nft_pda_account_size:u64,
    pub vote_account:u64,
    pub vote_account_size:u64,
    pub proposal_account:u64,
    pub proposal_account_size:u64,
    pub mint:u64,
    pub mint_size:u64,
    pub token_distribution_account:u64,
    pub token_distribution_account_size:u64,
    pub usertokenmint_account:u64,
    pub usertokenmint_account_size:u64,//64
    pub token_to_sol_account:u64,
    pub token_to_sol_account_size:u64,//56
    pub investor_account:u64,
    pub investor_account_size:u64,
    pub lamports_per_token_fundraising:u64,
    pub minimum_lamports_per_token:u64,
    pub buy_sell_order_account:u64,
    pub buy_sell_order_account_size:u64,
    pub small_account:u64,

}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct SellOrder{
    pub is_init:u8,
    pub seller:[u8;32],
    pub tokenization_mint:[u8;32],
    pub price_per_token:u64,
    pub amount_to_sell:u64,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct BuyOrder{
    pub is_init:u8,
    pub buyer:[u8;32],
    pub tokenization_mint:[u8;32],
    pub price_per_token:u64,
    pub amount_to_buy:u64,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq)]
pub struct SellToken{
    pub price_per_token:u64,
    pub amount_to_sell:u64,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq)]
pub struct BuyToken{
    pub price_per_token:u64,
    pub amount_to_buy:u64,
}
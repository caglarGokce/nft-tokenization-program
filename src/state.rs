use borsh::{BorshDeserialize, BorshSerialize};


#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq)]
pub struct NFTTerms{
    pub mint:String,//Tokenize olmus nftnin mint addresi
    pub mint_address_length:u8,
    pub tokenization_mint_address:String, //tokenize olmus nftnin mint addresi
    pub tokenization_mint_address_length:u8,
    pub price_currency:u8,//satisldigi varlik cinsi SOL?? USDC??
    pub price_bought:u8, //kaca alindi
    pub price_to_sell:u64, //kaza sailacak
    pub list_in_main_page:String, // belli bir fee odeyenleri ana sayfada gosterebiliriz
    pub tokenization_share:u64, //nft kac tokena bolundu
    pub liquidate:u8 ,// nft likide edilebilir mi
    pub liquidation_vote_number:u64,
    pub liquidation_vote_majority_treshold:u64,
    //other terms
    //
    //
    //
}

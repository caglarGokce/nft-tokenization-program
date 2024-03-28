import React, { FC, useEffect, useState } from 'react';
import { ActiveFundRaising,  MyFundRaising,  NFT,  NFTonSale, NFTTerms } from './model';
import { add_funds_to_fudnraising as add_funds_to_fundraising, buy_out_tokenized_nft, buy_part_of_tokenized_nft_from_this_program, buy_whole_nft_from_this_program, get_nft_mint, get_tokenized_assets_from_successfull_fundraising, join_fund_raising_to_buy_nft, list_nft_forsale_as_whole_in_this_program, remove_funds_from_the_fundraising, start_fund_raising_to_buy_nft, tokenize_nft_and_sell_in_this_program, tokenize_your_nft } from './service';
import { get_active_fundraisings, get_metaplex_metadata, get_my_fundraisings, get_my_nfts, get_nfts_on_sale, get_token2022_metadata, get_tokenized_nfts_on_sale } from './getAccounts';
import { WalletContextState } from '@solana/wallet-adapter-react';
import { deserialize_nft_metadata, get_fundraising_account } from './utils';
import { PublicKey } from '@metaplex-foundation/js';
import { TOKEN_PROGRAM_ID } from '@solana/spl-token';



interface MyNFTsProps {
    wallet: WalletContextState; 
}

const Fundraisings: FC<MyNFTsProps> = ({ wallet }) => {

    const [data, setData] = useState<NFT[] | null>(null); 
    let [price, setPrice] = useState(0);
    let [lamports_per_token, setLamportsPerToken] = useState(0);
    let [lamports_per_token_buyout, setLamportsPerTokenBuyout] = useState(0);
    let [divide_into, setDivideInto] = useState(0);

    useEffect(() => {

        const getAccounts = async () => {
            try {
                const nfts_metadata = await get_metaplex_metadata(wallet);
                const nfts_2022 = await get_token2022_metadata(wallet);
                const nfts = nfts_metadata.push(...nfts_2022);
                if (Array.isArray(nfts)) {
                    setData(nfts);
                } else {
                    setData([]);
                }
            } catch (error) {
                console.error('Error fetching data:', error);
                setData([]);
            }
        };

        getAccounts();

    }, [data]);


    function setThePrice(e: any)
    {
        setPrice(Number(e.target.value));
        price = e.target.value;
    }

    function setTheLamportsPerToken(e: any)
    {
        setLamportsPerToken(Number(e.target.value));
        lamports_per_token = e.target.value;
    }

    function setTheLamportsPerTokenBuyout(e: any)
    {
        setLamportsPerTokenBuyout(Number(e.target.value));
        lamports_per_token_buyout = e.target.value;
    }

    function setDivision(e: any)
    {
        setDivideInto(Number(e.target.value));
        divide_into = e.target.value;
    }

    const handleSellNFTasWhole = async (nft:NFT) => {
        await list_nft_forsale_as_whole_in_this_program(nft.nft_mint,wallet,price,lamports_per_token_buyout*divide_into);
    };

    const handleTokenizeNFTandSell = async (nft:NFT) => {
        await tokenize_nft_and_sell_in_this_program(nft.nft_mint,wallet,lamports_per_token,divide_into,lamports_per_token_buyout)
    };

    const handleTokenizeNFT = async (nft:NFT) => {
        await tokenize_your_nft(nft.nft_mint,wallet,lamports_per_token,lamports_per_token_buyout,divide_into)
    };


    return (
        <><div>
            <h1>New Page</h1>
            {data !== null ? (
                data.length > 0 ? (
                    <div>
                        {data.map((nft: NFT, index) => (
                            <div key={index} className="card">
                                <div className="card-body">
                                    <h5 className="card-title">My ongoing FundRaisings</h5>
                                    <p>metadata: {nft.url}</p>
                                    <p>mint: {nft.nft_mint}</p>
                                    <button onClick={()=>handleSellNFTasWhole(nft)} >Sell NFT</button>
                                    <input value={price} type="number" onChange={(e) => setThePrice(e)}></input>
                                    <button onClick={()=>handleTokenizeNFTandSell(nft)} >Tokenize and Sell NFT</button>
                                    <button onClick={()=>handleTokenizeNFT(nft)} >Tokenize NFT</button>
                                    <input value={lamports_per_token} type="number" onChange={(e) => setTheLamportsPerToken(e)}>price per token</input>
                                    <input value={lamports_per_token_buyout} type="number" onChange={(e) => setTheLamportsPerTokenBuyout(e)}>buy out price per token</input>
                                    <input value={divide_into} type="number" onChange={(e) => setDivision(e)}>divide into</input>
                                    
                                </div>
                            </div>
                        ))}
                    </div>
                ) : (
                    <p>There is no ongoing fundraising</p>
                )
            ) : (
                <p>Loading...</p>
            )}
        </div>
        </>
    );
};

export default Fundraisings;

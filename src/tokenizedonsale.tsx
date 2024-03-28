import React, { FC, useEffect, useState } from 'react';
import { NFTonSale, NFTTerms } from './model';
import { buy_out_tokenized_nft, buy_part_of_tokenized_nft_from_this_program, buy_whole_nft_from_this_program } from './service';
import { get_nfts_on_sale, get_tokenized_nfts_on_sale } from './getAccounts';
import { WalletContextState } from '@solana/wallet-adapter-react';



interface MyTokenizedNFTsProps {
    wallet: WalletContextState; 
}

const MyTokenizedNFTs: FC<MyTokenizedNFTsProps> = ({ wallet }) => {


    const [data, setData] = useState<NFTonSale[] | null>(null); 
    let [numberoftokens, setNumberofTokens] = useState(0);

    useEffect(() => {

        const getAccounts = async () => {
            try {
                const nfts = await get_tokenized_nfts_on_sale();
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


    function setAmounttoBuy(e: any)
    {
        setNumberofTokens(Number(e.target.value));
        numberoftokens = e.target.value;
    }

    const handleBuyOutNFT = async (nft:NFTonSale) => {

        buy_out_tokenized_nft(nft.nft_mint,nft.tokenization_mint,wallet)
        
    };

    const handleBuyTokenizedAsset= async (nft:NFTonSale) => {
        buy_part_of_tokenized_nft_from_this_program(nft.nft_mint,nft.tokenization_mint,wallet,numberoftokens)
        
    };

    return (
        <div>
            <h1>New Page</h1>
            {data !== null ? (
                data.length > 0 ? (
                    <div>
                        {data.map((nft: NFTonSale, index) => (
                            <div key={index} className="card">
                                <div className="card-body">
                                    <h5 className="card-title">TOKENIZED NFT</h5>
                                    <p>buy_out: {nft.price}</p>
                                    <p>divided into: {nft.number_of_tokens}</p>
                                    <p>sold: {nft.tokens_sold}</p>
                                    <p>Price per token: {nft.lamports_per_token}</p>
                                    <button onClick={()=>handleBuyOutNFT(nft)} >Buy Out</button>
                                    <button onClick={()=>handleBuyTokenizedAsset(nft)} >Buy Tokenized Shares</button>
                                    <input value={numberoftokens} type="number" onChange={(e) => setAmounttoBuy(e)}></input>
                                </div>
                            </div>
                        ))}
                    </div>
                ) : (
                    <p>There is no nft</p>
                )
            ) : (
                <p>Loading...</p>
            )}
        </div>
    );
};

export default MyTokenizedNFTs;

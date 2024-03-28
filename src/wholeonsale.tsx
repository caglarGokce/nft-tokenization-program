import React, { FC, useEffect, useState } from 'react';
import { NFTonSale, NFTTerms } from './model';
import { buy_out_tokenized_nft, buy_whole_nft_from_this_program } from './service';
import { get_nfts_on_sale } from './getAccounts';
import { WalletContextState } from '@solana/wallet-adapter-react';



interface MyTokenizedNFTsProps {
    wallet: WalletContextState; 
}

const MyTokenizedNFTs: FC<MyTokenizedNFTsProps> = ({ wallet }) => {


    const [data, setData] = useState<NFTonSale[] | null>(null); 

    useEffect(() => {

        const getAccounts = async () => {
            try {
                const nfts = await get_nfts_on_sale();
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

    const handleBuyNFT = async (nft:NFTonSale) => {
        if(nft.for_sale){
            buy_whole_nft_from_this_program(nft.nft_mint,wallet)
        }else{
            buy_out_tokenized_nft(nft.nft_mint,nft.tokenization_mint,wallet)
        }
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
                                    <h5 className="card-title">price: {nft.price}</h5>
                                    <button onClick={()=>handleBuyNFT(nft)} >Buy</button>
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

import React, { FC, useEffect, useState } from 'react';
import { NFTonSale, NFTTerms } from './model';
import { vote, init_voting_to_set_new_buy_out_price } from './service';
import { get_nfts_on_sale } from './getAccounts';

interface MyTokenizedNFTsProps {
    // Define prop types if needed
}

const MyTokenizedNFTs: FC<MyTokenizedNFTsProps> = ({}) => {
    const [data, setData] = useState<NFTonSale[] | null>(null); // State to hold the fetched data

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
                setData(null);
            }
        };

        getAccounts();
        
    }, [data]);

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

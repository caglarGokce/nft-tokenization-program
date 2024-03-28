import React, { FC, useEffect, useState } from 'react';
import { ActiveFundRaising,  NFTonSale, NFTTerms } from './model';
import { buy_nft_listed_in_program_with_the_funds, buy_out_tokenized_nft, buy_part_of_tokenized_nft_from_this_program, buy_whole_nft_from_this_program, get_nft_mint, join_fund_raising_to_buy_nft, register_nft_and_sell_to_fundraising, start_fund_raising_to_buy_nft } from './service';
import { get_active_fundraisings, get_nfts_on_sale, get_registered_nft_account_data, get_tokenized_nfts_on_sale } from './getAccounts';
import { WalletContextState } from '@solana/wallet-adapter-react';
import { get_fundraising_account } from './utils';
import { PublicKey } from '@metaplex-foundation/js';



interface FundraisingsProps {
    wallet: WalletContextState; 
}

const Fundraisings: FC<FundraisingsProps> = ({ wallet }) => {


    const [data, setData] = useState<ActiveFundRaising[] | null>(null); 
    let [amount, setAmount] = useState(0);

    useEffect(() => {

        const getAccounts = async () => {
            try {
                const nfts = await get_active_fundraisings();
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
        setAmount(Number(e.target.value));
        amount = e.target.value;
    }

    const handleJoinFundRaising = async (nft:ActiveFundRaising) => {
        join_fund_raising_to_buy_nft(nft.tokens_mint,nft.nft_mint,wallet,amount)//TODO amount nft.lamports_per_token katlari olmali
    };

    const handleStartFundraising= async () => {
        const nft_mint:PublicKey = await get_nft_mint();//TODO verilen linkten nft mintini getirecek bir fonksiyon yaz
        start_fund_raising_to_buy_nft(nft_mint,wallet,amount)
    };

    const handleSellNFTtoFundRaising= async (nft:ActiveFundRaising) => {
        try{
            const reg_nft_account = await get_registered_nft_account_data(nft.nft_mint);

            if (reg_nft_account.for_sale){
                try {
                await buy_nft_listed_in_program_with_the_funds(nft.nft_mint,nft.tokens_mint,wallet);
                    
                } catch (error) {
                    
                }
            }
        }catch(e){
            await register_nft_and_sell_to_fundraising(nft.nft_mint,nft.tokens_mint,wallet);
        }

    };

    return (
        <><div>
            <h1>New Page</h1>
            {data !== null ? (
                data.length > 0 ? (
                    <div>
                        {data.map((nft: ActiveFundRaising, index) => (
                            <div key={index} className="card">
                                <div className="card-body">
                                    <h5 className="card-title">FundRaisings</h5>
                                    <p>divided into: {nft.number_of_tokens}</p>
                                    <p>Price per token: {nft.lamports_per_token}</p>
                                    <button onClick={() => handleJoinFundRaising(nft)}>Join FundRaising</button>
                                    <button onClick={() => handleSellNFTtoFundRaising(nft)}>Sell NFT to the Fundraising</button>
                                    <input value={amount} type="number" onChange={(e) => setAmounttoBuy(e)}></input>
                                </div>
                            </div>
                        ))}
                    </div>
                ) : (
                    <p>There is no active fundraising</p>
                )
            ) : (
                <p>Loading...</p>
            )}
        </div>
        <button onClick={() => handleStartFundraising}>Start Fundraising</button>
        <div>

        </div></>
    );
};

export default Fundraisings;

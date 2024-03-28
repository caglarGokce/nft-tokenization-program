import React, { FC, useEffect, useState } from 'react';
import { ActiveFundRaising,  MyFundRaising,  NFTonSale, NFTTerms } from './model';
import { add_funds_to_fudnraising as add_funds_to_fundraising, buy_out_tokenized_nft, buy_part_of_tokenized_nft_from_this_program, buy_whole_nft_from_this_program, get_nft_mint, get_tokenized_assets_from_successfull_fundraising, join_fund_raising_to_buy_nft, remove_funds_from_the_fundraising, start_fund_raising_to_buy_nft } from './service';
import { get_active_fundraisings, get_my_fundraisings, get_nfts_on_sale, get_tokenized_nfts_on_sale } from './getAccounts';
import { WalletContextState } from '@solana/wallet-adapter-react';
import { get_fundraising_account } from './utils';
import { PublicKey } from '@metaplex-foundation/js';



interface MyFundraisingsProps {
    wallet: WalletContextState; 
}

const Fundraisings: FC<MyFundraisingsProps> = ({ wallet }) => {

    const [data, setData] = useState<MyFundRaising[] | null>(null); 
    let [add_amount, setAmountToAdd] = useState(0);
    let [remove_amount, setAmountToRemove] = useState(0);

    useEffect(() => {

        const getAccounts = async () => {
            try {
                const nfts = await get_my_fundraisings(wallet);
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


    function setAmounttoAdd(e: any)
    {
        setAmountToAdd(Number(e.target.value));
        add_amount = e.target.value;
    }
    function setAmounttoRemove(e: any)
    {
        setAmountToRemove(Number(e.target.value));
        remove_amount = e.target.value;
    }

    const addFundsToFundRaising = async (fundraising:MyFundRaising) => {
        add_funds_to_fundraising(fundraising.tokens_mint,fundraising.nft_mint,fundraising.funders_account,wallet,add_amount)//TODO amount nft.lamports_per_token katlari olmali
    };
    const removeFundsFromFundRaising= async (fundraising:MyFundRaising) => {
        remove_funds_from_the_fundraising(fundraising.nft_mint,fundraising.funders_account,wallet,remove_amount);
    };
    const claimTokensFromSuccesfulFundraising= async (fundraising:MyFundRaising) => {
        get_tokenized_assets_from_successfull_fundraising(fundraising.tokens_mint,fundraising.funders_account,wallet);
    };

    return (
        <><div>
            <h1>New Page</h1>
            {data !== null ? (
                data.length > 0 ? (
                    <div>
                        {data.map((fundraising: MyFundRaising, index) => (
                            <div key={index} className="card">
                                <div className="card-body">
                                    <h5 className="card-title">My ongoing FundRaisings</h5>
                                    <p>divided into: {fundraising.number_of_tokens}</p>
                                    <p>Sol per token: {fundraising.lamports_per_token}</p>
                                    <p>My tokenized share: {fundraising.tokens_to_receive}</p>
                                    <p>funds invested in Sol: {fundraising.funds_invested}</p>
                                    <p>total funds invested in Sol: {fundraising.funds_collected}</p>
                                    {fundraising.fund_raising_succesful ?(<div>
                                        <button onClick={() => addFundsToFundRaising(fundraising)}>Add Funds to the FundRaising</button>
                                    <button onClick={() => removeFundsFromFundRaising(fundraising)}>Remove Funds from the FundRaising</button>
                                    <input value={add_amount} type="number" onChange={(e) => setAmounttoAdd(e)}>amount to add</input>
                                    <input value={remove_amount} type="number" onChange={(e) => setAmounttoRemove(e)}>amount to remove</input>
                                    </div>):
                                    (<div>
                                    <button onClick={() => claimTokensFromSuccesfulFundraising(fundraising)}>Claim Tokenized assets</button>
                                    </div>)}
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

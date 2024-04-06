"use client"

import { DropdownRadioItems } from "@/components/CustomDropdown/RadioDropdown";
import { useEffect, useMemo, useState } from "react";
import MarketplaceHeader from "./components/MarketplaceTopHeader";
import Stack from "@/components/Stack";
import MarketplaceTopSlot from "./components/MarketplaceTopSlot";
import { MarketplaceTopBox } from "./data";
import { NFTonSale } from "@/helpers/model";
import { get_nfts_on_sale, get_tokenized_nfts_on_sale } from "@/helpers/getAccounts";

const Marketplace = () => {

    const [filter, setFilter] = useState('');
    const [data,setData] = useState<NFTonSale[] | null>(null)
    const filterValues: DropdownRadioItems[] = [
        {
            key: "new",
            value: "New Listed"
        },
        {
            key: "popular",
            value: "Popular"
        }
    ]
    const topList: MarketplaceTopBox[] = [
        {
            index: 998,
            imageURL: 'https://solana.com/_next/static/media/logotype.e4df684f.svg',
            owner: 'Angel Douglas',
            name: 'Central',
            price: 916.92
        },
        {
            index: 660,
            imageURL: 'https://solana.com/_next/static/media/logotype.e4df684f.svg',
            owner: 'Anita Cannon',
            name: 'Expect',
            price: 44.18
        },
        {
            index: 963,
            imageURL: 'https://solana.com/_next/static/media/logotype.e4df684f.svg',
            owner: 'Brian Alvarez',
            name: 'Data',
            price: 524.28
        },
        {
            index: 554,
            imageURL: 'https://solana.com/_next/static/media/logotype.e4df684f.svg',
            owner: 'Shannon Woods',
            name: 'Certain',
            price: 396.2
        },
        {
            index: 685,
            imageURL: 'https://solana.com/_next/static/media/logotype.e4df684f.svg',
            owner: 'Kyle Ramos',
            name: 'Sell',
            price: 674.7
        }
    ];

    useEffect(() => {
        const getNfts = async() => {
            if (filter == filterValues[0].key) {
                const tokenizedNftOnSale = await get_tokenized_nfts_on_sale();
                setData(tokenizedNftOnSale)
            }else{
                const nftOnSale =  await get_nfts_on_sale();
                setData(nftOnSale)
            }
        }
        getNfts()
    }, [filter])

    const changeFilterValue = (value: string) => {
        setFilter(value);
    };

    return (
        <Stack>
            <MarketplaceHeader
                filterValues={filterValues}
                changeFilterValue={changeFilterValue}
                filter={filter}
            />
            {
                <Stack className="space-x-1 overflow-x-auto" isRow>
                    {data?.map((nft,index)=>({
                        index:index,
                        imageURL:"",
                        owner:nft.owner.toString(),
                        name:nft.nft_mint.toString(),
                        price:Number(nft.price.toString())
                    })).map((item:MarketplaceTopBox) => (
                        <Stack
                            key={item.index}
                            className="w-3/12 min-w-[286px]"
                        >
                            <MarketplaceTopSlot data={item} filter={filter} />
                        </Stack>
                    ))}
                </Stack>
            }
        </Stack>

    )
}


export default Marketplace;
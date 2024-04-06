"use client"

import { DropdownRadioItems } from "@/components/CustomDropdown/RadioDropdown";
import { useState } from "react";
import MarketplaceHeader from "./components/MarketplaceTopHeader";
import Stack from "@/components/Stack";
import MarketplaceTopSlot from "./components/MarketplaceTopSlot";
import { MarketplaceTopBox } from "./data";

const Marketplace = () => {

    const [filter, setFilter] = useState('');
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
                    {topList?.map((item) => (
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
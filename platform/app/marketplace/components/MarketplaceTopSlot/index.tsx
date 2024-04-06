import Card from '@/components/Card';
import Stack from '@/components/Stack';
import Image from 'next/image';
import React from 'react';
import { MarketplaceTopBox } from '../../data';
import Typography from '@/components/Typography';
import Sol from "@/public/solana.png"

type P = {
    data: MarketplaceTopBox;
    filter: string;
};

const MarketplaceTopSlot: React.FC<P> = ({ data, filter }) => {

    return (
        <Card elevation={1} radius="lg" className="space-y-6 mx-4">
            <Stack spacing={20} className="justify-between">
                <Stack className="space-x-2 items-center">
                    {data.imageURL && data.imageURL !== '-' && (
                        <Image
                            src={data.imageURL}
                            alt={data.name}
                            width={256}
                            height={256}
                        />
                    )}
                </Stack>
                <Stack spacing={4}>
                    <Stack spacing={1}>
                        <Typography text={"#" + data.index} variant='caption1' />
                        <Typography subtle={true} text={data.owner} variant='subtitle2' />

                    </Stack>
                    <Stack isRow spacing={2}>
                        <Typography text={data.price} variant='body2' />
                        <Image alt='sol' src={Sol} />
                    </Stack>
                </Stack>
            </Stack>
        </Card>
    );
};
export default MarketplaceTopSlot;

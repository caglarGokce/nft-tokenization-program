'use client';

import DataTable from "@/components/DataTable";
import Divider from "@/components/Divider";
import Stack from "@/components/Stack";
import Typography from "@/components/Typography";
import { useTheme } from "@/hooks/theme";
import { sidebarActions } from "@/store/actions";
import { useDispatch } from "react-redux";
import madLadsData from "./data";
import Image from 'next/image';
import Button from "@/components/Button";
import dashboardBackground from "@/public/profile_bg.png";
import Card from "@/components/Card";
import MetricItem from "@/components/Metric";
import { UserAvatar } from "@/components/UserAvatar";
import exampleProfile from "@/public/profile_ex.png"
import { useEffect } from "react";
const Dashboard = () => {
    const dispatch = useDispatch();
    const { palette } = useTheme();

    useEffect(() => {
        dispatch(sidebarActions.changePath({ path: 'dashbaord' }));
    },[]);

    const renderName = (row: any) => {
        return <Typography variant="body1" text={row?.asset} />;
    };

    const metricIcon = (size: number) => (
        <Typography size={size} text={1} />
    );

    return (
        <Stack spacing={12}>
            <Stack spacing={4}>
                <Stack isRow={true} className="sm:flex-row justify-between z-20 w-full h-full p-6">
                    <Typography text={"Dashboard"} variant="header1" />
                    <Stack isRow={true} spacing={4}>
                        <UserAvatar
                            alt="user avatar"
                            size={'lg'}
                            url={exampleProfile}
                        />
                        <Stack>
                            <Typography disableDefaultColor={true} text="Welcome back" variant="caption3" />
                            <Typography text="@ernilmz" variant="caption3" />
                        </Stack>
                    </Stack>

                </Stack>
                <Stack className="relative">
                    <Image
                        src={dashboardBackground}
                        style={{ borderRadius: 16 }}
                        alt="Profile Header"
                        width={1156}
                        height={168}
                        className="relative w-full min-h-72 sm:min-h-40 z-10"
                    />
                    <Stack isRow={true} className="sm:flex-row absolute justify-between z-20 w-full h-full p-6">
                        <Stack className="justify-between">
                            <Stack className="relative h-20 w-34 rounded-md" spacing={6}>
                                <Typography text={"Your Wallet :"} variant="caption3" />
                                <Typography text={"40.000.00 $"} variant="header1" />
                            </Stack>
                            <Stack isRow={true} spacing={8}>
                                <Button text="Withdraw" variant="outlined" />
                                <Button text="Deposit" variant="outlined" />
                            </Stack>
                        </Stack>
                        <Card className="h-20" elevation={1}>
                            <Stack isRow={true} spacing={6} className="justify-between ">
                                <MetricItem
                                    isSmall
                                    value={12}
                                    caption={"Assets"}
                                />
                                <MetricItem
                                    isSmall
                                    value={12}
                                    caption={"Assets"}
                                />
                            </Stack>
                        </Card>

                    </Stack>
                </Stack>
                <Stack isRow className="justify-between">
                    <Typography variant="header2" text="Portfolio" />
                    <Button text="Explore" variant="outlined" />

                </Stack>
                <Divider />
                <DataTable
                    data={madLadsData}
                    cols={[
                        {
                            title: 'Asset',
                            key: 'asset',
                            cellRender: renderName,
                        },
                        {
                            title: 'Floor Price',
                            key: 'floorPrice',
                        },
                        {
                            title: 'Owners',
                            key: 'owners',
                        },
                        {
                            title: 'My Shares',
                            key: "myShares",
                        },
                    ]}
                />
            </Stack>
        </Stack>
    );
}

export default Dashboard
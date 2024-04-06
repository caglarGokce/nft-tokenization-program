import RadioDropdown, { DropdownRadioItems } from "@/components/CustomDropdown/RadioDropdown";
import Divider from "@/components/Divider";
import Stack from "@/components/Stack";
import Typography from "@/components/Typography";


type P = {
    filterValues: DropdownRadioItems[];
    changeFilterValue: (value: string) => void;
    filter: string;
};

const MarketplaceHeader: React.FC<P> = ({
    filterValues,
    changeFilterValue,
    filter,
}) => {

    return (
        <Stack>
            <Stack isRow className="justify-between items-center">
                <Typography text={filter} variant={'header2'} />
                <Stack isRow className=" items-end justify-end space-x-2">
                    <Stack className="w-[130px]">
                        <RadioDropdown
                            title="Category"
                            data={filterValues}
                            handleChange={(value: string) => changeFilterValue(value)}
                            selected={filter}
                        />
                    </Stack>
                </Stack>
            </Stack>
            <Divider margin={{ t: 1.5, b: 4 }} />
        </Stack>
    )
}

export default MarketplaceHeader;
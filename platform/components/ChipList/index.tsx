'use client';

import ChipImageIcon from '@/components/ChipImageIcon';
import SelectableChips from '@/components/SelectableChips';
import Stack from '@/components/Stack';
import Typography from '@/components/Typography';
import { useBreakpointMatch } from '@/hooks/useBreakpointMatch';
import { useMemo, useState } from 'react';
import SearchField from '../SearchField';

export type ChipListPropsType<T> = {
  /** List of items */
  items: T[];
  /** List of selected items */
  selected: T[];
  /** Called when the list of selected items changes
   * @param items The updated items, array with one element in case singleSelect is true
   */
  onChangeSelected: (items: T[]) => void;
  /** If true, only one item can be selected at a time */
  isSingleSelect?: boolean;
  /** Title of the card */
  title: string;
  /** Text to display when there are no items */
  noDataText?: string;
  /** If true, a search field will be displayed along with title */
  searchable?: boolean;
};

export type ChipListItemBaseType = { name: string; id: string; image?: string };

/** Renders a list of selectable chips along with a header in a card*/
const ChipList = <T extends ChipListItemBaseType>({
  items,
  selected,
  onChangeSelected,
  isSingleSelect,
  title,
  noDataText = 'No items available to select.',
  searchable,
}: ChipListPropsType<T>) => {
  const isMd = useBreakpointMatch('md');

  const [search, setSearch] = useState('');

  const searched = useMemo(() => {
    if (!search) return items;
    return items.filter((item) =>
      item.name.toLowerCase().includes(search.toLowerCase()),
    );
  }, [search, items]);

  return (
    <Stack spacing={4}>
      <Stack isRow className="justify-between items-center">
        <Typography variant="subtitle2" text={title} />
        {searchable && <SearchField value={search} onTextChange={setSearch} />}
      </Stack>
      {searched.length > 0 ? (
        <SelectableChips
          chips={searched.map((item) => ({
            icon: item.image
              ? ({ size }) => <ChipImageIcon size={size} image={item.image!} />
              : undefined,
            isSelected: selected.some((s) => s.id === item.id),
            label: item.name,
            value: item.id,
          }))}
          onSelect={(value) => {
            if (isSingleSelect) {
              onChangeSelected([items.find((item) => value === item.id)!]);
              return;
            }

            const index = selected.findIndex((s) => s.id === value);
            let updated = structuredClone(selected);
            if (index === -1) {
              updated.push(items.find((item) => value === item.id)!);
            } else {
              updated.splice(index, 1);
            }
            onChangeSelected(updated);
          }}
          grow={isMd}
        />
      ) : (
        <Stack className="justify-center">
          <Typography
            variant="link3"
            text={items.length > 0 ? 'No search results...' : noDataText}
          />
        </Stack>
      )}
    </Stack>
  );
};

export default ChipList;

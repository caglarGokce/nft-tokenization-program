'use client';

import Stack from '../Stack';
import Tag, { TagPropsType } from '../Tag';

export type SelectableChipsPropsType = {
  /** Data for chips */
  chips: (Pick<TagPropsType, 'icon'> & {
    /** Label shown on the chip */
    label: string;
    /** Unique value provided in onSelect event */
    value: string | number;
    /** If the chip is selected */
    isSelected?: boolean;
  })[];
  /** Called when a chip is selected or unselected
   * @param value Value of the chip
   * @param index Index of the chip
   */
  onSelect?: (value: string | number, index: number) => void;
  /** If the chips should take up max space */
  grow?: boolean;
};

/** Renders a horizontal wrapping row of chips */
const SelectableChips = ({
  chips,
  onSelect,
  grow,
}: SelectableChipsPropsType) => {
  return (
    <Stack isRow className="items-center flex-wrap flex-grow" spacing={2}>
      {chips.map((item, i) => (
        <Tag
          key={item.value}
          text={item.label}
          icon={item.icon}
          isActive={item.isSelected}
          onClick={() => onSelect && onSelect(item.value, i)}
          className={grow ? 'flex-grow basis-0 justify-center' : ''}
        />
      ))}
    </Stack>
  );
};

export default SelectableChips;

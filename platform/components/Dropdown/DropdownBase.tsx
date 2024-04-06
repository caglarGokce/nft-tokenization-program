'use client';

import { DropdownOptionType } from '@/types/Components/Dropdown';
import { Menu, MenuItem, MenuPropsType } from '@/components/Menu';
import React from 'react';
import { faCircleCheck } from '@fortawesome/free-solid-svg-icons';

export type DropdownBasePropsType<V extends string | number> = Pick<
  MenuPropsType,
  'onOpen' | 'onClose' | 'anchor' | 'alignment'
> & {
  /** List of options for dropdown */
  options: DropdownOptionType<V>[];
  /** Value of the selected option. Array of values in case of multiselect. */
  selected: V[] | V | undefined;
  /** Called when the value changes
   * @param option The option selected
   * @param index Index of the option in `options` array
   * @returns false if the change should be rejected
   */
  onChange?: (option: DropdownOptionType<V>, index: number) => boolean | void;
  /** Classes for the menu */
  menuClasses?: string;
};

/** Renders a custom dropdown component */
export const DropdownBase = <V extends string | number>({
  options,
  onChange,
  selected: selectedProp,
  menuClasses,
  ...menuProps
}: DropdownBasePropsType<V>) => {
  const isMultiselect = Array.isArray(selectedProp);
  const selected = isMultiselect ? selectedProp : [selectedProp];
  return (
    <Menu
      {...menuProps}
      closeOnItemSelect={!isMultiselect}
      className={menuClasses}
    >
      {options.map((o, i) => (
        <MenuItem
          key={o.value}
          text={o.name}
          onClick={() => onChange?.(o, i)}
          icon={
            selectedProp && selected.includes(o.value)
              ? faCircleCheck
              : undefined
          }
          iconRight
        />
      ))}
    </Menu>
  );
};

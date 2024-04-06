'use client';

import React, { useCallback, useEffect, useRef, useState } from 'react';
import { DropdownOptionType } from '@/types/Components/Dropdown';
import TextField, { TextFieldPropsType } from '@/components/Form/TextField';
import { faChevronDown, faChevronUp } from '@fortawesome/free-solid-svg-icons';
import { MenuAnchorPropsType } from '@/components/Menu';
import { DropdownBase, DropdownBasePropsType } from './DropdownBase';

export type TextDropdownPropsType<V extends string | number> =
  TextFieldPropsType &
    Omit<DropdownBasePropsType<V>, 'selected' | 'anchor'> & {
      /** Index of the initially selected element */
      initialIndex?: number;
      /** Index of the selected element */
      selectedIndex?: number;
    };

/** Renders a form based dropdown which allows input through a select menu */
export const TextDropdown = <V extends string | number>({
  options,
  onChange,
  initialIndex,
  onOpen,
  onClose,
  selectedIndex,
  ...fieldProps
}: TextDropdownPropsType<V>) => {
  /** Ref for the input element */
  const inputRef = useRef<HTMLInputElement>(null);

  /** Current index of the selected option */
  const [index, setIndex] = useState<number | undefined>(
    initialIndex ?? undefined,
  );

  /** Adjust index when selectedIndex changes */
  useEffect(() => {
    selectedIndex !== undefined && setIndex(selectedIndex);
  }, [selectedIndex, options]);

  /** Handles text field input focus */
  const handleFocus = useCallback(
    (e: React.FocusEvent<HTMLInputElement, Element>, open: () => void) => {
      inputRef.current?.blur();
      fieldProps.inputProps?.onFocus?.(e);
      open();
    },
    [inputRef, fieldProps.inputProps],
  );

  /** Handles menu item click */
  const handleItemClick = (item: DropdownOptionType<V>, index: number) => {
    if (onChange) {
      if (onChange(item, index) === false) return;
    }
    setIndex(index);
  };

  const Input = useCallback(
    ({ open }: MenuAnchorPropsType) => (
      <TextField
        {...fieldProps}
        value={index !== undefined ? options[index].name : undefined}
        inputProps={{
          ...fieldProps.inputProps,
          ref: inputRef,
          onFocus: (e) => handleFocus(e, open),
          className: '!cursor-pointer ' + fieldProps.inputProps?.className,
          tabIndex: -1,
        }}
        wrapperProps={{
          ...fieldProps.wrapperProps,
          className: '!cursor-pointer ' + fieldProps.wrapperProps?.className,
        }}
      />
    ),
    [fieldProps, index, handleFocus],
  );

  return (
    <DropdownBase
      anchor={Input}
      options={options}
      selected={index !== undefined ? options[index].value : undefined}
      onChange={handleItemClick}
      onOpen={onOpen}
      onClose={onClose}
    />
  );
};

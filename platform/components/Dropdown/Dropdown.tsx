'use client';

import React, { useCallback } from 'react';
import { MenuAnchorPropsType } from '@/components/Menu';
import { DropdownBase, DropdownBasePropsType } from './DropdownBase';
import { DropdownAnchor, DropdownAnchorPropsType } from './DropdownAnchor';

export type DropdownPropsType<V extends string | number> = Omit<
  DropdownBasePropsType<V>,
  'anchor'
> &
  Pick<DropdownAnchorPropsType, 'label'>;

/** Renders a dropdown that allows single/multiple item selection */
export const Dropdown = <V extends string | number>({
  label,
  ...menuProps
}: DropdownPropsType<V>) => {
  const anchor = useCallback(
    ({ active }: MenuAnchorPropsType) => (
      <DropdownAnchor label={label} open={active} />
    ),
    [label],
  );

  return <DropdownBase anchor={anchor} {...menuProps} menuClasses={'!w-48'} />;
};

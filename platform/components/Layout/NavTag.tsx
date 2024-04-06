import React from 'react';
import Typography from '../Typography';

export type NavTagPropsType = { tag: string };

export default function NavTag({ tag }: NavTagPropsType) {
  return (
    <Typography
      variant="caption6"
      text={tag.toUpperCase()}
      disableDefaultColor
      className="border border-accentDark text-typographyDark px-2 py-1 rounded-xs"
    />
  );
}

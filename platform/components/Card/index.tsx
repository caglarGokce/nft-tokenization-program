import React, { useMemo } from 'react';
import Surface, { SurfacePropsType } from '../Surface';
import { AppThemeSizeVariantsType } from '@/theme';

export type CardPropsOutlineType = 'heavy' | 'normal';

export type CardPropsType = SurfacePropsType & {
  /** Used to control card border */
  outline?: CardPropsOutlineType;
  /** Controls how rounded the card is */
  radius?: AppThemeSizeVariantsType;
};

export default function Card({
  elevation,
  outline,
  radius,
  ...props
}: CardPropsType) {
  const outlineClass = useMemo(() => {
    if (outline === 'heavy') {
      return 'border-2';
    }
    return 'border';
  }, [outline]);
  return (
    <Surface
      {...props}
      elevation={elevation}
      className={`rounded-${radius || 'md'} p-4 ${
        typeof elevation !== 'number'
          ? 'bg-transparent dark:bg-transparent'
          : ''
      } ${props.className ?? ''} ${
        outline
          ? `border-surface-dark dark:border-surfaceDark-dark ${outlineClass}`
          : ''
      }`}
    />
  );
}

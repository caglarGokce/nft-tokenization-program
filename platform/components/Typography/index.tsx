'use client';

import { useConfigSelector } from '@/hooks/store';
import { AppThemeTypographyVariantsType } from '@/theme';
import React, { CSSProperties, HTMLProps, useMemo } from 'react';

/** Type for typography font weights */
export type TypographyWeightTypes =
  | 'xthin'
  | 'thin'
  | 'light'
  | 'regular'
  | 'medium'
  | 'semibold'
  | 'bold'
  | 'black';

/** Type for typography props */
export type TypographyPropsType = HTMLProps<HTMLDivElement> & {
  /** The text */
  text: string | number;
  /** Variant of the text */
  variant?: AppThemeTypographyVariantsType;
  /** If the text should be light */
  subtle?: boolean;
  /** Weight for the font */
  weight?: TypographyWeightTypes;
  /** If the text should be italic */
  italic?: boolean;
  /** Custom color for text */
  color?: string;
  /** Disables default color class. Allows color control through parent classes. */
  disableDefaultColor?: boolean;
  /** The hyperlink */
  hyperlink?: string;
  /** The hyperlink text*/
  hyperlinkText?: string | number;
};

const weightMap: Record<TypographyWeightTypes, number> = {
  xthin: 100,
  thin: 200,
  light: 300,
  regular: 400,
  medium: 500,
  semibold: 600,
  bold: 700,
  black: 900,
};

/** Typography component to display text */
export default function Typography({
  text,
  variant,
  hyperlink,
  hyperlinkText,
  subtle,
  weight,
  italic,
  color,
  disableDefaultColor,
  className,
  ...props
}: Readonly<TypographyPropsType>) {
  const { dark } = useConfigSelector();
  /** Classes for the text component */
  const [classNames, styles] = useMemo(() => {
    const classes = [];
    const styles: CSSProperties = {};
    // Use dark colors if theme is dark
    if (color) styles.color = color;
    else if (!disableDefaultColor)
      classes.push(dark ? 'text-typographyDark' : 'text-typography');
    // Reduce font visibility if subtle
    if (subtle) classes.push(`opacity-40`);
    if (weight) styles.fontWeight = weightMap[weight];
    if (italic) classes.push('italic');
    if (variant) {
      classes.push(`text-${variant}`);
    } else {
      classes.push('text-body1');
    }
    if (className) classes.push(className);
    return [classes.join(' '), styles];
  }, [
    color,
    disableDefaultColor,
    dark,
    subtle,
    weight,
    italic,
    variant,
    className,
  ]);
  return (
    <div {...props} className={classNames} style={styles}>
      {hyperlink && hyperlinkText && <a className='underline' href={hyperlink}>{hyperlinkText}</a>} {text}
    </div>
  );
}

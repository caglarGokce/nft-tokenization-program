'use client';

import React, { useMemo } from 'react';
import FormSwitch, { FormSwitchPropsType } from '../Form/Switch';
import Typography, { TypographyPropsType } from '../Typography';
import { useTheme } from '@/hooks/theme';

export type LabeledSwitchPropsType = Omit<
  FormSwitchPropsType,
  'left' | 'right'
> & {
  /** Forces dark or light variant */
  variant?: 'dark' | 'light';
  /** Text or Element to render at left */
  left?: string | React.ReactElement;
  /** Text or Element to render at right */
  right?: string | React.ReactElement;
};

/** Renders a switch that shows labels at left and right */
export default function ThemeSwitch({
  variant,
  left,
  right,
  onChange,
  checked,
}: Readonly<LabeledSwitchPropsType>) {
  const { isDark } = useTheme();

  const dark = useMemo(() => {
    if (!variant) return isDark;
    return variant === 'dark';
  }, [variant, isDark]);

  return (
    <FormSwitch
      checked={checked}
      onChange={onChange}
      left={
        typeof left === 'string' ? (
          <DefaultLabel text={left} dark={dark} subtle={checked} />
        ) : (
          left
        )
      }
      right={
        typeof right === 'string' ? (
          <DefaultLabel text={right} dark={dark} subtle={!checked} />
        ) : (
          right
        )
      }
    />
  );
}

type DefaultLabelPropsType = Pick<TypographyPropsType, 'subtle' | 'text'> & {
  dark: boolean;
};
/** Default label component for labeled switch */
const DefaultLabel = ({ subtle, text, dark }: DefaultLabelPropsType) => (
  <span className={dark ? 'text-typographyDark' : 'text-typography'}>
    <Typography subtle={subtle} text={text} disableDefaultColor />
  </span>
);

import { IconDefinition } from '@fortawesome/fontawesome-svg-core';
import React, { HTMLProps, useMemo } from 'react';
import Stack from '../Stack';
import { useTheme } from '@/hooks/theme';
import Icon from '../Icon';
import Typography from '../Typography';

/** Type for LabelTag component props */
export type LabelTagPropsType = HTMLProps<HTMLDivElement> & {
  /** Main text of the LabelTag */
  text: string;
  /** Icon of the LabelTag */
  icon?: IconDefinition;
  /** Makes the LabelTag active */
  isActive?: boolean;
  /** Called when the LabelTag is clicked */
  onClick?: () => void;
};

/** Renders a LabelTag component */
export function LabelTag({
  text,
  icon,
  isActive,
  onClick,
  className,
  ...props
}: Readonly<LabelTagPropsType>) {
  const { palette } = useTheme();
  const [classes, iconColor] = useMemo(() => {
    const base = ['border items-center rounded-xs cursor-pointer px-4 py-1.5'];
    if (isActive) {
      base.push(
        'bg-surfaceDark-light dark:bg-surface-light border-surfaceDark-light dark:border-surface-light text-typographyDark dark:text-typography',
      );
    } else {
      base.push(
        'bg-surface-light dark:bg-surfaceDark-light hover:bg-surface dark:hover:bg-surfaceDark border-surface-dark dark:border-surfaceDark-dark text-typography dark:text-typographyDark',
      );
    }

    className && base.push(className);
    const classes = base.join(' ');
    const iconColor = isActive
      ? palette.accent.DEFAULT
      : palette.typography.DEFAULT;
    return [classes, iconColor];
  }, [palette, isActive, className]);

  return (
    <Stack
      tabIndex={0}
      onKeyDown={(e) => e.key === ' ' && onClick?.()}
      {...props}
      ref={null}
      isRow
      spacing={2}
      className={classes}
      onClick={onClick}
    >
      {icon && <Icon size="sm" icon={icon} color={iconColor} />}
      <Typography
        className="whitespace-nowrap"
        variant="link3"
        text={text}
        disableDefaultColor
      />
    </Stack>
  );
}

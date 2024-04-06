import { useTheme } from '@/hooks/theme';
import { AppThemeSizeVariantsType } from '@/theme';
import { IconDefinition as FAIcon } from '@fortawesome/fontawesome-svg-core';
import {
  FontAwesomeIcon,
  FontAwesomeIconProps,
} from '@fortawesome/react-fontawesome';
import React from 'react';
import { TypographyPropsType } from '../Typography';

/** Props type for Icon component */
export type IconPropsType = Omit<FontAwesomeIconProps, 'icon' | 'size'> &
  Pick<TypographyPropsType, 'disableDefaultColor'> & {
    /** Configuration for icon */
    icon: FAIcon;
    /** Size of the icon */
    size?: AppThemeSizeVariantsType;
    /** Called on click. Makes the icon into a clickable button. */
    onClick?: () => void;
  };

/** Renders given FA icon */
export default function Icon({
  size,
  icon,
  className,
  disableDefaultColor,
  ...props
}: IconPropsType) {
  const { iconSize } = useTheme();
  const classes =
    props.color || disableDefaultColor
      ? className
      : 'text-typography dark:text-typographyDark ' + className;
  return (
    <FontAwesomeIcon
      {...props}
      className={props.onClick ? 'cursor-pointer ' + classes : classes}
      icon={icon}
      style={{
        ...props.style,
        fontSize: iconSize[size ?? 'md'],
      }}
      tabIndex={props.onClick ? 0 : undefined}
    />
  );
}

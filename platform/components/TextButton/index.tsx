import React, { useMemo } from 'react';
import Typography, { TypographyPropsType } from '../Typography';
import { IconDefinition } from '@fortawesome/free-solid-svg-icons';
import Icon from '../Icon';

/** Type for button props */
export type TextButtonPropsType =
  React.ButtonHTMLAttributes<HTMLButtonElement> &
    Pick<TypographyPropsType, 'disableDefaultColor'> & {
      /** Text to show on the button */
      text: string;
      /** Icon to show */
      icon?: IconDefinition;
      /** If the icon should be rendered at right */
      iconRight?: boolean;
      /** If the button should be disabled */
      disabled?: boolean;
      /** Called when button is clicked */
      onClick?: () => void;
      /** Adds loading icon to button. Also applies disabled state.
       * Replaces icon. Affected by `iconRight` prop.
       */
      loading?: boolean;
      /** If the button should stretch to fill parent space */
      fullWidth?: boolean;
    };

/** Renders a clickable text button component */
export default function TextButton({
  text,
  icon,
  iconRight,
  disabled: inDisabled,
  className,
  onClick,
  loading,
  fullWidth,
  disableDefaultColor,
  ...props
}: TextButtonPropsType) {
  const btnClass = useMemo(() => {
    const disabled = loading || inDisabled;
    const btnClass = [
      'cursor-pointer flex flex-row items-center justify-center space-x-2',
    ];
    if (fullWidth) btnClass.push('basis-0 flex-grow');
    if (disabled) {
      btnClass.push('opacity-60 !cursor-default');
    }
    if (className) btnClass.push(className);
    return btnClass.join(' ');
  }, [inDisabled, loading, fullWidth, className]);

  const BtnIcon = () =>
    icon ? (
      <Icon icon={icon} size="md" disableDefaultColor={disableDefaultColor} />
    ) : (
      <></>
    );

  return (
    <button className={btnClass} onClick={onClick} {...props}>
      {!iconRight && <BtnIcon />}
      <Typography
        variant={'body2'}
        disableDefaultColor={disableDefaultColor}
        text={text}
        className="whitespace-nowrap"
      />
      {iconRight && <BtnIcon />}
    </button>
  );
}

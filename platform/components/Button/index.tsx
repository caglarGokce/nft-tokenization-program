import React, { useMemo } from 'react';
import Typography from '../Typography';


/** Possible variant types for button */
export type ButtonVariantType = 'outlined' | 'contained';

/** Type for button props */
export type ButtonPropsType = React.ButtonHTMLAttributes<HTMLButtonElement> & {
  /** Text to show on the button */
  text?: string;
  /** If the button should be large
   * @deprecated Use size prop
   */
  isLarge?: boolean;
  /** Size of the button. `md` by default. */
  size?: 'sm' | 'md' | 'lg';
  /** If the button should be disabled */
  disabled?: boolean;
  /** Called when button is clicked */
  onClick?: () => void;
  /** Variant of the button */
  variant?: ButtonVariantType;
  /** Adds loading icon to button. Also applies disabled state.
   * Replaces icon. Affected by `iconRight` prop.
   */
  loading?: boolean;
  /** If the button should stretch to fill parent space */
  fullWidth?: boolean;
};

/** Renders a clickable button component */
export default function Button({
  text,
  isLarge,
  disabled: inDisabled,
  className,
  onClick,
  variant,
  loading,
  fullWidth,
  size,
  ...props
}: ButtonPropsType) {
  const btnClass = useMemo(() => {
    const disabled = loading || inDisabled;
    const btnClass = ['rounded-xxl flex flex-row items-center justify-center'];
    if (size !== 'sm') btnClass.push('min-w-[140px]');
    if (isLarge || size === 'lg') {
      btnClass.push('px-6 py-3 space-x-2');
    } else {
      btnClass.push('px-3 py-1.5 space-x-1');
    }
    if (fullWidth) btnClass.push('basis-0 flex-grow');
    if (disabled) {
      btnClass.push(
        'cursor-default text-surface-muted dark:text-surfaceDark-muted',
      );
      if (variant === 'contained') {
        btnClass.push(
          'bg-surface-dark dark:bg-surfaceDark-dark text-primary dark:text-primaryDark',
        );
      } else {
        btnClass.push('border border-primary dark:border-primaryDark');
      }
    } else {
      btnClass.push('cursor-pointer');
      if (variant === 'contained') {
        btnClass.push(
          'bg-primary dark:bg-primaryDark hover:text-surfaceDark-muted dark:hover:text-surface-muted text-primary-contrast dark:text-primaryDark-contrast',
        );
      } else {
        btnClass.push(
          'border border-primary dark:border-primaryDark hover:bg-primary dark:hover:bg-primaryDark text-surface-contrast dark:text-surfaceDark-contrast hover:text-primary-contrast hover:dark:text-primaryDark-contrast',
        );
      }
    }
    if (className) btnClass.push(className);
    return btnClass.join(' ');
  }, [inDisabled, isLarge, variant, loading, fullWidth, className, size]);
  return (
    <button className={btnClass} onClick={inDisabled ? undefined : onClick} {...props}>
      {text && (
        <Typography
          variant={
            size === 'sm'
              ? 'caption5'
              : size === 'lg' || isLarge
              ? 'button1'
              : 'button2'
          }
          disableDefaultColor
          text={text}
        />
      )}
    </button>
  );
}

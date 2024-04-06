import React, { DetailedHTMLProps, useMemo } from 'react';
import Typography from '../Typography';
import Icon from '../Icon';
import { useTheme } from '@/hooks/theme';
import { AppThemeSizeVariantsType } from '@/theme';
import { faClose } from '@fortawesome/free-solid-svg-icons';

/** Type for Tag component props */
export type TagPropsType = Omit<
  DetailedHTMLProps<
    React.ButtonHTMLAttributes<HTMLButtonElement>,
    HTMLButtonElement
  >,
  'onClick'
> & {
  /** Main text of the Tag */
  text: string;
  /** Icon of the Tag */
  icon?: (props: { size: AppThemeSizeVariantsType }) => React.ReactNode;
  /** Renders a prefix text in the Tag */
  prefix?: string;
  /** Called when the tag is clicked */
  onClick?: () => void;
  /** Called when the close button on tag is clicked.
   * Not providing onClose will remove close button.
   */
  onClose?: () => void;
  /** Styles Tag as active */
  isActive?: boolean;
};

export default function Tag({
  text,
  prefix,
  icon: PropIcon,
  onClick,
  onClose,
  isActive,
  ...buttonProps
}: Readonly<TagPropsType>) {
  const { colors, isDark } = useTheme();
  const rootClass = useMemo(() => {
    const classes = [
      'flex flex-row items-center px-3 py-1.5 gap-1.5 rounded-xxl border whitespace-nowrap',
    ];

    if (isActive)
      classes.push(
        'bg-primary dark:bg-primaryDark text-primary-contrast dark:text-primaryDark-contrast border-primary dark:border-primaryDark',
      );
    else {
      classes.push(
        'bg-surface-light dark:bg-surfaceDark-light border-surface-dark dark:border-surfaceDark-dark text-typography dark:text-typographyDark',
      );
      if (onClick)
        classes.push(
          'hover:border-surface-muted dark:hover:border-surfaceDark-muted',
        );
    }

    if (!onClick) classes.push('cursor-default');

    if (buttonProps.className) classes.push(buttonProps.className);

    return classes.join(' ');
  }, [isActive, buttonProps.className, onClick]);
  return (
    <button
      {...buttonProps}
      tabIndex={onClick ? 0 : -1}
      className={rootClass}
      onClick={onClick}
    >
      {prefix && (
        <Typography
          variant="button2"
          text={prefix}
          color={
            isDark || isActive || (isDark && isActive)
              ? colors.typographyDark.muted!
              : colors.primary.muted!
          }
        />
      )}
      <Typography variant="button2" text={text} disableDefaultColor />
      {onClose && (
        <Icon
          size="sm"
          icon={faClose}
          className="text-typographyDark-muted dark:text-surface-dark"
          tabIndex={0}
          onClick={onClose}
        />
      )}
    </button>
  );
}

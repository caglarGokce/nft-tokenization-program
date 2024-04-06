import { useMemo } from 'react';
import Stack, { StackPropsType } from '../Stack';
import Typography from '../Typography';
import Icon from '../Icon';
import { faChevronUp, faChevronDown } from '@fortawesome/free-solid-svg-icons';

export type DropdownAnchorPropsType = Omit<
  StackPropsType,
  'isRow' | 'spacing'
> & {
  /** If the dropdown is open */
  open?: boolean;
  /** Label of the dropdown anchor */
  label: string;
};

/** Anchor component for dropdown */
export const DropdownAnchor = ({
  open,
  label,
  ...props
}: DropdownAnchorPropsType) => {
  const classes = useMemo(() => {
    return (
      'rounded-xxl items-center justify-between py-2 px-4 bg-surface-light dark:bg-surfaceDark-light cursor-pointer border border-surface-dark dark:border-surfaceDark-dark hover:border-surface-muted dark:hover:border-surfaceDark-muted ' +
      props.className
    );
  }, [props.className]);

  return (
    <Stack {...props} ref={null} isRow className={classes} spacing={2}>
      <Typography variant="link3" text={label} />
      <Icon icon={open ? faChevronUp : faChevronDown} size="md" />
    </Stack>
  );
};

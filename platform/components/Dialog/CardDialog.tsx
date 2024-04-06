'use client';

import React, { useMemo } from 'react';
import Card from '../Card';
import Typography from '../Typography';
import Stack from '../Stack';
import Divider from '../Divider';
import Dialog, { DialogPropsType } from './Dialog';
import Button, { ButtonPropsType } from '../Button';
import Icon from '../Icon';
import { faClose } from '@fortawesome/free-solid-svg-icons';

export type CardDialogPropsType = DialogPropsType & {
  /** Title of the dialog. Given a string, renders a header text with close button. */
  title?: string;
  /** Called with method when the user tries to close dialog.
   * @param method How the user tried to close the modal.
   */
  onClose?: (method: 'backdrop' | 'button') => void;
  /** Action buttons at bottom of modal. First item is primary button.
   * Variants are automatically applied unless specified.
   */
  buttons?: ButtonPropsType[];
  /** Restricts the dialog from being closed through close button or backdrop click */
  persist?: boolean;
  /** Classes for the container card */
  className?: string;
};
const CardDialog = ({
  isOpen,
  onClose,
  children,
  title,
  buttons,
  persist,
  className,
}: CardDialogPropsType) => {
  const classes = useMemo(() => {
    const base = ['p-4 w-full m-4 lg:p-8 lg:w-1/2 lg:m-0 max-w-2xl'];
    if (className) base.push(className);
    return base.join(' ');
  }, [className]);
  return (
    <Dialog
      isOpen={isOpen}
      onClose={() => !persist && onClose?.('backdrop')}
      className="p-4"
    >
      <Card elevation={0} className={classes}>
        <Stack>
          {title && (
            <Stack isRow className="justify-between items-center">
              <Typography variant="header2" text={title} />
              {!persist && (
                <Icon
                  icon={faClose}
                  size="xl"
                  onClick={() => onClose?.('button')}
                />
              )}
            </Stack>
          )}
          {title && <Divider margin={{ t: 2 }} />}
          <div className="overflow-y-auto">
            <div className="h-6"></div>
            {children}
          </div>
          {buttons && (
            <Stack isRow isReverse className="mt-8" spacing={1.5}>
              {buttons.map((b, i) => (
                <Button
                  key={b.id}
                  variant={i === 0 ? 'contained' : 'outlined'}
                  {...b}
                  onClick={() => {
                    b.onClick?.();
                    onClose?.('button');
                  }}
                  fullWidth
                />
              ))}
            </Stack>
          )}
        </Stack>
      </Card>
    </Dialog>
  );
};

export default CardDialog;

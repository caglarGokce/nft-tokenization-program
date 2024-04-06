'use client';
import { useTheme } from '@/hooks/theme';
import { useBreakpointMatch } from '@/hooks/useBreakpointMatch';
import React, { HTMLProps, ReactNode, useRef } from 'react';

export type DialogPropsType = HTMLProps<HTMLDialogElement> & {
  /** If the dialog is open or not */
  isOpen?: boolean;
  /** Called with method when the user tries to close dialog.*/
  onClose?: () => void;
  /** Content of the dialog box */
  children: ReactNode;
};
const Dialog = ({
  isOpen,
  onClose,
  children,
  className,
  ...props
}: DialogPropsType) => {
  const {
    isDark,
    dimensions: { sidebar },
  } = useTheme();
  const ref = useRef<HTMLDialogElement>(null);
  const isMd = useBreakpointMatch('md');

  if (!isOpen) return <></>;

  return (
    <dialog
      {...props}
      aria-hidden="true"
      ref={ref}
      className={`${
        isDark ? 'dark bg-surfaceDark-muted' : 'bg-surfaceDark-light'
      } flex fixed top-0 bg-opacity-40 z-50 dark:backdrop-blur-xs dark:md:backdrop-blur-sm h-full w-full justify-center items-center ${className}`}
      style={
        isMd
          ? { left: sidebar.width, width: `calc(100% - ${sidebar.width})` }
          : {}
      }
      onClick={(e) => {
        if (ref.current === e.target) {
          onClose?.();
        }
      }}
    >
      {children}
    </dialog>
  );
};

export default Dialog;

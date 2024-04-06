import React from 'react';
import Logo from '../Logo';
import { useTheme } from '@/hooks/theme';
import { useBreakpointMatch } from '@/hooks/useBreakpointMatch';

export type HeaderPropsType = {
  /** Called when menu icon is clicked */
  onOpenMenu: () => void;
};

export default function Header({ onOpenMenu }: Readonly<HeaderPropsType>) {
  const {
    dimensions: { headerMobile },
  } = useTheme();
  const isMd = useBreakpointMatch('md');
  return (
    <div
      className="fixed top-0 left-0 md:static flex flex-row gap-2 justify-between md:justify-between items-center bg-surfaceDark-light dark:bg-surfaceDark md:dark:bg-transparent md:bg-transparent py-1 px-4 md:p-0 md:h-auto w-full z-40"
      style={{ height: isMd ? 'auto' : headerMobile.height }}
    >
      <Logo height={21} dark={false} wide className="block md:hidden" />
    </div>
  );
}

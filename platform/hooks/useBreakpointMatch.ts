'use client';

import { AppThemeScreenSizeVariantsType } from '@/theme';
import { useTheme } from './theme';
import { useWindowSize } from 'usehooks-ts';
import { useMemo } from 'react';

/** Matches the given breakpoint with the current screen size
 * to determine if the breakpoint is triggered or not. If the
 * screen width >= given breakpoint, the hook will return true.
 * Otherwise, it will return false.
 * @param breakpoint The breakpoint to match
 * @param width Custom width to match breakpoint with. Screen
 * width is used by default.
 */
export const useBreakpointMatch = (
  breakpoint: AppThemeScreenSizeVariantsType,
  width?: number,
) => {
  const { screens } = useTheme();
  const size = useWindowSize();
  const match = useMemo(() => {
    const w = width ?? size.width;
    return w >= screens[breakpoint];
  }, [breakpoint, screens, size, width]);

  return match;
};

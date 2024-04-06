import {
  AppTheme,
  AppThemeColorsDefaultVariantsType,
  AppThemeColorsType,
  AppThemeDarkColors,
  AppThemeDefaultColors,
  AppThemeType,
} from '@/theme';
import { useConfigSelector } from '../store';

export type DynamicThemeType = AppThemeType & {
  /** Dynamically adjusted colors that support dark mode */
  palette: AppThemeColorsType;
  /** If the app is in dark mode */
  isDark: boolean;
};

/** Provides theme options like palette, dimensions etc. Adapts
 * with dark mode.
 */
export function useTheme(): DynamicThemeType {
  const { dark } = useConfigSelector();
  let palette = {} as AppThemeColorsType;
  // Assign all dark colors if dark mode enabled
  if (dark) {
    palette = structuredClone(AppThemeDarkColors);
  }
  // Assign default colors if dark mode doesn't have them
  Object.keys(AppThemeDefaultColors).forEach((_key) => {
    const key = _key as AppThemeColorsDefaultVariantsType;
    if (!palette[key]) palette[key] = AppThemeDefaultColors[key];
  });
  return { ...AppTheme, palette, isDark: dark };
}

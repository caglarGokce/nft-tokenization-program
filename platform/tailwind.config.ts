import type { Config } from 'tailwindcss';
import {
  AppTheme,
  AppThemeBackdropBlurVariantsType,
  AppThemeScreenSizeVariantsType,
  AppThemeSizeVariantsType,
  AppThemeTypographyVariantsType,
} from './theme';

const getFontSize = () => {
  const fontSize = {} as Record<
    AppThemeTypographyVariantsType,
    [string, Record<'lineHeight' | 'fontWeight', string>]
  >;
  Object.keys(AppTheme.typography).forEach((_key) => {
    const key = _key as AppThemeTypographyVariantsType;
    const rule = AppTheme.typography[key];
    fontSize[key] = [
      rule.fontSize + 'px',
      {
        lineHeight: rule.lineHeight + 'px',
        fontWeight: rule.fontWeight.toString(),
      },
    ];
  });
  return fontSize;
};

/** Converts app theme numeric values into css compatible `px` strings */
const numericDictToPx = <T extends string | number | symbol>(
  numerics: Record<T, number>,
) => {
  const value = {} as Record<T, string>;
  Object.keys(numerics).forEach((_key) => {
    const key = _key as T;
    value[key] = numerics[key] + 'px';
  });
  return value;
};

const getBorderRadius = () => {
  const base = numericDictToPx<AppThemeSizeVariantsType>(AppTheme.borderRadius);
  const borderRadius = {
    ...base,
    DEFAULT: AppTheme.borderRadius['xxs'] + 'px',
  };
  return borderRadius;
};

const getScreens = () =>
  numericDictToPx<AppThemeScreenSizeVariantsType>(AppTheme.screens);

const getBackdropBlur = () =>
  numericDictToPx<AppThemeBackdropBlurVariantsType>(AppTheme.backdropBlur);

const config: Config = {
  content: [
    './pages/**/*.{js,ts,jsx,tsx,mdx}',
    './components/**/*.{js,ts,jsx,tsx,mdx}',
    './app/**/*.{js,ts,jsx,tsx,mdx}',
  ],
  darkMode: 'class',
  theme: {
    extend: {
      backgroundImage: {
        'gradient-radial': 'radial-gradient(var(--tw-gradient-stops))',
        'gradient-conic':
          'conic-gradient(from 180deg at 50% 50%, var(--tw-gradient-stops))',
      },
      colors: AppTheme.colors,
      boxShadow: AppTheme.shadows,
    },
    fontFamily: AppTheme.fontFamily,
    fontSize: getFontSize(),
    borderRadius: getBorderRadius(),
    screens: getScreens(),
    backdropBlur: getBackdropBlur(),
  },
  safelist: [
    { pattern: /text-+/ },
    { pattern: /flex-+/ },
    { pattern: /gap-+/ },
  ],
  plugins: [],
};
export default config;

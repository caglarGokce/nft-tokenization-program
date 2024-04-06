/** App theme color variants */
export type AppThemeColorsDefaultVariantsType =
  | 'primary'
  | 'accent'
  | 'typography'
  | 'surface'
  | 'error'
  | 'warning'
  | 'success'
  | 'info'
  | 'purple'
  | 'orange';

/** App theme dark shade color variants */
export type AppThemeColorsDarkVariantsType = {
  [P in AppThemeColorsDefaultVariantsType]: `${P}Dark`;
}[AppThemeColorsDefaultVariantsType];

export type AppThemeColorsVariantsType =
  | AppThemeColorsDefaultVariantsType
  | AppThemeColorsDarkVariantsType;

/** Base type for color variants */
export type AppThemeColorVariantsType = {
  /** Default color */
  DEFAULT: string;
  /** Lighter shade of the color */
  light?: string;
  /** Darker shade of the color */
  dark?: string;
  /** Subtle shade of the color */
  muted?: string;
  /** Contrast color for elements over this color */
  contrast?: string;
};

/** Type for colors in the theme */
export type AppThemeColorsType<
  T extends string | number | symbol = AppThemeColorsDefaultVariantsType,
> = Record<T, AppThemeColorVariantsType>;

/** Type for font family in theme */
export type AppThemeFontFamilyType = { sans: string[] };

/** Type for typography variants in theme */
export type AppThemeTypographyVariantsType =
  | 'caption1'
  | 'caption2'
  | 'caption3'
  | 'caption4'
  | 'caption5'
  | 'caption6'
  | 'caption7'
  | 'display1'
  | 'display2'
  | 'display3'
  | 'header1'
  | 'header2'
  | 'header3'
  | 'subtitle1'
  | 'subtitle2'
  | 'body1'
  | 'body2'
  | 'button1'
  | 'button2'
  | 'button3'
  | 'link1'
  | 'link2'
  | 'link3';

/** Type of the typography rules in theme */
export type AppThemeTypographyType = Record<
  AppThemeTypographyVariantsType,
  Record<'fontSize' | 'fontWeight' | 'lineHeight', number>
>;

/** Type for app theme size variants */
export type AppThemeSizeVariantsType =
  | 'xxl'
  | 'xl'
  | 'lg'
  | 'md'
  | 'sm'
  | 'xs'
  | 'xxs';

/** Type for screen size variants */
export type AppThemeScreenSizeVariantsType = 'sm' | 'md' | 'lg' | 'xl' | 'xxl';

/** Type for backdrop blur variants */
export type AppThemeBackdropBlurVariantsType = 'xs' | 'sm';

/** Type of the theme */
export type AppThemeType = {
  /** Color palette of the theme */
  colors: AppThemeColorsType<AppThemeColorsDefaultVariantsType> &
    AppThemeColorsType<AppThemeColorsDarkVariantsType>;
  /** Font family for the theme */
  fontFamily: AppThemeFontFamilyType;
  /** Typography rules in `px` for the theme */
  typography: AppThemeTypographyType;
  /** Icon sizing rules in `px` for the theme */
  iconSize: Record<AppThemeSizeVariantsType, number>;
  /** Radius rules in `px` for the theme */
  borderRadius: Record<AppThemeSizeVariantsType, number>;
  /** Shadow rules */
  shadows: Record<'snackbar', string>;
  /** Dimensions for components of app */
  dimensions: Record<
    'sidebar' | 'headerMobile',
    Partial<Record<'width' | 'height', string>>
  >;
  /** Screen Breakpoints for the theme in `px` */
  screens: Record<AppThemeScreenSizeVariantsType, number>;
  /** Backdrop blur vlaues for the theme in `px` */
  backdropBlur: Record<AppThemeBackdropBlurVariantsType, number>;
};

/** Default theme colors */
export const AppThemeDefaultColors: AppThemeColorsType = {
  primary: {
    DEFAULT: '#000000',
    muted: '#808080',
    contrast: '#FFFFFF',
  },
  accent: {
    DEFAULT: '#5CFBC2',
    muted: '#EFFFF9',
    contrast: '#000000',
  },
  typography: {
    DEFAULT: '#000000',
    muted: '#444444',
  },
  surface: {
    light: '#FFFFFF',
    DEFAULT: '#FAFAFA',
    dark: '#EFEFEF',
    muted: '#C2C2C2',
    contrast: '#000000',
  },
  error: {
    DEFAULT: '#FF4B4B',
    muted: '#FFE5E6',
    contrast: '#FFFFFF',
  },
  warning: {
    DEFAULT: '#A27825',
    muted: '#FFF6E5',
    contrast: '#FFFFFF',
  },
  success: {
    DEFAULT: '#2B8866',
    muted: '#E3FDF3',
    contrast: '#FFFFFF',
  },
  info: {
    DEFAULT: '#2CA5EA',
    muted: '#DAEFFB',
    contrast: '#FFFFFF',
  },
  purple: {
    DEFAULT: '#9744FF',
    muted: '#E8D6FF',
    contrast: '#FFFFFF',
  },
  orange: {
    DEFAULT: '#FF6B00',
    muted: '#FFEDE5',
    contrast: '#FFFFFF',
  },
};

/** Dark theme colors */
export const AppThemeDarkColors: AppThemeColorsType = {
  primary: {
    DEFAULT: '#FFFFFF',
    muted: '#C2C2C2',
    contrast: '#000000',
  },
  accent: {
    DEFAULT: AppThemeDefaultColors.accent.DEFAULT,
    muted: '#0E261D',
    contrast: '#000000',
  },
  typography: {
    DEFAULT: '#FFFFFF',
    muted: '#686868',
  },
  surface: {
    light: '#000000',
    DEFAULT: '#131313',
    dark: '#242424',
    muted: '#808080',
    contrast: '#FFFFFF',
  },
  error: {
    DEFAULT: AppThemeDefaultColors.error.DEFAULT,
    muted: '#491E1E',
    contrast: '#FFFFFF',
  },
  warning: {
    DEFAULT: AppThemeDefaultColors.warning.DEFAULT,
    muted: '#301E03',
    contrast: '#FFFFFF',
  },
  success: {
    DEFAULT: AppThemeDefaultColors.success.DEFAULT,
    muted: '#0F241E',
    contrast: '#FFFFFF',
  },
  info: {
    DEFAULT: AppThemeDefaultColors.info.DEFAULT,
    muted: '#002737',
    contrast: '#FFFFFF',
  },
  purple: {
    DEFAULT: AppThemeDefaultColors.purple.DEFAULT,
    muted: '#35134F',
    contrast: '#FFFFFF',
  },
  orange: {
    DEFAULT: AppThemeDefaultColors.orange.DEFAULT,
    muted: '#391A04',
    contrast: '#FFFFFF',
  },
};

const makePalette = () => {
  const def = AppThemeDefaultColors;
  const dark = {} as AppThemeColorsType<AppThemeColorsDarkVariantsType>;
  Object.keys(AppThemeDarkColors).forEach((_key) => {
    const key = _key as keyof AppThemeColorsType;
    const darkKey = (_key + 'Dark') as AppThemeColorsDarkVariantsType;
    dark[darkKey] = AppThemeDarkColors[key];
  });
  return {
    ...def,
    ...dark,
  };
};

export const AppTheme: AppThemeType = {
  colors: makePalette(),
  fontFamily: {
    sans: ['Neue Haas Grotesk Display Pro', 'sans-serif'],
  },
  typography: {
    caption1: {
      fontSize: 14,
      fontWeight: 700,
      lineHeight: 18,
    },
    caption2: {
      fontSize: 12,
      fontWeight: 700,
      lineHeight: 14,
    },
    caption3: {
      fontSize: 12,
      fontWeight: 500,
      lineHeight: 14,
    },
    caption4: {
      fontSize: 10,
      fontWeight: 500,
      lineHeight: 12,
    },
    caption5: {
      fontSize: 10,
      fontWeight: 400,
      lineHeight: 12,
    },
    caption6: {
      fontSize: 8,
      fontWeight: 700,
      lineHeight: 10,
    },
    caption7: {
      fontSize: 8,
      fontWeight: 400,
      lineHeight: 10,
    },
    display1: {
      fontSize: 76,
      fontWeight: 500,
      lineHeight: 76,
    },
    display2: {
      fontSize: 44,
      fontWeight: 700,
      lineHeight: 50,
    },
    display3: {
      fontSize: 40,
      fontWeight: 500,
      lineHeight: 46,
    },
    header1: {
      fontSize: 32,
      fontWeight: 500,
      lineHeight: 36,
    },
    header2: {
      fontSize: 24,
      fontWeight: 500,
      lineHeight: 28,
    },
    header3: {
      fontSize: 20,
      fontWeight: 500,
      lineHeight: 24,
    },
    subtitle1: {
      fontSize: 16,
      fontWeight: 500,
      lineHeight: 20,
    },
    subtitle2: {
      fontSize: 14,
      fontWeight: 500,
      lineHeight: 18,
    },
    body1: {
      fontSize: 16,
      fontWeight: 400,
      lineHeight: 20,
    },
    body2: {
      fontSize: 14,
      fontWeight: 400,
      lineHeight: 18,
    },
    button1: {
      fontSize: 16,
      fontWeight: 500,
      lineHeight: 20,
    },
    button2: {
      fontSize: 12,
      fontWeight: 500,
      lineHeight: 14,
    },
    button3: {
      fontSize: 10,
      fontWeight: 500,
      lineHeight: 12,
    },
    link1: {
      fontSize: 14,
      fontWeight: 500,
      lineHeight: 18,
    },
    link2: {
      fontSize: 14,
      fontWeight: 400,
      lineHeight: 18,
    },
    link3: {
      fontSize: 12,
      fontWeight: 400,
      lineHeight: 14,
    },
  },
  iconSize: {
    xxl: 32,
    xl: 20,
    lg: 18,
    md: 16,
    sm: 14,
    xs: 10,
    xxs: 8,
  },
  borderRadius: {
    xxl: 9999,
    xl: 30,
    lg: 20,
    md: 16,
    sm: 12,
    xs: 8,
    xxs: 4,
  },
  shadows: {
    snackbar:
      '0px 4px 6px -2px rgba(16, 24, 40, 0.03), 0px 12px 16px -4px rgba(16, 24, 40, 0.08)',
  },
  dimensions: {
    sidebar: {
      width: '236px',
    },
    headerMobile: {
      height: '48px',
    },
  },
  screens: {
    sm: 640,
    md: 768,
    lg: 1024,
    xl: 1280,
    xxl: 1536,
  },
  backdropBlur: {
    xs: 1.5,
    sm: 3,
  },
};

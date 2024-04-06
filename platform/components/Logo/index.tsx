import Image, { ImageProps } from 'next/image';
import React from 'react';
import AppLogo from '@/public/Logo.png';

type P = Omit<ImageProps, 'alt' | 'src'> & {
  /** Use the wider version of the logo */
  wide?: boolean;
  /** Use logo optimised for dark background */
  dark?: boolean;
};

/** Renders Owni logo */
const Logo = ({ wide, dark, ...props }: P) => {
  return <Image {...props} alt="owni-logo" src={AppLogo} />;
};

export default Logo;

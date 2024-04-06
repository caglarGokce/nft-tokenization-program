'use client';

import Stack from '@/components/Stack';
import { useTheme } from '@/hooks/theme';
import { AppThemeSizeVariantsType } from '@/theme';
import Image from 'next/image';

/** Renders an image as an icon to be used in Chips */
const ChipImageIcon = ({
  image,
  size: inSize,
}: {
  image: string;
  size: AppThemeSizeVariantsType;
}) => {
  const { iconSize } = useTheme();
  const size = iconSize[inSize];
  return (
    <Stack
      className="justify-center rounded-xxl overflow-hidden"
      style={{ width: size, height: size }}
    >
      <Image
        width={size}
        height={size}
        src={image}
        style={{ width: size, height: size, objectFit: 'contain' }}
        alt=""
      />
    </Stack>
  );
};

export default ChipImageIcon;

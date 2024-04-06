import React from 'react';
import Stack from '../Stack';
import Tooltip from '../Tooltip';
import Image, { ImageProps } from 'next/image';
import { useTheme } from '@/hooks/theme';

export type AvatarSetAvatarType = Pick<ImageProps, 'alt' | 'src'> & {
  /** Identifier of the image, used as key */
  id?: string;
  /** Tooltip text to show on hover */
  tooltip?: string;
};

export type AvatarSetProps = {
  /** List of avatars */
  avatars: AvatarSetAvatarType[];
};

/** Shows a set of images in a row with each image slightly overlapping the following image */
export default function AvatarSet({ avatars }: Readonly<AvatarSetProps>) {
  const data = avatars;
  return (
    <Stack isRow spacing={1}>
      {data.map((a) => (
        <Avatar {...a} key={a.id} tooltip={a.tooltip} />
      ))}
    </Stack>
  );
}

const Avatar = ({ alt, src, tooltip }: Omit<AvatarSetAvatarType, 'id'>) => {
  const {
    iconSize: { xl },
  } = useTheme();
  return (
    <Wrapper tooltip={tooltip}>
      <Image
        className="rounded-xxl"
        alt={alt}
        height={xl}
        width={xl}
        src={src}
      />
    </Wrapper>
  );
};

const Wrapper = ({
  children,
  tooltip,
}: { children: React.ReactNode } & Pick<AvatarSetAvatarType, 'tooltip'>) =>
  tooltip ? <Tooltip text={tooltip}>{children}</Tooltip> : <>{children}</>;

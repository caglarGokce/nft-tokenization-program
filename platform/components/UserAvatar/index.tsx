import { faUser } from '@fortawesome/free-solid-svg-icons';
import Image, { ImageProps, StaticImageData } from 'next/image';
import Icon from '../Icon';
import Surface from '../Surface';

export type UserAvatarPropsType = Omit<
  ImageProps,
  'src' | 'width' | 'height'
> & {
  /** URL for the image */
  url?: string | StaticImageData;
  /** Size of the image */
  size?: 'lg' | 'md';
};

export const UserAvatar = ({ url, size, ...props }: UserAvatarPropsType) => {
  const sizePx = size === 'lg' ? 32 : 24;
  const sizes = {
    width: sizePx,
    height: sizePx,
  };
  return (
    <div className="rounded-xxl overflow-hidden">
      {url ? (
        <Image {...props} src={url} {...sizes} />
      ) : (
        <Surface
          elevation={2}
          style={sizes}
          className="flex items-center justify-center"
        >
          <Icon icon={faUser} fontSize={sizePx} />
        </Surface>
      )}
    </div>
  );
};

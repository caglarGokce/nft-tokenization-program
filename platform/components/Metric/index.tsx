import React, { useMemo } from 'react';
import Typography from '../Typography';
import Card, { CardPropsOutlineType } from '../Card';
import { useTheme } from '@/hooks/theme';
import Tooltip from '../Tooltip';
import Icon from '../Icon';
import { IconDefinition } from '@fortawesome/fontawesome-svg-core';
import { faInfoCircle } from '@fortawesome/free-solid-svg-icons';
import Stack from '../Stack';

/** Props type for MetricItem component */
export type MetricItemPropsType = {
  /** Value of the data*/
  value: number | string;
  /** Shows a subtle text next to value */
  value2?: string;
  /** Description of the data */
  caption?: string;
  /** Shows another caption between caption and tooltip, only
   * visible when caption is specified.
   */
  caption2?: string;
  /** Custom Icon describing the data. Replaces any icon provided
   * through `icon` prop. For icons, use `icon` prop. This should
   * be used to render custom nodes like typography instead of icon.
   */
  Icon?: ({ size }: { size: number }) => React.ReactNode;
  /** FA Icon to use */
  icon?: IconDefinition;
  /** Renders a smaller version of MetricItem */
  isSmall?: boolean;
  /** Used to control card border */
  outline?: CardPropsOutlineType;
  /** Hint to show in info tooltip, only visible when caption is
   * specified.
   */
  hint?: string;
};

/** Renders MetricItem component to show data with caption and icon */
export default function MetricItem({
  value,
  value2,
  caption,
  caption2,
  Icon: IconComponent,
  icon,
  isSmall,
  outline,
  hint,
}: Readonly<MetricItemPropsType>) {
  const { palette, iconSize: themeIconSizes } = useTheme();
  // Calculate surface class, icon size and text variant based on size
  const [iconCardClass, cardClass, iconSize, textVariant] = useMemo(() => {
    const common =
      'flex items-center justify-center text-typography dark:text-typographyDark';
    if (isSmall) {
      return [
        'w-10 h-10 !p-2.5 ' + common,
        !outline ? ' !p-0' : '',
        IconComponent ? themeIconSizes.xl : ('xl' as const),
        'subtitle2' as const,
      ];
    }
    return [
      'w-[60px] h-[60px] ' + common,
      !outline ? ' !p-0' : '',
      IconComponent ? themeIconSizes.xxl : ('xxl' as const),
      'header3' as const,
    ];
  }, [isSmall, outline, IconComponent, themeIconSizes]);

  return (
    <Card
      className={['flex flex-row gap-4 items-center', cardClass].join(' ')}
      outline={outline}
    >
      {(IconComponent || icon) && (
        <Card
          elevation={2}
          className={[iconCardClass, cardClass].join(' ')}
          radius={isSmall ? 'xs' : 'sm'}
        >
          {IconComponent ? (
            <IconComponent size={iconSize as number} />
          ) : (
            <Icon size={iconSize as 'xl' | 'xxl'} icon={icon!} />
          )}
        </Card>
      )}
      <div className="flex flex-col gap-0.5">
        <Stack isRow className="items-center" spacing={1}>
          <Typography text={value} variant={textVariant} />
          {value2 && (
            <Typography
              text={value2}
              variant="link2"
              className="text-typographyDark-muted dark:text-surfaceDark-muted"
            />
          )}
        </Stack>
        {caption && (
          <div className="flex flex-row gap-1">
            <Typography variant="link3" text={caption} />
            {caption2 && <Typography variant="button2" text={caption2} />}
            {hint && (
              <Tooltip text={hint}>
                <div
                  className={`text-typography dark:text-typographyDark text-link3`}
                >
                  <Icon
                    icon={faInfoCircle}
                    size={'sm'}
                    color={palette.surface.muted}
                  />
                </div>
              </Tooltip>
            )}
          </div>
        )}
      </div>
    </Card>
  );
}

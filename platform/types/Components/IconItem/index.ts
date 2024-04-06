import { IconDefinition } from '@fortawesome/pro-regular-svg-icons';
import React from 'react';

/** Props for IconItem component */
export type IconItemPropType = {
  /** MUI Icon (or text as icon) to show with the data */
  icon: IconDefinition | string;
  /** Color of the icon */
  iconColor?: string;
  /** If the icon color should replace the default background color */
  iconColorAffectsBg?: boolean;
  /** Children rendered to the right side of the icon */
  children: React.ReactNode | string;
};

/** Data type for IconItemBody component */
export type IconItemBodyDataType = {
  /** Value of the item */
  value: number | string;
  /** Change in the value, shown next to the value as increase or decrease based on positive or negative value */
  change?: number;
};

/** Props for Icon Item custom render component */
export type IconItemRenderPropType = IconItemBodyDataType & {
  /** The default component rendered */
  DefaultComponent: () => JSX.Element;
};

/** Props used for customization of IconItem component */
export type IconItemModifierPropType = {
  /** Custom component to render instead of text
   * @param props Props for the IconItem
   */
  render?: (props: IconItemRenderPropType) => React.ReactElement;
  /** Custom component to render instead of caption
   * @param props Props for the IconItem
   */
  renderCaption?: (props: IconItemRenderPropType) => React.ReactElement;
  /** Function used to format the value before it is displayed
   * @param v Value of the item
   * @returns Formatted value of the item
   */
  formatValue?: (v: number | string) => string;
  /** Function used to format the change before it is displayed
   * @param v Change of the item
   * @returns Formatted change of the item
   */
  formatChange?: (v: number) => string;
};

/** Props for IconItemBody component */
export type IconItemBodyPropType = IconItemBodyDataType &
  IconItemModifierPropType & {
    /** Caption shown with the data */
    caption: string;
  };

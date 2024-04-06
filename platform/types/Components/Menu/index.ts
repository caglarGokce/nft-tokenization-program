import { CardPropsType } from '@/components/Card';
import { IconDefinition } from '@fortawesome/pro-regular-svg-icons';
import { HTMLProps } from 'react';

export type MenuPropsType = HTMLProps<HTMLDivElement> & {
  /** The element the menu should originate from */
  anchor?: HTMLElement;
  /** If the menu should be open */
  active: boolean;
  /** Props for the card container */
  cardProps?: CardPropsType;
  /** Called when dropdown is opened */
  onOpen?: () => void;
  /** Called when dropdown is closed */
  onClose?: () => void;
  /** If the dropdown is disabled */
  disabled?: boolean;
};

export type MenuItemPropsType = {
  /** Text of the item */
  text: string;
  /** Icon of the item */
  icon?: IconDefinition;
  /** If the icon should be at right side */
  iconRight?: boolean;
  /** Called when item is clicked */
  onClick?: () => void;
};

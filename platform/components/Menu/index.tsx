import { IconDefinition } from '@fortawesome/free-solid-svg-icons';
import React, {
  HTMLProps,
  createContext,
  useCallback,
  useContext,
  useMemo,
  useRef,
  useState,
} from 'react';
import Card, { CardPropsType } from '@/components/Card';
import FaIcon from '@/components/Icon';
import Stack from '@/components/Stack';
import Typography from '@/components/Typography';
import { useOnClickOutside } from 'usehooks-ts';

const MenuContext = createContext<
  { close: () => void } & Pick<MenuPropsType, 'closeOnItemSelect'>
>({ close: () => {} });
const MenuProvider = MenuContext.Provider;

export type MenuAnchorPropsType = {
  /** If the menu is open */
  active: boolean;
  /** Opens the menu */
  open: () => void;
  /** Closes the menu */
  close: () => void;
};

export type MenuAnchorType = (props: MenuAnchorPropsType) => React.ReactElement;

export type MenuPropsType = HTMLProps<HTMLDivElement> & {
  /** The element the menu should originate from */
  anchor: MenuAnchorType;
  /** If the menu should be open, automatically handled if not provided */
  active?: boolean;
  /** Props for the card container */
  cardProps?: CardPropsType;
  /** Called when menu is opened */
  onOpen?: () => void;
  /** Called when menu is closed */
  onClose?: () => void;
  /** If the menu should close when an item is selected */
  closeOnItemSelect?: boolean;
  /** Disables opening menu on anchor click. Anchor can open the menu
   * using the `open` prop passed to it.
   */
  disableOpenOnClick?: boolean;
  /** Disables closing the menu through keyboard input */
  disableCloseOnEscape?: boolean;
  /** Sets alignment of the menu */
  alignment?: 'left' | 'right';
};

/** Renders a popover menu */
export function Menu({
  anchor: Anchor,
  active: propActive,
  cardProps,
  onOpen,
  onClose,
  closeOnItemSelect,
  disableOpenOnClick,
  disableCloseOnEscape,
  alignment,
  ...props
}: MenuPropsType) {
  const anchorRef = useRef<HTMLDivElement>(null);
  /** Ref for the Menu container */
  const menuRef = useRef<HTMLDivElement>(null);

  // Menu active state
  const [active, setActive] = useState(false);

  /** If the menu is open */
  const isActive = useMemo(() => propActive ?? active, [propActive, active]);

  /** Opens the menu */
  const open = useCallback(() => {
    setActive(true);
    onOpen?.();
  }, [onOpen]);

  /** Closes the menu */
  const close = () => {
    setActive(false);
    onClose?.();
  };

  /** Closes the menu when clicking outside */
  useOnClickOutside(menuRef, close);

  /** Handles keyboard input */
  const handleInput = useCallback(
    (e: React.KeyboardEvent<HTMLDivElement>) => {
      const key = e.key;
      if (key === 'Escape' && !disableCloseOnEscape) {
        e.preventDefault();
        close();
      }
      if (key === ' ' && !disableOpenOnClick) {
        e.preventDefault();
        open();
      }
    },
    [close],
  );

  /** Interactivity handlers for the anchor element*/
  const anchorHandlers = useMemo(() => {
    if (!disableOpenOnClick) {
      const onClick = open;
      const onKeyDown = handleInput;
      const tabIndex = 0;
      return { onClick, onKeyDown, tabIndex };
    }
    return {};
  }, [disableOpenOnClick, open, handleInput]);

  return (
    <MenuProvider value={{ close, closeOnItemSelect }}>
      <Stack className="relative">
        <div {...anchorHandlers} ref={anchorRef}>
          <Anchor {...{ active: isActive, open, close }} />
        </div>
        {isActive ? (
          <div
            {...props}
            ref={menuRef}
            onKeyDown={handleInput}
            style={{
              top: anchorRef.current?.offsetHeight,
              right: alignment === 'right' ? 0 : undefined,
              ...props.style,
            }}
            tabIndex={-1}
            className={
              'z-20 mt-2 w-full ' + (props.className ?? '') + ' absolute'
            }
          >
            <MenuCard {...cardProps}>{props.children}</MenuCard>
          </div>
        ) : (
          <></>
        )}
      </Stack>
    </MenuProvider>
  );
}

/** Renders a menu card */
export function MenuCard(props: CardPropsType) {
  return (
    <Card
      radius="md"
      elevation={0}
      {...props}
      className={
        'border border-surface-dark dark:border-surfaceDark-dark my-1 mx-0 max-h-32 overflow-y-auto ' +
        props.className
      }
    />
  );
}

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

/** Renders a clickable menu item */
export function MenuItem({
  text,
  icon,
  iconRight,
  onClick,
}: Readonly<MenuItemPropsType>) {
  const { close, closeOnItemSelect } = useContext(MenuContext);

  const handleClick = () => {
    onClick?.();
    closeOnItemSelect && close();
  };

  const Icon = useMemo(
    () =>
      icon && (
        <FaIcon icon={icon} className={iconRight ? 'ml-auto' : ''} size="sm" />
      ),
    [icon, iconRight],
  );

  const handleKeyDown = (e: React.KeyboardEvent<HTMLDivElement>) => {
    if (e.key === ' ') {
      e.preventDefault();
      handleClick();
    }
    const next = e.currentTarget.nextSibling as HTMLElement;
    const prev = e.currentTarget.previousSibling as HTMLElement;
    if (e.key === 'ArrowDown') {
      if (next) {
        e.preventDefault();
        next.focus();
      }
    }
    if (e.key === 'ArrowUp') {
      if (prev) {
        e.preventDefault();
        prev.focus();
      }
    }
    const isTabAndNoNext = e.key === 'Tab' && !next;
    const isShTabAndNoPrev = e.shiftKey && e.key === 'Tab' && !prev;
    if (isTabAndNoNext || isShTabAndNoPrev) {
      e.preventDefault();
      close();
    }
  };

  return (
    <Stack
      isRow
      className="py-2 px-3 cursor-pointer hover:bg-surface dark:hover:bg-surfaceDark"
      spacing={3}
      onClick={handleClick}
      tabIndex={0}
      onKeyDown={handleKeyDown}
    >
      {!iconRight && Icon}
      <Typography variant="link3" text={text} />
      {iconRight && Icon}
    </Stack>
  );
}

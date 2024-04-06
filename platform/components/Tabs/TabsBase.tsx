import React, {
  HTMLProps,
  useCallback,
  useEffect,
  useMemo,
  useState,
} from 'react';
import Stack from '../Stack';
import { useEffectOnce } from 'usehooks-ts';

/** Type for tab item */
export type TabsItemType = {
  /** Label of the tab */
  label: string;
  /** Unique ID of the tab */
  key: string;
  /** If the tab should be disabled */
  disabled?: boolean;
  /** If the tab is active. Turns into a controlled component if true. */
  active?: boolean;
};

/** Props for single tab item component */
export type TabItemPropsType<T extends HTMLElement = HTMLElement> =
  HTMLProps<T> & {
    /** Data for the tab */
    item: TabsItemType;
    /** If the tab is active */
    active: boolean;
  };

/** Type for TabsBase component props */
export type TabsBasePropsType<T extends HTMLElement = HTMLElement> = {
  /** List of tabs to render */
  tabs: TabsItemType[];
  /** Called when active tab is changed
   * @param key Key of the new active tab
   * @param prev Key of the previous active tab
   */
  onChange?: (key: string, prev: string) => void;
  /** If the tabs should take full horizontal space */
  fullWidth?: boolean;
  /** Component that shows a tab item */
  Tab: (props: TabItemPropsType<T>) => React.ReactNode;
};

/** Provides base component for horizontal tab list items.
 * Handles state management while allowing custom tab components
 * to be plugged in through the `Tab` prop.
 */
export default function TabsBase<T extends HTMLElement>({
  tabs,
  onChange,
  fullWidth,
  Tab,
}: Readonly<TabsBasePropsType<T>>) {
  const [active, setActive] = useState<string>('');
  /** If the tabs are controlled through active state of tabs */
  const isControlled = useMemo(
    () => tabs.some((t) => t.active !== undefined),
    [tabs],
  );

  /** Adjust active tab when initialized */
  useEffectOnce(() => {
    const defaultKey = isControlled ? '' : tabs[0]?.key;
    setActive(tabs.find((t) => t.active)?.key ?? defaultKey ?? '');
  });

  /** Used to set active state. Triggers onChange event */
  const updateActive = useCallback(
    (key: string) => {
      if (
        key === active ||
        tabs.length === 0 ||
        tabs.find((t) => t.key === key)?.disabled
      )
        return;
      onChange?.(key, active);
      setActive(key);
    },
    [active, onChange, tabs],
  );

  // Updates selected tab whenever tabs are changed
  useEffect(() => {
    const activeTab = tabs.find((t) => t.active);
    if (activeTab && activeTab.key !== active) {
      setActive(activeTab.key);
    }
  }, [tabs, active]);

  return (
    <Stack isRow className={`overflow-x-auto${fullWidth ? ' w-full' : ''}`}>
      {tabs.map((t) => (
        <Tab
          key={t.key}
          item={t}
          active={t.key === active}
          onClick={() => updateActive(t.key)}
        />
      ))}
    </Stack>
  );
}

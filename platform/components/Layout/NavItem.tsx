'use client';

import { usePathname, useRouter } from 'next/navigation';
import React, { useMemo } from 'react';
import Typography from '../Typography';
import { useTheme } from '@/hooks/theme';
import Stack from '../Stack';
import NavTag from './NavTag';
import { NavRouteType } from '@/routes';
import useSidebarSelector from '@/hooks/useSidebarSelector';
import { useDispatch } from 'react-redux';
import { sidebarActions } from '@/store/actions';
import Icon from '../Icon';

type P = {
  /** Called when the item is clicked */
  onClick?: () => void;
  /** Collapsed state of the item */
  collapsed?: boolean;
  /** If the item is child of a route */
  isChild?: boolean;
};

/** Shows a ListItem for sidebar navigation list */
const NavItem = ({
  id,
  icon,
  label,
  children,
  path,
  onClick,
  collapsed,
  isEnabled,
  isChild,
  tag,
}: NavRouteType & P) => {
  const { palette } = useTheme();
  const router = useRouter();
  const pathname = usePathname();
  const sidebar = useSidebarSelector();
  const dispatch = useDispatch();

  /** If the nav item is active */
  const active = useMemo(() => {
    return (
      (pathname && path?.startsWith(pathname)) ||
      label.toLowerCase() === sidebar?.mainPath ||
      (sidebar.path !== undefined && sidebar.path === label.toLowerCase())
    );
  }, [path, pathname, label]);

  const expanded = useMemo(() => {
    return children && label.toLowerCase() === sidebar?.mainPath;
  }, [label, children]);

  const colorClass = useMemo(() => {
    if (isEnabled) {
      return active ? 'text-accent' : 'text-white';
    }
    return '!text-typographyDark-muted';
  }, [isEnabled, active]);

  const handleClick = () => {
    if (!isEnabled) return;
    if (onClick) onClick();
    if (path) {
      dispatch(sidebarActions.changePath({ path: label.toLowerCase() }));
      router.push(path);

    }
  };

  const [childClasses, childTextColor] = useMemo(() => {
    if (!isEnabled)
      return ['border-l border-transparent', palette.typography.muted];
    if (isChild) {
      if (active) return ['border-l-2 border-accentDark', 'white'];
      else
        return [
          'border-l border-typographyDark-muted',
          palette.typography.muted,
        ];
    }
    if (active) return ['text-accentDark', 'white'];
    return ['', null];
  }, [isChild, active, palette.typography.muted, isEnabled]);

  return (
    <Stack className={'py-1.5 ' + childClasses}>
      <button onClick={handleClick} className="cursor-pointer">
        <Stack isRow className="flex-grow items-center">
          {icon && (
            <Icon
              icon={icon}
              className={colorClass + ' w-4 h-4'}
              disableDefaultColor
            />
          )}
          {!collapsed && (
            <Stack
              isRow
              className="ml-2 flex-grow space-x-2 md:space-x-0 md:justify-between"
            >
              <Stack>
                <Typography color={childTextColor ?? 'white'} text={label} />
                {!isEnabled && (
                  <Typography
                    color={childTextColor ?? 'white'}
                    text={'( Coming Soon )'}
                  />
                )}
              </Stack>
              {tag && <NavTag tag={tag} />}
            </Stack>
          )}
        </Stack>
      </button>
      {expanded && (
        <Stack className="pl-8 mt-2">
          {children?.map((v) => <NavItem key={v.id} {...v} isChild />)}
        </Stack>
      )}
    </Stack>
  );
};

export default NavItem;

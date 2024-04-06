'use client';

import React, { useMemo } from 'react';
import Logo from '@/components/Logo';
import {
  faGgCircle
} from '@fortawesome/free-brands-svg-icons';
import NavItem from './NavItem';
import { useTheme } from '@/hooks/theme';
import { useBreakpointMatch } from '@/hooks/useBreakpointMatch';
import { useParams, useRouter } from 'next/navigation';
import { NavRouteType, appRoutes } from '@/routes';
import { useConfigSelector } from '@/hooks/store';
import Icon from '../Icon';

type P = {
  /** Routes to show in sidebar */
  routes: NavRouteType[];
  /** If the sidebar is open */
  open?: boolean;
  /** Called when sidebar is closed */
  onClose?: () => void;
};

/** Sidebar for the logged in user layout */
const Sidebar = ({ routes, open, onClose }: P) => {
  const router = useRouter();
  const id = useParams()?.id as string;
  const { sidebarCollapsed: collapsed } = useConfigSelector();
  const {
    dimensions: { sidebar, headerMobile },
  } = useTheme();
  const isMd = useBreakpointMatch('md');

  /** Routes to show at the top/bottom of the list */
  const [topRoutes, bottomRoutes] = useMemo(() => {
    const top: NavRouteType[] = [];
    const bottom: NavRouteType[] = [];
    routes.forEach((r) => {
      if (r.isBottom) bottom.push(r);
      else {
        const routeList = { ...r };

        top.push(routeList);
      }
    });
    return [top, bottom];
  }, [routes, id]);

  const containerClasses = useMemo(() => {
    const base = [
      'fixed top-0 left-0 flex flex-col bg-surfaceDark-light dark:bg-surfaceDark h-screen w-screen px-4 md:p-6 md:pt-10 z-50',
    ];
    if (!isMd && !open) base.push('hidden');
    return base.join(' ');
  }, [open, isMd]);

  const navigation = () => {
    onClose?.();
  };

  return (
    <div
      className={containerClasses}
      style={{width:sidebar.width}}
    >
      <div
        className="flex cursor-pointer items-center justify-between mb-4 py-1 md:py-0 md:mb-9 border-b md:border-b-0 border-surfaceDark-dark"
        style={isMd ? {} : { height: headerMobile.height }}
      >
        <button onClick={navigation}>
          <Logo height={21} dark={false} wide />
        </button>
      </div>

      <div className="flex flex-col space-y-1 5 md:space-y-3">
        {topRoutes.map((r) => (
          <NavItem onClick={onClose} key={r.id} {...r} collapsed={collapsed} />
        ))}
      </div>
      <div className="my-6 md:my-0 md:pt-6 md:mt-auto border-t border-surfaceDark-dark flex flex-col" />
      <div className="flex flex-col space-y-1 5 md:space-y-3">
        {bottomRoutes.map((r) => (
          <NavItem onClick={onClose} key={r.id} {...r}  collapsed={collapsed} />
        ))}
      </div>
    </div>
  );
};

export default Sidebar;

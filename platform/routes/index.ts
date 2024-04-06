import { IconDefinition, faGoogleWallet, faRust, faShopware } from '@fortawesome/free-brands-svg-icons';

/** Base app route type */
export type RouteType = {
  /** ID of the route */
  id: string;
  /** Label of the route */
  label: string;
  /** Path of the route */
  path?: string;
};

/** Sidebar route type */
export type NavRouteType = RouteType & {
  /** Default icon */
  icon?: IconDefinition;
  /** Children of the route. Collapsed by default, can be expanded. */
  children?: NavRouteType[];
  /** If the route should be anchored at the bottom of the sidebar. */
  isBottom?: boolean;
  /** If the route should be enabled in the sidebar. */
  isEnabled?: boolean;
  /** Tag shown along with the nav link indicating status as a badge */
  tag?: 'beta';
};


/** Contains a list of pages for routing. Object structure based on the file structure of app/pages dir.
 * A route (except root) should use self to refer to its own data and its children should be keyed objects.
 */
export const appRoutes = {
  root: {
    path: '/',
    label: 'Dashboard',
  },
} as const;

const navRoutes: NavRouteType[] = [
  {
    id: 'dashboard',
    isEnabled: true,
    icon: faRust,
    label: 'Dashboard',
    path: '/dashboard',
  },
  {
    id: 'marketplace',
    label: 'Marketplacae',
    icon: faShopware,
    isEnabled: true,
    path: '/marketplace',
  },
  {
    id: 'dextokens',
    icon: faGoogleWallet,
    label: 'Dex Tokens',
    isEnabled: true,
    path: '/dex',
  },
];

export { navRoutes };
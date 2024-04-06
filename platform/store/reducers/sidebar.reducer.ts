import { ActionType } from '../actions/action.type';
import { SidebarActionType } from '../actions/sidebar.actions';
import { ISidebarState } from '../state/sidebar.state';

/** Initial auth state */
const initialState = {} as ISidebarState;

/**
 * Sidebar reducer function
 * @param state - Current auth state
 * @param action - Dispatched action
 * @returns Updated auth state
 */
export default function SidebarReducer(
  state = initialState,
  action: SidebarActionType | ActionType<'', undefined>,
) {
  switch (action.type) {
    case 'path':
      return { path: action.payload };
    case 'subpath':
      return { path: action.payload?.path, mainPath: action.payload?.mainPath };
    default:
      return state;
  }
}

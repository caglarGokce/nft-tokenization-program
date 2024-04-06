import { IConfigState } from '@/store/state/config.state';
import { ActionType } from '../actions/action.type';
import { ConfigActionType } from '../actions/config.actions';

/** Initial config state */
const initialState = {
  dark: true,
  sidebarCollapsed: false,
} as IConfigState;

/**
 * Config reducer function
 * @param state - Current config state
 * @param action - Dispatched action
 * @returns Updated config state
 */
export default function ConfigReducer(
  state = initialState,
  action: ConfigActionType | ActionType<'', undefined>,
) {
  switch (action.type) {
    case 'config/set':
      return { ...state, ...action.payload };
    case 'config/reset':
      return initialState;
    default:
      return state;
  }
}

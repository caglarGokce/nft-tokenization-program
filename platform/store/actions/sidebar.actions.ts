import { ActionType } from './action.type';

type SidebarPathPayloadType = {
  path: string;
};

/** Action type for path */
type SidebarPathActionType = ActionType<'path', SidebarPathPayloadType>;

type SidebarSubPathPayloadType = {
  path: string;
  mainPath: string;
};

/** Action type for subpath */
type SidebarSubpathActionType = ActionType<
  'subpath',
  SidebarSubPathPayloadType
>;

/** Union of all auth action types */
export type SidebarActionType =
  | SidebarSubpathActionType
  | SidebarPathActionType;

/** Sidebar action creators */
const sidebarActions = {
  /**
   * Sets path of sidebar state
   * @param payload - Path
   * @returns New path of sidebar
   */
  changePath: (payload: SidebarPathPayloadType): SidebarPathActionType => {
    return {
      type: 'path',
      payload,
    };
  },
  /**
   * Sets sub path of sidebar state
   * @param payload - Sub path
   * @returns New sub path of sidebar
   */
  changeSubPath: (
    payload: SidebarSubPathPayloadType,
  ): SidebarSubpathActionType => {
    return {
      type: 'subpath',
      payload,
    };
  },
};

export default sidebarActions;

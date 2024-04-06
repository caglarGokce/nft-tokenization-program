/** Interface for sidebar state */
export interface ISidebarState {
  /** Data of current sub path */
  path?: string;
  /** Data of current main path */
  mainPath?: string;
}

/** Authorization state of the user */
export default class SidebarState implements ISidebarState {
  path?: string;
  mainPath?: string;

  constructor(obj?: Partial<SidebarState>) {
    if (obj) {
      this.path = obj.path;
      this.mainPath = obj.mainPath;
    }
  }

  /** Initializes an Auth state from server JSON response. */
  static fromJson(data: any) {
    const sidebar = new SidebarState(data);
    return sidebar;
  }
}

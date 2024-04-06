/** Interface for the config state */
export interface IConfigState {
  /** If dark mode is enabled */
  dark: boolean;
  /** If sidebar should be collapsed */
  sidebarCollapsed: boolean;
}

/** Config state used to manage user preferences */
export default class ConfigState implements IConfigState {
  dark: boolean;
  sidebarCollapsed: boolean;

  constructor(obj?: Partial<IConfigState>) {
    this.dark = obj?.dark ?? true;
    this.sidebarCollapsed = obj?.sidebarCollapsed ?? false;
  }
}

import ConfigState from './config.state';
import SidebarState from './sidebar.state';

export type AppStateType = {
  config: ConfigState;
  sidebar: SidebarState;
};

export {
  ConfigState,
  SidebarState,
};

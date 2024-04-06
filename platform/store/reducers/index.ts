import { combineReducers } from 'redux';
import ConfigReducer from './config.reducer';
import SidebarReducer from './sidebar.reducer';

const Reducer = combineReducers({
  config: ConfigReducer,
  sidebar: SidebarReducer,
});

export default Reducer;

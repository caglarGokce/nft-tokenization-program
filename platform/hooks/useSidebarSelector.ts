import { AppStateType, SidebarState } from '@/store/state';
import { useSelector } from 'react-redux';

const useSidebarSelector = () => {
  const sidebar = useSelector<AppStateType, SidebarState>(
    (store) => store.sidebar,
  );
  return sidebar;
};

export default useSidebarSelector;

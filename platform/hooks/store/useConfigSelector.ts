import { useSelector } from 'react-redux';
import { AppStateType, ConfigState } from '@/store/state';

const useConfigSelector = () => {
  const config = useSelector<AppStateType, ConfigState>(
    (store) => store.config,
  );
  return config;
};

export default useConfigSelector;

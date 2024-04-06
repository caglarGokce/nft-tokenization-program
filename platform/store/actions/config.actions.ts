import { IConfigState } from '@/store/state/config.state';
import { ActionType } from './action.type';

/** Payload type for config/set action */
type ConfigSetPayloadType = Partial<IConfigState>;

/** Action type for config/set */
type ConfigSetActionType = ActionType<'config/set', ConfigSetPayloadType>;

/** Action type for config/reset */
type ConfigResetActionType = ActionType<'config/reset'>;

/** Union of config action types */
export type ConfigActionType = ConfigSetActionType | ConfigResetActionType;

/** Config action creators */
const configActions = {
  /**
   * Updates the config with given state
   * @param payload - Config state with properties to update
   * @returns Config set action
   */
  set: (payload: ConfigSetPayloadType): ConfigSetActionType => {
    return {
      type: 'config/set',
      payload,
    };
  },

  /**
   * Resets the config to initial state
   * @returns Config reset action
   */
  reset: (): ConfigResetActionType => {
    return {
      type: 'config/reset',
    };
  },
};

export default configActions;

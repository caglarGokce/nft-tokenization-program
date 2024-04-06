import { ActionType } from './action.type';

type SearchbarPayloadType = {
  text: string;
};

/** Action type for path */
export type SearchbarActionType = ActionType<'search', SearchbarPayloadType>;


/** Searchbar action creators */
const searchbarActions = {
  /**
   * Sets path of searchbar state
   * @param payload - Path
   * @returns New path of searchbar
   */
  textChange: (payload: SearchbarPayloadType): SearchbarActionType => {
    return {
      type: 'search',
      payload,
    };
  },
};

export default searchbarActions;

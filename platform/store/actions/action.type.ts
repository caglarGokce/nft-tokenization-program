/** Base type for Redux store actions */
export type ActionType<Type, Payload = undefined> = {
  /** Type of the action */
  type: Type;
  /** Payload of the action */
  payload?: Payload;
};

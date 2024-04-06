/** Base type for graph parameters */
type BaseType = {
  /** Name of the parameter */
  name: string;
  /** Base graph endpoint to fetch param data from*/
  ep: string;
};

/** Graph parameters type */
export type GraphParamType = BaseType & {
  /** Key used to access param in container */
  key: string;
  /** Child parameters */
  children?: GraphParamType[];
};

/** Graph parameters category (parent) type */
export type GraphParamCategoryType = BaseType & {
  /** Parameters or categories in the category */
  children: GraphParamType[] | GraphParamCategoryType[];
};

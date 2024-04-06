/** Type for DropDown component options */
export type DropdownOptionType<V extends string | number> = {
  /** Name of the option */
  name: string;
  /** Value of the option */
  value: V;
};

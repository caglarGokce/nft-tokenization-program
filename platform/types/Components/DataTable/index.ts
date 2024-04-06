import { ReactElement } from 'react';

type ColumnCellPropsType<T> = {
  /** Data of the row */
  row: T;
  /** Index of the row */
  index: number;
};

type ColumnHeaderPropsType<T> = {
  /** The column this header is for  */
  column: DataTableColumnType<T>;
};

type CellReturnType = ReactElement | ReactElement[] | string;

export type DataTableColumnType<T> = {
  /** ID of the column. Used as a unique identifier for the
   * column. Assigned `key` by default.
   */
  id?: string;
  /** Key in the data object representing this column's data. Used
   * to render the cell value by default
   */
  key?: keyof T;
  /** Title of the column shown in header */
  title: string;
  /** Styling to apply to this column
   * @param row Data of the row
   * @param index Index of the row
   */
  cellStyle?: (row: T, index: number) => string;
  /** Custom element to render for this column
   * @param row Data of the row
   * @param index Index of the row
   * @deprecated use `cell` prop instead
   */
  cellRender?: (row: T, index: number) => CellReturnType;
  /** Custom element to render for this column's row cells */
  cell?: (props: ColumnCellPropsType<T>) => CellReturnType;
  /** Show tooltip along with header */
  hint?: string;
  /** Called when the header of this column is clicked */
  onHeaderClick?: () => void;
  /** Custom element to render header for this column
   * @param column The column this header is for
   * @deprecated use `header` prop instead
   */
  cellHeaderRender?: (column: DataTableColumnType<T>) => CellReturnType;
  /** Custom element to render this column's header cell */
  header?: (props: ColumnHeaderPropsType<T>) => CellReturnType;
  /** Property in the data used to sort this column. Supports dot
   * notation for nested objects.
   * @example
   * "person.name" // sorts based on name property of person object
   */
  sortKey?: string;
};

export type RowDataPropsType<T> = {
  /** Data of the row */
  row: T;
  /** Index of the row */
  index: number;
};

export type DataTablePropsType<T> = {
  /** List of Columns for the table */
  cols: DataTableColumnType<T>[];
  /** Data for the table */
  data?: T[];
  /** Indicator for loading */
  loading?: boolean;
  /** Error to show */
  error?: Error;
  /** Action performed on clicking a row
   * @param row Data of the row
   * @param index Index of the row
   */
  onRowClick?: (row: T, index: number) => void;
  /** Styling to apply to a row
   * @param row Data of the row
   * @param index Index of the row
   */
  rowStyle?: (row: T, index: number) => string;
  /** Message to show when there is no data */
  noDataMessage?: string;
  /** Variant for card container */
  elevated?: boolean;
  /** Appends another row below this row with given node as content */
  rowChild?: (props: RowDataPropsType<T>) => React.ReactNode;
  /** True if header is rounded */
  rounded?: boolean;
};

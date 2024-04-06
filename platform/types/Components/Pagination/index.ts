export type PaginationPropsType = {
  /** Current page number */
  page: number;
  /** Total Records */
  total: number;
  /** Records per page */
  perPage: number;
  /** Method called on page change
   * @param p The new page number
   */
  onPageChange: (p: number) => void;
  /** If the data is loading. Disables pagination. */
  loading?: boolean;
};

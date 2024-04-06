import {
  DataTableColumnType,
  DataTablePropsType,
} from '@/types/Components/DataTable';
import React, { ReactNode, useContext, useMemo, useState } from 'react';
import { useTheme } from '@/hooks/theme';
import Tooltip from '../Tooltip';
import { faInfoCircle } from '@fortawesome/free-solid-svg-icons';
import Typography from '../Typography';
import Icon from '../Icon';
import Stack from '../Stack';
import { faSort, faSortUp, faSortDown } from '@fortawesome/free-solid-svg-icons';
import { once } from 'lodash';
import { extractNestedPropertyValue } from '@/helpers/utils/paramExtractor';

/** Type for possible sorting directions */
type SortingDirection = 'asc' | 'desc' | 'default';

/** Type for sorting configuration on a Datatable column */
type SortingConfig = {
  sortKey: string;
  direction: SortingDirection;
};

/** Context type for DataTable components */
type ContextType<T> = Omit<DataTablePropsType<T>, 'data'> & {
  sortedData?: T[];
  sortingConfig: SortingConfig | null;
  setSortingConfig: React.Dispatch<React.SetStateAction<SortingConfig | null>>;
};

/** Creates an instance of the Datatable context */
const createTableContext = once(<T,>() =>
  React.createContext({} as ContextType<T>),
);

/** Hook to use the DataTable context */
export const useTableContext = <T,>() => useContext(createTableContext<T>());

/** Component that renders a table view based on given columns and data */
const DataTable = <T,>({ data, ...props }: DataTablePropsType<T>) => {
  const DataTablePropsContext = createTableContext<T>();

  /** State for sorting configuration */
  const [sortingConfig, setSortingConfig] = useState<SortingConfig | null>({
    sortKey: '',
    direction: 'default',
  });

  /** The sorted data based on current sorting configuration state */
  const sortedData = useMemo(() => {
    // If no sorting configuration is available, return the data as is
    if (!sortingConfig || sortingConfig.direction === 'default') {
      return data;
    }

    // If data is available, sort the data based on the sorting configuration
    if (data) {
      return data.sort((a, b) => {
        // Extract the values of the property to be sorted
        const valueA = extractNestedPropertyValue(a, sortingConfig.sortKey);
        const valueB = extractNestedPropertyValue(b, sortingConfig.sortKey);

        // If the values are numbers, sort them using number math
        if (typeof valueA === 'number' && typeof valueB === 'number') {
          return sortingConfig.direction === 'asc'
            ? valueA - valueB
            : valueB - valueA;
        }

        // If the values are strings, sort them using locale compare
        const strA = String(valueA).toLowerCase();
        const strB = String(valueB).toLowerCase();

        return sortingConfig.direction === 'asc'
          ? strA.localeCompare(strB)
          : strB.localeCompare(strA);
      });
    }
  }, [data, sortingConfig]);

  // Create the context value
  const contextValue = useMemo(
    () => ({ ...props, sortedData, sortingConfig, setSortingConfig }),
    [props, sortedData, sortingConfig],
  );

  return (
    <DataTablePropsContext.Provider value={contextValue}>
      <div className="overflow-x-auto">
        <table className="min-w-full border-collapse">
          <Header />
          <Body />
        </table>
      </div>
    </DataTablePropsContext.Provider>
  );
};

export default DataTable;

/** Component that renders the body of the DataTable in case there's no data or data is loading */
const NoDataBody = () =>
    <NoDataText />
  

/** Component that renders the text for when no data is available */
const NoDataText = () => {
  const { noDataMessage } = useTableContext();
  const text = noDataMessage ?? 'No data available';

  return (
    <Stack className="items-center">
      <Typography text={text} />
    </Stack>
  );
};

/** Component that renders the header row of a datatable */
const Header = () => {
  const { cols, rounded } = useTableContext();
  const roundedBorder = rounded ? 'rounded-sm' : '';

  return (
    <thead className={roundedBorder}>
      <tr className={`dark:bg-surfaceDark bg-surface ${roundedBorder}`}>
        {cols.map((c, i) => (
          <HeaderCol column={c} index={i} key={c.id ?? c.key} />
        ))}
      </tr>
    </thead>
  );
};

/** Type for the props of the header column */
type ColumnPropsType<T> = { column: DataTableColumnType<T>; index: number };

/** Component that renders the header column of a datatable */
const HeaderCol = <T,>({ column, index }: ColumnPropsType<T>) => {
  const { sortingConfig, setSortingConfig, rounded, cols } = useTableContext();
  const colId = column.id ?? (column.key as string);

  /** Handles header click
   * @param c Column that was clicked
   */
  const handleHeaderClick = () => {
    // If the column has a sort key, set the sorting configuration
    const sortKey = column.sortKey;
    if (sortKey) {
      setSortingConfig((prevConfig) => {
        if (sortingConfig?.sortKey === sortKey) {
          let dir = prevConfig?.direction;
          dir = dir === 'asc' ? 'desc' : 'asc';
          // Toggle the direction if the same column is clicked again
          return {
            ...prevConfig,
            sortKey,
            direction: dir ?? 'default',
          };
        }
        // Set the new column and default direction for a different column
        return { sortKey, direction: 'asc' };
      });
    }
    // Call the onHeaderClick prop function if available
    column.onHeaderClick?.();
  };

  /** Classes for the header column */
  const className = useMemo(() => {
    const base = [
      'px-4 py-2 text-sm font-semibold text-left dark:text-primaryDark text-primary',
    ];
    // Add rounded classes if the column is the first or last and rounded prop is provided
    if (rounded) {
      if (index === 0) base.push('rounded-tl-sm rounded-bl-sm');
      if (index === cols.length - 1) base.push('rounded-tr-sm rounded-br-sm');
    }
    return base.join(' ');
  }, [rounded, index, cols]);

  return (
    <th key={colId} onClick={handleHeaderClick} className={className}>
      <Stack
        isRow
        spacing={1.5}
        className={`items-center ${column.sortKey && 'cursor-pointer'}`}
      >
        <HeaderCell column={column} index={index} />
        {column.sortKey && (
          <SortIcon key={column.sortKey} sortKey={column.sortKey} />
        )}
      </Stack>
    </th>
  );
};

/** Component that renders the header cell of a datatable */
const HeaderCell = <T,>({ column }: ColumnPropsType<T>) => {
  const { palette } = useTheme();

  /** The custom header created through prop input */
  const CustomHeader =
    column.cellHeaderRender?.(column) ?? column.header?.({ column });

  /** If the custom header is a string to be used as a title */
  const customHeaderIsString = typeof CustomHeader === 'string';

  /** The title of the column */
  const title = customHeaderIsString ? CustomHeader : column.title;

  /** The text component to be rendered */
  const Text = <Typography text={title} variant="subtitle2" />;

  /** The default column with a tooltip if hint is available. Just text otherwise. */
  const DefaultColumn = column.hint ? (
    <Stack isRow spacing={1} className="items-center">
      {Text}
      <Tooltip text={column?.hint}>
        <Icon icon={faInfoCircle} size="sm" color={palette.surface.muted} />
      </Tooltip>
    </Stack>
  ) : (
    Text
  );

  /** The column to be rendered */
  const Column =
    !customHeaderIsString && CustomHeader ? CustomHeader : DefaultColumn;

  return <>{Column}</>;
};

/** Props for the SortIcon component */
type SortIconProps = {
  /** Sort key for the column */
  sortKey?: string;
};

/** Shows sort icon for datatable header title */
const SortIcon = ({ sortKey }: SortIconProps) => {
  const { sortingConfig } = useTableContext();
  /** Icon based on sorting direction */
  let icon = faSort;
  /** Classes for the icon */
  let className = 'opacity-25';

  // If the sorting configuration is available and the sort key matches, update icon and classes
  if (sortingConfig && sortKey === sortingConfig.sortKey) {
    const dir = sortingConfig.direction;
    // Set the icon based on the sorting direction
    if (dir === 'asc') {
      icon = faSortUp;
    } else {
      icon = faSortDown;
    }
    // Set the classes to make the icon prominent, indicating the sorting is active
    className = 'opacity-100';
  }

  return <Icon icon={icon} size="sm" className={className} />;
};

/** Component that renders the body of the DataTable */
const Body = () => {
  const { loading, error, sortedData } = useTableContext();

  // If no data is available, return an empty fragment
  if (!sortedData) return <></>;

  return (
    <tbody>
      {!loading && !error && sortedData ? (
        sortedData.map((data, index) => <Row key={index} {...{ data, index }} />)
      ) : (
        <NoDataBody  />
      )}
    </tbody>
  );
};

/** Type for the props of the Row component */
type TableRowPropsType<T> = {
  /** Row data */
  data: T;
  /** Row index */
  index: number;
};

/** Component that renders a row of the DataTable */
const Row = <T,>({ data, index }: TableRowPropsType<T>) => {
  const { onRowClick, rowStyle, cols, rowChild } = useTableContext<T>();

  /** Props for the row */
  const rowProps = onRowClick
    ? {
        onClick: () => onRowClick(data, index),
        className:
          'cursor-pointer hover:bg-surface-dark dark:hover:bg-surfaceDark-dark',
      }
    : {};

  return (
    <React.Fragment>
      <tr
        className={`border-b mt-1 dark:border-surfaceDark-dark border-surface-dark ${
          rowStyle ? rowStyle(data, index) : ''
        }`}
        {...rowProps}
      >
        {cols.map((c) => (
          <Cell
            key={c.id ?? (c.key as string)}
            column={c}
            data={data}
            index={index}
          />
        ))}
      </tr>
      {rowChild && (
        <tr>
          <td colSpan={cols.length}>{rowChild({ row: data, index })}</td>
        </tr>
      )}
    </React.Fragment>
  );
};

/** Type for the props of the Cell component */
type CellProps<T> = TableRowPropsType<T> & ColumnPropsType<T>;

/** Component that renders a cell of the DataTable */
const Cell = <T,>({ column, data, index }: CellProps<T>) => {
  /** The default node for the cell */
  const DefaultNode = column.key ? (data[column.key] as ReactNode) : <></>;
  /** Custom node for the cell based on if a custom cell rendering prop is provided*/
  const CustomNode =
    column.cell?.({ row: data, index }) ?? column.cellRender?.(data, index);

  return (
    <td
      id={index.toString()}
      className={`p-2 md:px-4 text-sm border-none dark:text-primaryDark text-primary ${
        column.cellStyle ? column.cellStyle(data, index) : ''
      }`}
    >
      {CustomNode ?? DefaultNode}
    </td>
  );
};

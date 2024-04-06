import { useTheme } from '@/hooks/theme';
import {
  DataTableColumnType,
  DataTablePropsType,
  RowDataPropsType,
} from '@/types/Components/DataTable';
import React, { useContext, useEffect, useMemo, useState } from 'react';
import { useWindowSize } from 'usehooks-ts';
import DataTable from '../DataTable';
import Stack from '../Stack';
import Card, { CardPropsType } from '../Card';
import Typography from '../Typography';
import Divider from '../Divider';
import Icon from '../Icon';
import { faChevronDown } from '@fortawesome/free-solid-svg-icons';
import { cloneDeep, once } from 'lodash';

/** Classes for selected row */
const selectedRowClasses =
  'bg-surface-dark dark:bg-surfaceDark-dark text-typography dark:text-typographyDark rounded-sm';

type ContextType<T> = Pick<
  DataTableDetailFlowPropsType<T>,
  'rowChildContainerCardProps' | 'rowChild'
> & {
  /** Index of the row currently selected */
  selectedRow: number;
};

const createPropsContext = once(<T,>() =>
  React.createContext({} as ContextType<T>),
);

export const usePropsContext = <T,>() => useContext(createPropsContext<T>());

export type DataTableDetailFlowPropsType<T> = DataTablePropsType<T> &
  Required<Pick<DataTablePropsType<T>, 'rowChild'>> & {
    /** Forces accordion based layout on large devices */
    forceAccordion?: boolean;
    /** Props for the rowChild container Card */
    rowChildContainerCardProps?: CardPropsType;
  };

/** Shows a table with selectable elements to load detail */
export default function DataTableDetailFlow<T>({
  rowChild,
  onRowClick,
  rowStyle,
  data,
  cols,
  forceAccordion,
  rowChildContainerCardProps: containerProps,
  ...props
}: DataTableDetailFlowPropsType<T>) {
  const { screens } = useTheme();
  const screen = useWindowSize();
  const [selectedRow, setSelectedRow] = useState(-1);
  /** If the screen width is small or accordion layout is forced */
  const [isSmall, setIsSmall] = useState(!!forceAccordion);

  const PropsContext = createPropsContext<T>();

  /** Update isSmall if screen size changes */
  useEffect(() => {
    const small = screen.width < screens.lg;
    setIsSmall((isSmall) => {
      if (forceAccordion) return true;
      if (!isSmall && small) return true;
      else if (isSmall && !small) return false;
      return isSmall;
    });
  }, [screen.width, screens.lg, forceAccordion]);

  // Reset selected row whenever data updates
  useEffect(() => {
    setSelectedRow(-1);
  }, [data]);

  /** Cols adjusted to add accordion icon for small layouts */
  const adjustedCols = useMemo(() => {
    /** Transformed columns */
    let outCols: DataTableColumnType<T>[] = cloneDeep(cols);
    if (isSmall) {
      /** Column for Accordion icon */
      const iconCol: DataTableColumnType<T> = {
        title: '',
        cell: Cell,
        cellStyle: () => 'w-4',
      };
      /** Transformed columns */
      outCols = [iconCol, ...cols];
    } else {
      // Adjust text style for large layout
      outCols[0].cellStyle = (r, i) => {
        const classes = [];
        if (cols[0].cellStyle) classes.push(cols[0].cellStyle(r, i));
        if (i === selectedRow) classes.push('font-medium');
        return classes.join(' ');
      };
    }
    return outCols;
  }, [cols, selectedRow, isSmall]);

  const contextValue = useMemo(
    () => ({
      selectedRow,
      rowChild,
      rowChildContainerCardProps: containerProps,
    }),
    [selectedRow, rowChild, containerProps],
  );

  return (
    <PropsContext.Provider value={contextValue}>
      <Stack isRow spacing={2}>
        <div className={isSmall ? 'w-full' : 'w-1/2'}>
          <DataTable
            {...props}
            data={data}
            cols={adjustedCols}
            onRowClick={(r, i) => {
              if (selectedRow === i) setSelectedRow(-1);
              else setSelectedRow(i);
              onRowClick?.(r, i);
            }}
            rowStyle={(r, i) => {
              const classes = [];
              if (rowStyle) classes.push(rowStyle(r, i));
              const isSelected = i === selectedRow && !isSmall;
              if (isSelected) classes.push(selectedRowClasses);
              return classes.join(' ');
            }}
            rowChild={isSmall && selectedRow > -1 ? RowChild : undefined}
          />
        </div>
        {isSmall ? (
          <></>
        ) : (
          <RowChildCard
            {...{
              rowChild,
              rowChildContainerCardProps: {
                ...containerProps,
                className: ['p-0 w-1/2', containerProps?.className ?? ''].join(
                  ' ',
                ),
              },
              selectedRow,
              data,
            }}
          />
        )}
      </Stack>
    </PropsContext.Provider>
  );
}

/** Handles rendering cell as accordion */
const Cell = <T,>({ row: _, index }: RowDataPropsType<T>) => {
  return <SmallCellAccordion index={index} />;
};

/** Handles rowChild prop */
const RowChild = <T,>({ row, index }: RowDataPropsType<T>) => {
  const {
    selectedRow,
    rowChildContainerCardProps: containerProps,
    rowChild,
  } = usePropsContext();

  return index === selectedRow ? (
    <SmallRowChildCard
      children={rowChild({ row, index })}
      props={containerProps}
    />
  ) : undefined;
};

type SmallCellAccordionPropsType = { index: number };

const SmallCellAccordion = ({ index }: SmallCellAccordionPropsType) => {
  const { selectedRow } = usePropsContext();

  return (
    <Icon
      icon={faChevronDown}
      className={index === selectedRow ? 'rotate-180' : 'rotate-0'}
    />
  );
};

type SmallRowChildCardPropsType = {
  children: React.ReactNode;
  props?: CardPropsType;
};

const SmallRowChildCard = ({ children, props }: SmallRowChildCardPropsType) => (
  <Card elevation={1} {...props}>
    {children}
  </Card>
);

type RowChildCardPropsType<T> = Pick<
  DataTableDetailFlowPropsType<T>,
  'data' | 'rowChild' | 'rowChildContainerCardProps'
> & { selectedRow: number };

const RowChildCard = <T,>({
  rowChild,
  data,
  selectedRow,
  rowChildContainerCardProps: props,
}: RowChildCardPropsType<T>) => {
  return (
    <Card elevation={1} {...props}>
      <Stack>
        <div className="my-2 mx-4">
          <Typography variant="subtitle2" text="Description" />
        </div>
        <div className="w-full">
          <Divider />
        </div>
        <div className="m-6 mt-4">
          {data && selectedRow > -1 ? (
            rowChild({ row: data[selectedRow], index: selectedRow })
          ) : (
            <Typography text="Select a row to view detail." />
          )}
        </div>
      </Stack>
    </Card>
  );
};

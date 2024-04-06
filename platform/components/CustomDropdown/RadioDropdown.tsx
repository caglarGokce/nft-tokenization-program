import React, { useMemo, useState } from 'react';
import { v4 as uuid } from 'uuid';
import Stack from '../Stack';
import Typography from '../Typography';
import Card from '../Card';
import Icon from '../Icon';
import { faChevronDown, faSearch } from '@fortawesome/free-solid-svg-icons';
import TextField from '../Form/TextField';

export type DropdownRadioItems = {
  value: string;
  key: string;
};
type P = {
  /** Data Array for dropdown */
  data: DropdownRadioItems[];
  /** Title of dropdown */
  title: string;
  /** Selected value for dropdown radio */
  selected: string;
  /** Action performed on clicking a row
   * @param value selected value of dropdown
   * @param index Index of the selected value object
   */
  handleChange: (value: string, index: number) => void;
  /** Show textfield filter for options */
  textfield?: boolean;
};

const RadioDropdown: React.FC<P> = ({
  title,
  data,
  selected,
  handleChange,
  textfield,
}) => {
  const [isOpen, setIsOpen] = useState<boolean>(false);
  const [search, setSearch] = useState('');

  const filteredData = useMemo(() => {
    return search
      ? data?.filter(
          (d) => d.value.toLowerCase()?.includes(search.toLowerCase()),
        )
      : data;
  }, [search, data]);

  const toggleDropdown = () => {
    setIsOpen((old) => !old);
  };

  const handleSelect = (value: string, index: number) => {
    handleChange(value, index);
    setIsOpen(false);
  };

  const dropdownVisibilityClass = isOpen ? 'flex' : 'hidden';

  const backdrop = () => (
    <button
      className="fixed top-0 right-0 bottom-0 left-0 z-[99999999] bg-transparent"
      onClick={toggleDropdown}
    />
  );

  return (
    <Stack>
      <Stack className="relative">
        {/* Dropdown header */}
        <button onClick={toggleDropdown}>
          <Card
            elevation={0}
            className="px-3 py-2 border dark:border-surfaceDark-dark border-surface-dark min-w-24 flex items-center justify-between"
            radius="xl"
          >
            <Typography text={title} variant="button2" />
            <Icon
              size="md"
              icon={faChevronDown}
              className={`dark:text-surface-light text-surfaceDark-light ${
                isOpen ? 'rotate-180' : 'rotate-0'
              } duration-300`}
            />
          </Card>
        </button>
        {/* Dropdown content */}
        <Card
          elevation={0}
          radius="md"
          className={`absolute flex flex-col border dark:border-surfaceDark-dark border-surface-dark top-10 right-0 z-[999999991] ${
            textfield ? 'w-52' : 'w-44'
          }  p-3 pb-2 ${dropdownVisibilityClass}`}
          onClick={(e) => e?.stopPropagation()}
        >
          {textfield && (
            <TextField
              rounded
              inputProps={{ placeholder: 'Search', className: 'w-full' }}
              wrapperProps={{
                className: 'mb-4 !bg-surface !dark:bg-surfaceDark border-none',
              }}
              value={search}
              onTextChange={(e) => setSearch(e)}
            />
          )}
          <Stack className={`max-h-28 overflow-auto`}>
            {filteredData?.map((item, index) => (
              <Stack
                isRow
                key={uuid()}
                className="space-x-2 pb-2 cursor-pointer"
                onClick={() => handleSelect(item?.value, index)}
              >
                <input
                  name={title}
                  type="radio"
                  value={item?.value}
                  className="accent-surfaceDark-dark"
                  checked={selected === item?.value}
                  onChange={() => {}}
                />
                <Typography text={item?.value} variant="button2" />
              </Stack>
            ))}
          </Stack>
        </Card>
      </Stack>
      {isOpen && backdrop()} {/* Close on outer click */}
    </Stack>
  );
};
export default RadioDropdown;

/**  
 * Usage of radio dropdown
 * 
   <RadioDropdown
    title="Category"
    data={dataArray}
    handleChange={(value: string, index: number) => {
    console.log('index', index);
    console.log('value', value);
    }}
    selected="Value 1"
    />
 */

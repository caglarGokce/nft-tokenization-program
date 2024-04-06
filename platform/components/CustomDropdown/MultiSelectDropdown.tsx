import React, { useState } from 'react';
import { v4 as uuid } from 'uuid';
import Stack from '../Stack';
import Typography from '../Typography';
import Card from '../Card';
import Icon from '../Icon';
import { faChevronDown } from '@fortawesome/free-solid-svg-icons';

type P = {
  /** Data Array for dropdown */
  data: string[];
  /** Title of dropdown */
  title: string;
  /** Selected values for dropdown */
  selected: string[];
  /** Action performed on clicking a row
   * @param value selected value of dropdown
   * @param index Index of the selected value object
   */
  handleChange: (value: string, index: number) => void;
};

const MultiSelectDropdown: React.FC<P> = ({
  title,
  data,
  selected,
  handleChange,
}) => {
  const [isOpen, setIsOpen] = useState<boolean>(false);

  const toggleDropdown = () => {
    setIsOpen((old) => !old);
  };
  const handleSelect = (value: string, index: number) => {
    handleChange(value, index);
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
          className={`absolute overflow-auto flex flex-col border dark:border-surfaceDark-dark border-surface-dark top-10 right-0 z-[999999991] w-44 max-h-32 p-3 pb-2 ${dropdownVisibilityClass}`}
          onClick={(e) => e?.stopPropagation()}
        >
          {data?.map((item, index) => (
            <Stack
              isRow
              key={uuid()}
              className="space-x-2 pb-2 cursor-pointer"
              onClick={() => handleSelect(item, index)}
            >
              <input
                name={title}
                type="checkbox"
                value={item}
                className="accent-surfaceDark-dark"
                checked={selected.includes(item)}
                onChange={() => {}}
              />
              <Typography text={item} variant="button2" />
            </Stack>
          ))}
        </Card>
      </Stack>
      {isOpen && backdrop()} {/* Close on outer click */}
    </Stack>
  );
};

export default MultiSelectDropdown;

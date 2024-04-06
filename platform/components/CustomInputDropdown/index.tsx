import React, {
  KeyboardEvent,
  ReactElement,
  useEffect,
  useRef,
  useState,
} from 'react';
import { faChevronDown } from '@fortawesome/free-solid-svg-icons';
import Tooltip from '../Tooltip';
import Typography from '../Typography';
import Icon from '../Icon';

export type SuggestionData = {
  key: string;
  value: string;
};
type InputDropdownProps = {
  name: string;
  label?: string;
  value: string;
  placeholder?: string;
  disabled?: boolean;
  onChange: (value: any) => void;
  suggestionData: SuggestionData[];
  searchElement?: (value: SuggestionData) => void;
  icon?: ReactElement;
  iconAlign?: 'Left' | 'Right';
  type?: 'default' | 'search';
  validator?: (value: string) => string | false;
  tooltip?: string;
};

const OwniInputDropdown: React.FC<InputDropdownProps> = ({
  name,
  value,
  placeholder = '',
  label = null,
  onChange,
  disabled = false,
  suggestionData,
  searchElement,
  icon = null,
  iconAlign = null,
  type = 'default',
  validator,
  tooltip = '',
}) => {
  const [selectedIndex, setSelectedIndex] = useState<number | null>(null);
  const [isDropdownOpen, setIsDropdownOpen] = useState<boolean>(false);
  const containerRef = useRef<any>(null);
  const [selectedValue, setSelectedValue] = useState<string>(value);
  const [error, setError] = useState<string>('');

  const handleOptionClick = (option: SuggestionData) => {
    setSelectedValue(option.value);
    validateInput(option.key);

    if (type === 'default') {
      onChange(option);
    } else if (searchElement) {
      onChange(option.value);
      searchElement(option);
    }
    setIsDropdownOpen(false);
  };

  const handleKeyDown = (event: KeyboardEvent<HTMLInputElement>) => {
    const handleArrowDown = () => {
      setSelectedIndex((prev) => {
        if (prev === null) {
          return 0;
        } else if (suggestionData.length - 1 === prev) {
          return prev;
        } else {
          return prev + 1;
        }
      });
    };

    const handleArrowUp = () => {
      setSelectedIndex((prev) =>
        prev === null
          ? suggestionData.length - 1
          : (prev - 1 + suggestionData.length) % suggestionData.length,
      );
    };

    const handleEnter = () => {
      if (selectedIndex !== null) {
        event.preventDefault();
        handleOptionClick(suggestionData[selectedIndex]);
      }
    };

    switch (event.key) {
      case 'ArrowDown':
        event.preventDefault();
        handleArrowDown();
        break;
      case 'ArrowUp':
        event.preventDefault();
        handleArrowUp();
        break;
      case 'Enter':
        handleEnter();
        break;
      default:
        break;
    }
  };

  useEffect(() => {
    // Scroll to the selected index when it changes
    if (containerRef.current && selectedIndex && selectedIndex > 1) {
      const selectedOption = containerRef.current.children[selectedIndex];
      const containerHeight = containerRef.current.clientHeight;
      const scrollPosition = containerRef.current.scrollTop;
      const selectedOptionHeight = selectedOption.offsetHeight;

      if (
        selectedOption.offsetTop + selectedOptionHeight >
        scrollPosition + containerHeight
      ) {
        // Scroll only if the selected option is below the visible area
        containerRef.current.scrollTop =
          selectedOption.offsetTop +
          selectedOptionHeight -
          containerHeight -
          70;
      } else if (selectedOption.offsetTop < scrollPosition) {
        // Scroll up if the selected option is above the visible area
        containerRef.current.scrollTop = selectedOption.offsetTop;
      }
    }
  }, [selectedIndex]);

  const toggleDropdown = () => {
    setIsDropdownOpen(!isDropdownOpen);
  };

  const validateInput = (val: string) => {
    if (validator) {
      const errorMessage = validator(val);
      setError(errorMessage !== false ? errorMessage : '');
    }
  };

  return (
    <div>
      {label && <Typography text={label} variant="body1" weight="medium" />}
      <Tooltip text={tooltip} placement="bottom">
        <div
          className={`${
            error === '' ? '' : 'border-red-700'
          } flex mt-4 h-12 bg-white dark:bg-black border border-surface-dark dark:border-surfaceDark-dark px-2 justify-left items-center space-x-2 rounded-xs`}
        >
          {icon && iconAlign === 'Left' && (
            <span className="icon-left">{icon}</span>
          )}
          <input
            autoComplete="off"
            className="text-black dark:text-white dark:bg-black w-full px-5 h-10 outline-none"
            type={type === 'default' ? 'button' : 'text'}
            name={name}
            value={selectedValue}
            onKeyDown={handleKeyDown}
            placeholder={placeholder}
            onChange={(e) => {
              setSelectedIndex(null);
              setIsDropdownOpen(true);
              setSelectedValue(e.target.value);
              onChange(e.target.value);
              validateInput(e.target.value);
            }}
            onClick={() => {
              toggleDropdown();
            }}
          />
          <Icon
            color="black"
            icon={faChevronDown}
            size="md"
            className={`${
              suggestionData.length && isDropdownOpen ? 'rotate-180' : ''
            } duration-300 dark:hidden`}
          />
          <Icon
            color="white"
            icon={faChevronDown}
            size="md"
            className={`${
              suggestionData.length && isDropdownOpen ? 'rotate-180' : ''
            } duration-300 hidden dark:block`}
          />
        </div>
      </Tooltip>
      {!!suggestionData.length && isDropdownOpen ? (
        <select
          id="suggestions"
          tabIndex={0}
          ref={containerRef}
          size={Math.max(2, suggestionData.length)}
          className={`border border-surface-dark dark:border-surfaceDark-dark w-full rounded-xs py-4 bg-white dark:bg-black overflow-y-auto no-scrollbar ${
            suggestionData.length > 1 ? 'h-auto' : 'max-h-16'
          }`}
        >
          {suggestionData.map((suggestion: SuggestionData, index: number) => (
            <option
              tabIndex={index}
              aria-selected={selectedIndex === index}
              className={`h-full w-full cursor-pointer hover:bg-gray-200 pl-4 py-1 text-black dark:text-white dark:hover:text-black dark:focus-visible:text-black ${
                selectedIndex === index ? 'bg-gray-200 dark:!text-black' : ''
              }`}
              key={suggestion.value}
              value={suggestion.value}
              onClick={() => {
                handleOptionClick(suggestion);
              }}
            >
              {suggestion.value}
            </option>
          ))}
        </select>
      ) : null}
      {error && <span className="text-red-500 text-sm mt-1">{error}</span>}
    </div>
  );
};

export default OwniInputDropdown;

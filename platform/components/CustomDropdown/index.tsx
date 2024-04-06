import React, { useState } from 'react';
import Button from '../Button';
import { faChevronUp } from '@fortawesome/free-solid-svg-icons';
import Icon from '../Icon';

type DropdownProps = {
  options?: any[];
  logo?: any;
  customOption?: any;
  label?: string;
  disabled?: boolean;
  setCheckedValues?: (val: any) => void;
  align?: 'Right' | 'Left';
};

const OwniDropdown: React.FC<DropdownProps> = ({
  label = '',
  logo = null,
  disabled = false,
  options,
  customOption,
  setCheckedValues,
  align = 'Left',
}) => {
  const [isDropdownOpen, setIsDropdownOpen] = useState<boolean>(false);
  const toogleIsDropdownOpen = () => {
    setIsDropdownOpen(!isDropdownOpen);
  };

  const setSelectedValue = (event: any, option: any) => {
    if (setCheckedValues) {
      if (event.target.checked)
        setCheckedValues((prev: any) => [...prev, option]);
      else {
        setCheckedValues((prev: any) =>
          prev.filter((value: any) => value !== option),
        );
      }
    }
  };

  return (
    <div className="space-y-2">
      <div
        className={`flex ${align === 'Left' ? 'justify-start' : 'justify-end'}`}
      >
        <Button
          className=' rounded-xl'
          text={label}
          onClick={toogleIsDropdownOpen}
          disabled={disabled}
        />
      </div>
      {isDropdownOpen &&
        !disabled &&
        (!customOption ? (
          <div className="border bg-current justify-left px-5 items-center space-x-2 rounded-lg">
            {options?.length ? (
              <div className="w-full p-4 bg-current max-h-36 overflow-y-auto">
                {options.map((option: any) => (
                  <div key={option} className="flex text-black space-x-2 h-8">
                    <input
                      type="checkbox"
                      value={option}
                      onChange={(event) => setSelectedValue(event, option)}
                    />
                    <span className="self-center">{logo}</span>
                    <span className="self-center">{option}</span>
                  </div>
                ))}
              </div>
            ) : null}
          </div>
        ) : (
          <div>{customOption}</div>
        ))}
    </div>
  );
};

export default OwniDropdown;

import React, { useMemo } from 'react';

/** Props for the FormSwitch component */
export type FormSwitchPropsType = Omit<
  React.DetailedHTMLProps<
    React.InputHTMLAttributes<HTMLInputElement>,
    HTMLInputElement
  >,
  'type' | 'value' | 'checked' | 'onChange'
> & {
  /** If the toggle should be enabled */
  checked: boolean;
  /** Called toggle state is changed
   * @param checked If the new state is checked
   */
  onChange?: (checked: boolean) => void;
  /** Renders a larger version of the switch */
  isLarge?: boolean;
  /** Element to render left of switch */
  left?: React.ReactElement;
  /** Element to render right of switch */
  right?: React.ReactElement;
};

/** Renders a toggle switch component */
export default function FormSwitch({
  checked,
  onChange,
  isLarge,
  left,
  right,
  ...inputProps
}: Readonly<FormSwitchPropsType>) {
  /** Classes for switch based on input props */
  const classNames = useMemo(() => {
    const classes = [];
    if (isLarge) classes.push('w-14 h-8 after:h-6 after:w-6');
    else classes.push('w-12 h-6 after:h-4 after:w-4');
    if (checked) {
      classes.push(
        'after:translate-x-6 rtl:peer-checked:after:-translate-x-6 dark:bg-primaryDark bg-primary border-primary after:bg-primaryDark dark:after:bg-primaryDark-contrast dark:border-primaryDark',
      );
    } else {
      classes.push(
        'after:opacity-40 after:bg-primary dark:after:bg-primaryDark border border-surface-dark',
      );
    }
    classes.push(
      `border peer-focus:outline-none rounded-xxl peer after:content-[''] after:absolute after:top-[4px] after:start-[4px] after:rounded-xxl after:transition-all dark:border-surfaceDark-dark`,
    );
    return classes.join(' ');
  }, [isLarge, checked]);

  const inputClass = useMemo(() => {
    const cls = ['sr-only peer'];
    if (inputProps?.className) cls.push(inputProps.className);
    return cls.join(' ');
  }, [inputProps]);

  const handleInput = (e: React.KeyboardEvent<HTMLDivElement>) => {
    if (e.key === ' ') {
      e.preventDefault();
      onChange?.(!checked);
    }
  };

  return (
    <div
      className="flex flex-col justify-center select-none"
      tabIndex={0}
      onKeyDown={handleInput}
    >
      <label className="inline-flex gap-2 cursor-pointer items-center">
        <input
          {...inputProps}
          type="checkbox"
          value=""
          className={inputClass}
          checked={checked}
          onChange={(e) => onChange?.(e.target.checked)}
        />
        <span>{left}</span>
        <div className="relative inline-flex flex-col">
          <div className={classNames}></div>
        </div>
        <span>{right}</span>
      </label>
    </div>
  );
}

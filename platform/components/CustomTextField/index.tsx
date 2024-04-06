import Tooltip from '../Tooltip';
import React, { HTMLProps, ReactElement, useCallback, useState } from 'react';
import Typography from '../Typography';
import Card from '../Card';

export type OmiInputFieldPropsType = Omit<
  HTMLProps<HTMLInputElement>,
  'onChange'
> & {
  label?: string;
  onChange?: (value: any) => void;
  icon?: ReactElement;
  type?: 'text' | 'password' | 'email';
  iconAlign?: 'Left' | 'Right';
  value?: string;
  validator?: (value: string) => string | false;
  tooltip?: string;
  autocomplete?: string;
};

const OmiInputField: React.FC<OmiInputFieldPropsType> = ({
  className = '',
  label = null,
  onChange,
  type = 'text',
  icon = null,
  iconAlign = null,
  value = "",
  validator,
  tooltip = '',
  autocomplete = "off",
  ...props
}) => {
  const validateText = useCallback(
    (value: string) => {
      switch (type) {
        case 'email':
          return value.includes('@');
        case 'password':
          // Add password validation logic here if needed
          return value.length >= 8;
        default:
          // Default text validation if necessary
          return true;
      }
    },
    [type],
  );

  const [text, setText] = useState<string>(value);
  const [error, setError] = useState<string>('');

  const validateInput = (text: string) => {
    if (validator) {
      const errorMessage = validator(text);
      setError(errorMessage !== false ? errorMessage : '');
    } else {
      setError(validateText(text) ? '' : 'Invalid input');
    }
  };
  return (
    <div className={className}>
      {label && <Typography text={label} variant="body1" weight="medium" />}
      <Tooltip text={tooltip} placement="bottom" className="w-full">
        <Card
          elevation={0}
          className={`h-12 flex w-full bg-current border mt-4 ${
            error === '' ? '' : 'border-error dark:border-errorDark'
          } border-surface-dark dark:border-surfaceDark-dark justify-left px-5 items-center space-x-2 rounded-xs`}
        >
          {icon && iconAlign === 'Left' && (
            <span className="icon-left">{icon}</span>
          )}
          <input
            autoComplete={autocomplete}
            {...props}
            type={type}
            className="bg-surface-light dark:bg-surfaceDark-light dark:border-surfaceDark-dark text-typography dark:text-typographyDark w-full h-10 outline-none"
            value={text}
            onChange={(e) => {
              setText(e.target.value);
              onChange(e.target.value);
              validateInput(e.target.value);
            }}
          />
          {icon && iconAlign === 'Right' && (
            <span className="icon-right">{icon}</span>
          )}
        </Card>
      </Tooltip>
      {error && (
        <span className="text-error dark:text-errorDark text-sm mt-1">
          {error}
        </span>
      )}
    </div>
  );
};

export default OmiInputField;

import React from 'react';

type InputFieldProps = {
  type: string;
  name: string;
  error?: string;
  label?: string;
  value: any;
  placeholder?: string;
  onChange: (value: any) => void;
  disabled?: boolean;
  forgotPasswordLink?: string;
  handleRememberPassword?: (value: string) => void;
  max?: number;
  min?: number;
};

const OwniInputField: React.FC<InputFieldProps> = ({
  type,
  name,
  value,
  placeholder = '',
  error,
  label,
  onChange,
  disabled = false,
  handleRememberPassword,
  forgotPasswordLink,
  max,
  min,
}) => {
  const errorStyle = `
    ${error ? 'border-red-600' : ''}
  `;
  const disabledLinkStyle = `
    ${disabled ? 'pointer-events-none' : ''}
  `;

  return (
    <div className="space-y-4 w-full">
      {label ? <div className="font-bold">{label}</div> : null}
      <input
        max={max}
        min={min}
        className={`border ${errorStyle} w-full h-12 rounded-lg bg-surface dark:bg-surfaceDark text-typography dark:text-typographyDark p-3`}
        type={type}
        name={name}
        value={value}
        placeholder={placeholder}
        onChange={(e) => onChange(e.target.value)}
        disabled={disabled}
      />
      {type === 'password' && (
        <div className="flex">
          <div className="w-full space-x-2">
            <input
              disabled={disabled}
              type="checkbox"
              onChange={(e) => (handleRememberPassword ? e.target.value : null)}
            />
            <span>Remember me</span>
          </div>
          <a
            href={forgotPasswordLink}
            target="_blank"
            rel="noopener noreferrer"
            className={`${disabledLinkStyle} text-xs text-gray-600 text-end w-full`}
          >
            Forgot your password?
          </a>
        </div>
      )}
      {error ? (
        <div className="w-[100%] text-xs text-red-600 text-right">{error}</div>
      ) : null}
    </div>
  );
};

export default OwniInputField;

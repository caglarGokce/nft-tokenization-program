'use client';

import { ReactElement, useCallback, useEffect, useMemo, useState } from 'react';
import { FormFieldType, FormFieldValueTypes } from '@/types/Components/Form';
import Tooltip, { TooltipPropsType } from '../Tooltip';
import Switch from './Switch';
import Typography from '../Typography';
import { TextDropdown } from '../Dropdown';
import TextField from './TextField';

type FormFieldPropsType = FormFieldType & {
  /** Called when the field value changes
   * @param value
   * @param valid The validity of the value
   */
  onChange: (value: FormFieldValueTypes, valid: boolean) => void;
  /** Tracks how many times the field was autofilled */
  autofillUpdate?: number;
};

type TooltipWrapperPropsType = TooltipPropsType & {
  /** The field props */
  field: FormFieldPropsType;
  /** Children to render inside tooltip */
  children: ReactElement;
};

/** Validates given value for given field
 * @param value The value to validate
 * @param field The field to validate value for
 */
function validate<T = string>(value: T, field: FormFieldType) {
  return 'validator' in field && field.validator
    ? field.validator(value as any)
    : false;
}

/** Wrapper for components with tooltip parent */
const TooltipWrapper = ({
  field,
  children,
  ...props
}: Omit<TooltipWrapperPropsType, 'text'>) => {
  if (field.tooltip)
    return (
      <Tooltip {...props} text={field.tooltip}>
        {children}
      </Tooltip>
    );
  return <>{children}</>;
};

/** Component used to render fields inside a form */
export const FormField = (field: FormFieldPropsType) => {
  const [value, setValue] = useState(field.value);
  const [dirty, setDirty] = useState(false);
  const [tooltipOpen, setTooltipOpen] = useState(false);

  const openTooltip = useCallback(() => {
    setTooltipOpen(true);
  }, []);
  const closeTooltip = useCallback(() => {
    setTooltipOpen(false);
  }, []);

  /** Listeners for tooltip handling */
  const tooltipListeners = useMemo(
    () =>
      ({
        onMouseEnter: openTooltip,
        onMouseLeave: closeTooltip,
        onClick: closeTooltip,
      }) as const,
    [closeTooltip, openTooltip],
  );

  /** Sets field to dirty if autofilled */
  useEffect(() => {
    if (field.autofillUpdate) setDirty(true);
  }, [field.autofillUpdate]);

  /** Contains the validation error, false if field is valid */
  const error = useMemo(() => validate(value, field), [field, value]);

  const showError = useMemo(
    () => (dirty && !!error ? error || 'Invalid Input' : undefined),
    [dirty, error],
  );

  const Component = useMemo(() => {
    switch (field.type) {
      case 'switch':
        return (
          <Switch
            {...tooltipListeners}
            {...field.switchProps}
            id={field.id}
            checked={value as boolean}
            onChange={(checked) => {
              setValue(checked);
              field.onChange(checked, true);
            }}
            right={field.label ? <Typography text={field.label} /> : undefined}
          />
        );
      case 'select':
        return (
          <TextDropdown
            {...field}
            {...field.dropdownProps}
            onChange={(v) => {
              if ('value' in v) {
                setValue(v.value);
                field.onChange(v.value, !validate(v.value, field));
              }
            }}
            error={showError}
            onOpen={closeTooltip}
          />
        );
      case 'date':
        return <Typography text="Date input not supported" />;
      default:
        return (
          <TextField
            {...field.fieldProps}
            inputProps={{
              ...field.fieldProps?.inputProps,
              onBlur: (e) => {
                setDirty(true);
                field.fieldProps?.inputProps?.onBlur?.(e);
              },
              id: field.id,
              type: field.type,
              disabled: field.disabled,
            }}
            label={field.label}
            error={showError}
            value={value as string}
            onTextChange={(v) => {
              setValue(v);
              field.onChange(v, !validate(v, field));
            }}
          />
        );
    }
  }, [field, tooltipListeners, closeTooltip, setValue, value, showError]);

  return (
    <TooltipWrapper field={field} open={tooltipOpen}>
      {Component}
    </TooltipWrapper>
  );
};

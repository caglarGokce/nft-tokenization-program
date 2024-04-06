import { ReactElement } from 'react';
import { ButtonPropsType } from '@/components/Button';
import { TypographyPropsType } from '@/components/Typography';
import { DropdownOptionType } from '../Dropdown';
import { DropdownPropsType } from '@/components/Dropdown';
import { FormSwitchPropsType } from '@/components/Form/Switch';
import { TextFieldPropsType } from '@/components/Form/TextField';

/** Possible values for a form field. */
export type FormFieldValueTypes = string | Date | boolean;

export type FormTooltipComponentType = {
  /** Tooltip shown on hover */
  tooltip?: string;
};

/** Type for form validator functions
 * @template V The type of the field value
 * @param value Value of the field
 * @returns error as a string, false if field is valid
 */
export type FormFieldValidatorType<V = string> = (value: V) => string | false;

/** Base type for FormField component types. */
type FormFieldBaseType<V = string> = FormTooltipComponentType & {
  /** ID of the field. Used to identify the field. Should be unique. */
  id: string;
  /** Label of the field. Shown to the user with the field. Indicates what the field is for. */
  label?: string;
  /** Value of the field. */
  value: V;
  /** If the field should be disabled. */
  disabled?: boolean;
  /** Type of the field. */
  type?: 'text' | 'password' | 'switch' | 'select' | 'date';
};

/** Type for fields that need validation. */
type FormFieldValidatableType<V = string> = FormFieldBaseType<V> & {
  /** Validator method for the field
   * @template V The type of the field value
   * @param value Value of the field
   * @returns error as a string, or false if field is valid
   */
  validator?: FormFieldValidatorType<V>;
};

/** Text field type. */
export type FormTextFieldType = FormFieldValidatableType<string> & {
  /** Type of the field. Must be text or password. */
  type: 'text' | 'password';
  /** Props for the text field component */
  fieldProps?: Omit<TextFieldPropsType, keyof FormFieldValidatableType<string>>;
};

/** Date field type. */
export type FormDateFieldType = FormFieldValidatableType<Date> & {
  /** Type of the field. Must be date. */
  type: 'date';
  /** Minimum date */
  min?: Date;
  /** Maximum date */
  max?: Date;
};

/** Switch field type. */
export type FormSwitchFieldType = FormFieldBaseType<boolean> & {
  /** Type of the field. Must be switch. */
  type: 'switch';
  /** Props for the switch component */
  switchProps?: Omit<FormSwitchPropsType, keyof FormFieldBaseType<boolean>>;
};

/** Select (dropdown) field type. */
export type FormSelectFieldType<T extends string | number = string> =
  FormFieldValidatableType<T> & {
    /** Options for the dropdown. */
    options: DropdownOptionType<T>[];
    /** Type of the field. Must be select. */
    type: 'select';
    /** Props for the Dropdown component */
    dropdownProps?: Omit<
      DropdownPropsType<T>,
      keyof FormFieldValidatableType<T> | 'options'
    >;
  };

/** Union of form field types. */
export type FormFieldType =
  | FormTextFieldType
  | FormDateFieldType
  | FormSwitchFieldType
  | FormSelectFieldType;

export type FormButtonType = FormTooltipComponentType & {
  /** If the button is the primary action of the form */
  isSubmit?: boolean;
  /** Text of the button */
  label: string;
  /** Props for the button */
  props?: Omit<ButtonPropsType, 'text'>;
};

/** Type for form link elements. */
export type FormLinkType = {
  /** URL the link should navigate to. */
  link: string;
  /** Text of the link */
  label: string;
  /** Props for the link typography */
  props?: Omit<TypographyPropsType, 'text'>;
};

/** Type for rendering a custom form element. */
export type FormCustomElementType = () => ReactElement;

/** Union type for form elements. */
export type FormElementType =
  | FormFieldType
  | FormButtonType
  | FormLinkType
  | FormCustomElementType;

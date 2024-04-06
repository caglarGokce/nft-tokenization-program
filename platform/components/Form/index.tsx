'use client';

import React, { useContext, useEffect, useMemo, useRef, useState } from 'react';
import {
  FormButtonType,
  FormElementType,
  FormFieldType,
  FormFieldValueTypes,
} from '@/types/Components/Form';
import { FormButton } from './Button';
import { FormField } from './Field';
import { FormLink } from './Link';
import Stack from '../Stack';
import Card from '../Card';
import Typography from '../Typography';
import Divider from '../Divider';
import { AppThemeTypographyVariantsType } from '@/theme';

type ValidatorMethodType = (v: FormFieldValueTypes) => boolean;

/** Contains field value and validation state */
class FormValue {
  /** The field value */
  value: FormFieldValueTypes;
  /** Validator method used to validate the field */
  validator: ValidatorMethodType;

  constructor(value: FormFieldValueTypes, validator: ValidatorMethodType) {
    this.value = value;
    this.validator = validator;
  }

  /** True if the field is valid, otherwise false */
  get valid() {
    return this.validator(this.value);
  }
}

type SubType = Record<string, FormFieldValueTypes>;

type P = {
  /** Object if the form should be contained in a card, otherwise false */
  card?: {
    /** The title of the card */
    title: string;
    /** Variant of the title */
    titleVariant?: AppThemeTypographyVariantsType;
  };
  /** Elements of the form */
  elements: FormElementType[];
  /** Called when the primary action of the form is triggered
   * @param data The data of the form
   */
  onSubmit: (data: SubType) => Promise<void> | void;
};

/** Generates processed data for all the form fields */
const generateFormData = (elements: FormElementType[]) => {
  // Create fields from elements with id present
  const fields = elements.filter((e) => 'id' in e) as FormFieldType[];
  const obj: Record<string, FormValue> = {};
  // Create FormValue instances from all the fields
  fields.forEach((f) => {
    const value = f.value;
    const validator =
      'validator' in f
        ? (v: FormFieldValueTypes) => !f.validator!(v as any)
        : () => true;
    obj[f.id] = new FormValue(value, validator);
  });
  /** Duplicate values would replace `obj` children. This check makes sure
   * no duplicate ids are present */
  if (Object.keys(obj).length !== fields.length) {
    throw Error('Form: Duplicate ID found in form fields!');
  }
  if (fields.length === 0) {
    throw Error('Form: No form fields given');
  }
  return obj;
};

/** Body of the form component */
const Body = ({ elements, onSubmit }: P) => {
  const [data, setData] = useState(generateFormData(elements));
  const [loading, setLoading] = useState(false);
  const formRef = useRef<HTMLDivElement>(null);
  const [autofillUpdate, setAutofillUpdate] = useState(0);

  useEffect(() => {
    // Get the form node from the ref
    const formNode = formRef.current;

    if (!formNode) return;

    // Handles browser autofill
    const autocompleteHandler = (e: Event) => {
      const target = e.target as HTMLInputElement; // Cast EventTarget to HTMLElement
      const isAutofilled = target.hasAttribute('autocompleted');
      let hasError = false;
      if (isAutofilled) {
        setData((data) => {
          data[target.id].value = target.value;
          hasError = hasError || !data[target.id].valid;
          return { ...data };
        });
      }
      if (hasError) setAutofillUpdate((a) => a + 1);
    };

    // Add the event listener
    formNode.addEventListener('onautocomplete', autocompleteHandler);

    // Clean up function to remove the event listener
    return () => {
      formNode.removeEventListener('onautocomplete', autocompleteHandler);
    };
  }, [formRef]);

  /** True if the form is valid */
  const valid = useMemo(
    () => Object.keys(data).every((k) => data[k].valid),
    [data],
  );

  /** Handles primary form action */
  const handleSubmit = async (element?: FormButtonType) => {
    // If element is not provided, find the primary action in form
    element ??= elements.find((e) => 'isSubmit' in e) as FormButtonType;
    if (!element || element.props?.disabled || !valid) {
      return;
    }
    element.props?.onClick?.();
    setLoading(true);
    // Attempt form submission
    try {
      const submission: SubType = {};
      Object.keys(data).forEach((k) => (submission[k] = data[k].value));
      await onSubmit(submission);
    } catch (e) {
      console.error(e)
    } finally {
      setLoading(false);
    }
  };

  return (
    <Stack
      spacing={2}
      onKeyUp={(e) => {
        if (e.key === 'Enter') handleSubmit();
      }}
      ref={formRef}
    >
      {elements.map((Element, index) => {
        const key = 'id' in Element ? Element.id : 'FE: ' + index;
        let Rendered;
        if (typeof Element === 'function') Rendered = <Element />;
        else if ('id' in Element) {
          Rendered = (
            <FormField
              {...Element}
              onChange={(value) => {
                setData((data) => {
                  data[Element.id].value = value;
                  return { ...data };
                });
              }}
              autofillUpdate={autofillUpdate}
            />
          );
        } else if ('link' in Element) {
          Rendered = <FormLink {...Element} />;
        } else {
          Rendered = (
            <FormButton
              {...Element}
              props={{
                disabled:
                  Element.props?.disabled || (Element.isSubmit && !valid),
                loading,
                onClick: Element.isSubmit
                  ? () => handleSubmit(Element)
                  : Element.props?.onClick,
              }}
            />
          );
        }
        return <React.Fragment key={key}>{Rendered}</React.Fragment>;
      })}
    </Stack>
  );
};

/** Form component used to render given elements inside a form as fields */
export const Form = ({ card, elements, onSubmit }: P) => {
  return card ? (
    <Card style={{ minWidth: 480 }}>
      <Typography variant={card.titleVariant ?? 'header3'} text={card.title} />
      <Divider margin={{ t: 2, b: 4 }} />
      <Body {...{ elements, onSubmit }} />
    </Card>
  ) : (
    <Body {...{ elements, onSubmit }} />
  );
};

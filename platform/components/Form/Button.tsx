'use client';

import React from 'react';
import { FormButtonType } from '@/types/Components/Form';
import Button from '../Button';
import Tooltip from '../Tooltip';

type FormButtonPropsType = FormButtonType;

/** Button component to be used in forms */
export const FormButton = ({
  tooltip,
  isSubmit,
  label,
  props,
}: FormButtonPropsType) => {
  const Btn = (
    <Button
      {...props}
      text={label}
      variant={props?.variant || (isSubmit ? 'contained' : undefined)}
    />
  );
  return tooltip ? <Tooltip text={tooltip}>{Btn}</Tooltip> : Btn;
};

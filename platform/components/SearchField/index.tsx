import React from 'react';
import TextField, { TextFieldPropsType } from '../Form/TextField';

export default function SearchField(
  props: Readonly<Omit<TextFieldPropsType, 'icon'>>,
) {
  return (
    <TextField
      rounded
      {...props}
      inputProps={{
        ...props.inputProps,
        placeholder: props.inputProps?.placeholder ?? 'Search',
        type: 'search',
      }}
    />
  );
}

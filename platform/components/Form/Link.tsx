'use client';

import Link from 'next/link';
import { FormLinkType } from '@/types/Components/Form';
import Typography from '../Typography';

type FormLinkPropsType = FormLinkType;

/** Link component for form fields */
export const FormLink = ({ link, label, ...textProps }: FormLinkPropsType) => {
  return (
    <Link
      href={link}
      className="text-typography-muted dark:text-typographyDark-muted"
    >
      <Typography
        variant="body2"
        disableDefaultColor
        text={label}
        {...textProps}
      />
    </Link>
  );
};

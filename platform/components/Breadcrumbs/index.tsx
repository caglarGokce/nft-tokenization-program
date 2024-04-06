import React, { useMemo } from 'react';
import Typography from '../Typography';
import { v4 as uuid } from 'uuid';
import Link from 'next/link';
import Stack from '../Stack';

type BreadcrumbsProps = {
  data: Record<string, string>;
};

const Breadcrumbs: React.FC<BreadcrumbsProps> = ({ data }) => {
  const crumbs = useMemo(() => Object?.keys(data), [data]);
  return (
    <Stack isRow>
      {crumbs?.map((item, index) => {
        return crumbs?.length === index + 1 ? (
          <Typography
            text={`${item}`}
            variant="button2"
            weight="semibold"
            key={uuid()}
          />
        ) : (
          <Link
            href={data[item]}
            className="cursor-pointer mr-0.5"
            key={uuid()}
          >
            <Typography subtle text={`${item} / `} variant="button2" />
          </Link>
        );
      })}
    </Stack>
  );
};
export default Breadcrumbs;

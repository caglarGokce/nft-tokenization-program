import React, { useState } from 'react';
import Stack from '../Stack';
import Typography from '../Typography';
import Divider from '../Divider';
import Card from '../Card';
import { faChevronDown } from '@fortawesome/free-solid-svg-icons';
import Icon from '@/components/Icon';

type P = {
  title: string;
  summary: string;
};
const HeaderSummaryCard: React.FC<P> = ({ title, summary }) => {
  const [open, setOpen] = useState(false);
  const toggleCard = () => setOpen(!open);

  return (
    <Stack>
      <Typography text={title} variant="header1" />
      <Divider margin={{ t: 1.5, b: 4 }} />
      <CardItem open={open} toggleCard={toggleCard} summary={summary} />
    </Stack>
  );
};
export default HeaderSummaryCard;

type CardItemProps = {
  open: boolean;
  toggleCard: () => void;
  summary: string;
};

const CardItem: React.FC<CardItemProps> = ({ summary, open, toggleCard }) => (
  <Card elevation={1}>
    <button
      className="flex items-center justify-between w-full"
      onClick={toggleCard}
    >
      <Stack isRow className="items-center justify-between w-full">
        <Typography text={'Summary'} variant="header3" />
        <Icon
          icon={faChevronDown}
          className={`dark:text-surface-light text-surfaceDark-light ${
            open ? 'rotate-180' : 'rotate-0'
          } duration-300`}
          size={'xl'}
        />
      </Stack>
    </button>
    <Stack className={`${open ? 'flex' : 'hidden'} flex-col`}>
      <Divider margin={{ t: 4, b: 5 }} />
      <Typography text={summary} variant="body1" />
    </Stack>
  </Card>
);

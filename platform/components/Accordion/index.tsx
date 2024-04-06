import React, { ReactNode, useState } from 'react';
import { v4 as uuid } from 'uuid';
import Divider from '../Divider';
import { faChevronDown, faChevronUp } from '@fortawesome/free-solid-svg-icons';
import Icon from '../Icon';

type P = {
  data: ItemProps[];
};

const Accordion: React.FC<P> = ({ data }) => {
  const [active, setActive] = useState<number | undefined>(undefined);
  const toggleAccordion = (index: number) => {
    active === index ? setActive(undefined) : setActive(index);
  };
  return (
    <div>
      {data?.map((item, index) => (
        <AccordionItem
          key={uuid()}
          item={item}
          active={active}
          index={index}
          toggleAccordion={toggleAccordion}
        />
      ))}
    </div>
  );
};
export default Accordion;

type ItemProps = {
  content: (active: boolean) => ReactNode;
  header: (active: boolean) => ReactNode;
};
type AccordionItemProps = {
  item: ItemProps;
  active: number | undefined;
  index: number;
  toggleAccordion: (index: number) => void;
};

const AccordionItem: React.FC<AccordionItemProps> = ({
  item,
  active,
  index,
  toggleAccordion,
}) => {
  const isActive = active === index;

  return (
    <div>
      <div>
        <div className="pt-1 flex items-center">
          <button className="my-2 mx-4" onClick={() => toggleAccordion(index)}>
            <Icon icon={isActive ? faChevronUp : faChevronDown} size="lg" />
          </button>
          {item?.header(isActive)}
        </div>
        <Divider margin={{ t: 2 }} />
      </div>
      {isActive && <div className="mt-4">{item?.content(isActive)}</div>}
    </div>
  );
};

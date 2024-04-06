import React, { useMemo } from 'react';
import TabsBase, { TabItemPropsType, TabsBasePropsType } from './TabsBase';
import Typography from '../Typography';

/** Renders a horizontal row of selectable tabs */
export default function Tabs(props: Readonly<Omit<TabsBasePropsType, 'Tab'>>) {
  return <TabsBase {...props} Tab={Tab} />;
}

/** Renders a single tab item for Tabs list */
function Tab({ item, active, ...props }: TabItemPropsType<HTMLButtonElement>) {
  const classNames = useMemo(() => {
    const classes = [
      'flex flex-grow whitespace-nowrap md:min-w-[238px] h-10 p-2 items-center justify-center',
    ];
    if (active)
      classes.push(
        'border-b-2 border-primary dark:border-primaryDark text-primary dark:text-primaryDark',
      );
    else {
      classes.push(
        'border-b border-surface-dark text-typographyDark-muted dark:border-surfaceDark-dark dark:text-surface-dark',
      );
      if (item.disabled) classes.push('opacity-50');
      else
        classes.push(
          'hover:border-typographyDark-muted dark:hover:border-surface-dark',
        );
    }
    return classes.join(' ');
  }, [active, item]);

  return (
    <button {...props} type="button" className={classNames}>
      <Typography variant="body1" disableDefaultColor text={item.label} />
    </button>
  );
}

import React, { HTMLProps, useMemo } from 'react';

/** Elevation type */
export type ElevationType = 0 | 1 | 2 | 3;

/** Props type for surface component */
export type SurfacePropsType = HTMLProps<HTMLDivElement> & {
  /** Level of surface. Determines deepness of background color. */
  elevation?: ElevationType;
};

const classes = [
  'bg-surface-light dark:bg-surfaceDark-light',
  'bg-surface dark:bg-surfaceDark',
  'bg-surface-dark dark:bg-surfaceDark-dark',
  'bg-surface-muted dark:bg-surfaceDark-muted',
];

export default function Surface({ elevation, ...props }: SurfacePropsType) {
  const className = useMemo(() => {
    const base = elevation ? classes[elevation] : classes[0];
    const custom = props.className;
    return custom ? [base, custom].join(' ') : base;
  }, [elevation, props.className]);
  return <div {...props} className={className} />;
}

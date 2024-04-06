import { getServerSafeDocument } from '@/helpers/ssr';
import React, { HTMLProps, useId, useMemo } from 'react';
import { createPortal } from 'react-dom';
import { Tooltip as ReactTooltip } from 'react-tooltip';

export type TooltipPlacementType =
  | 'top'
  | 'top-start'
  | 'top-end'
  | 'right'
  | 'right-start'
  | 'right-end'
  | 'bottom'
  | 'bottom-start'
  | 'bottom-end'
  | 'left'
  | 'left-start'
  | 'left-end';

/** Type for Tooltip component props */
export type TooltipPropsType<T = HTMLElement> = HTMLProps<T> & {
  /** Text to show in tooltip */
  text: string;
  /** Position of the tooltip */
  placement?: TooltipPlacementType;
  /** Props for the element container. Tooltip
   * children are wrapped in a div. These props
   * are passed to that container.
   */
  containerProps?: HTMLProps<HTMLDivElement>;
};

/** Displays a tooltip for children */
export default function Tooltip<T = HTMLElement>({
  text,
  children,
  placement,
  containerProps,
}: TooltipPropsType<T>) {
  const id = useId();
  const document = getServerSafeDocument();

  const Component = useMemo(() => {
    if (document) {
      return createPortal(
        <ReactTooltip
          id={id}
          place={placement}
          content={text}
          className="!bg-black !text-white !rounded-sm z-50 max-w-lg"
        />,
        document.body,
      );
    }
    return <></>;
  }, [id, placement, text, document]);

  return (
    <>
      <div
        {...containerProps}
        data-tooltip-id={id}
        data-tooltip-placement={placement}
      >
        {children}
      </div>
      {Component}
    </>
  );
}

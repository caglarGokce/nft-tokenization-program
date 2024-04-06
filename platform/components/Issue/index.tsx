import React, { useMemo } from 'react';
import { useConfigSelector } from '@/hooks/store';
import Typography from '../Typography';
import Icon from '../Icon';
import {
  faArrowUp,
  faInfoCircle,
} from '@fortawesome/free-solid-svg-icons';
import Stack from '../Stack';
import Tooltip from '../Tooltip';

export type IssueColorProps =
  | 'critical'
  | 'high'
  | 'medium'
  | 'low'
  | 'info'
  | 'other';

type P = {
  /** Color variant for issue component */
  color?: IssueColorProps;
  /** Card Type variant for issue component */
  variant?: 'primary' | 'secondary';
  /** Count of issues */
  count: string | number;
  /** Status of issues */
  status: string;
  /** Trigger function when click on discover more*/
  onClick?: () => void;
  /** Hint for issues */
  hint?: string;
};

/** Renders the Issue component with given Props */
const Issue: React.FC<P> = ({
  color = 'medium',
  variant = 'primary',
  count,
  status,
  onClick,
  hint = '',
}) => {
  const { dark } = useConfigSelector();

  /** Background color for issue component */
  const bg = useMemo(() => {
    return {
      critical: dark ? 'bg-errorDark-muted' : 'bg-error-muted',
      high: dark ? 'bg-orangeDark-muted' : 'bg-orange-muted',
      medium: dark ? 'bg-warningDark-muted' : 'bg-warning-muted',
      low: dark ? 'bg-successDark-muted' : 'bg-success-muted',
      info: dark ? 'bg-purpleDark-muted' : 'bg-purple-muted',
      other: 'bg-info-muted dark:bg-infoDark-muted',
    };
  }, [dark]);

  /** Font color for issue component (need to update once theme added) */
  const fontColor = useMemo(() => {
    return {
      critical: dark ? 'text-errorDark' : 'text-error',
      high: dark ? 'text-orangeDark' : 'text-orange',
      medium: dark ? 'text-warningDark' : 'text-warning',
      low: dark ? 'text-successDark' : 'text-success',
      info: dark ? 'text-purpleDark' : 'text-purple',
      other: 'text-info dark:text-infoDark',
    };
  }, [dark]);

  /** Renders if variant is primary */
  const primaryComponent = () => {
    return (
      <div className="flex justify-between items-center w-full ">
        <Typography text={count} variant="header2" />
        <div className="flex justify-center items-center">
          <div className={`text-base font-semibold mr-1.5 ${fontColor[color]}`}>
            {status}
          </div>
          {hint && (
            <Tooltip text={hint}>
              <div className="text-typography dark:text-typographyDark">
                <Icon
                  icon={faInfoCircle}
                  size="sm"
                  className={fontColor[color]}
                />
              </div>
            </Tooltip>
          )}
        </div>
      </div>
    );
  };

  /** Renders if variant is secondary */
  const secondaryComponent = () => {
    return (
      <div className="flex justify-between items-end w-full">
        <div className="flex flex-row md:flex-col justify-center items-start space-x-6 md:space-x-0">
          <Typography text={count} variant="header2" />
          <div
            className={`text-base font-semibold mt-0 md:mt-1.5 ${fontColor[color]}`}
          >
            {status}
          </div>
        </div>
        <button
          className="justify-center items-center cursor-pointer hidden" /** replace hidden with flex to show button */
          onClick={onClick}
        >
          <Typography text={'Discover more'} variant="body2" />
          <Icon
            size="lg"
            icon={faArrowUp}
            className={`w-5 h-5 ml-1.5 dark:text-typographyDark text-typography`}
          />
        </button>
      </div>
    );
  };

  /** Render Issue by variant type */
  return (
    <Stack isRow className={`p-4 rounded-md ${bg[color]} w-full h-[75px]`}>
      {variant === 'primary' ? primaryComponent() : secondaryComponent()}
    </Stack>
  );
};
export default Issue;

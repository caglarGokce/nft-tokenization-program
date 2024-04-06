import { IconDefinition } from '@fortawesome/pro-regular-svg-icons';
import { IconItemModifierPropType } from '@/types/Components/IconItem';

/** Base type for ParamRow types */
type BaseRowParamType<T> = { name: string; key: keyof T };

/** ParamRow type */
export type ParamRowParamType<T = any> = BaseRowParamType<T> &
  IconItemModifierPropType & {
    /** Icon (or text as icon) to show */
    icon: IconDefinition | undefined;
  };

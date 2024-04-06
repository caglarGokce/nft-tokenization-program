import { CamelToSnakeCase, SnakeToCamelCase } from '../../types/Utility';

type Dict<T> = Record<keyof T, any>;
type Type = 'snakeToCamel' | 'camelToSnake';

/** Converts given snake key to camel case
 * @param _snake The snake case string
 */
const snakeToCamel = <T extends string | number | symbol>(
  _snake: T,
): SnakeToCamelCase<T> => {
  const snake = _snake.toString();
  let camel = '';
  let i = 1;

  while (i < snake.length) {
    if (snake[i] === '_') {
      camel += snake[++i].toUpperCase();
    } else {
      camel += snake[i];
    }

    i++;
  }

  const first = snake.length > 0 ? snake[0] : '';
  return (first + camel) as SnakeToCamelCase<T>;
};

/** Converts given camel case string to snake case
 * @param _camel The camel case string
 */
const camelToSnake = <T extends string | number | symbol>(
  _camel: T,
): CamelToSnakeCase<T> => {
  const camel = _camel.toString();
  let snake = '';
  for (let i = 1; i < camel.length; i++) {
    const isLower = camel[i].toLowerCase() === camel[i];
    if (!isLower) snake += '_';
    snake += camel[i];
  }
  const first = camel.length > 0 ? camel[0] : '';
  return (first + snake.toLowerCase()) as CamelToSnakeCase<T>;
};

/** Handles recursive object case conversion
 * @param type Type of transformation
 */
class CaseTransformer {
  method: typeof snakeToCamel | typeof camelToSnake;

  constructor(type: Type = 'snakeToCamel') {
    this.method = type === 'snakeToCamel' ? snakeToCamel : camelToSnake;
  }

  /** Recursively goes through given array of tokens and
   * converts them using given transformation
   * @param el The list of tokens
   * @returns The list of transformed tokens
   */
  handleArr: (el: any[]) => any = (el) => {
    return el.map((i: any) => {
      if (Array.isArray(i)) return this.handleArr(i);
      if (typeof i === 'object') return this.transform(i);
      return i;
    });
  };

  /** Transforms keys of given dictionary object using given
   * transformation type
   * @param obj The dictionary object
   * @returns The given obj with keys converted to camel case
   */
  transform = <T = any>(obj: Dict<T>) => {
    if (!obj) return obj;
    const keys = Object.keys(obj) as (keyof T)[];
    const newObj: Partial<Dict<T>> = {};
    keys.forEach((key) => {
      let v = obj[key];
      const conv = this.method(key as string) as keyof T;
      if (typeof obj[key] === 'object') {
        if (Array.isArray(obj[key])) v = this.handleArr(obj[key]);
        else v = this.transform(obj[key]);
      }
      newObj[conv] = v;
    });
    return newObj;
  };
}

/** Converts keys of given dictionary object from snake
 * case to camel case
 * @param obj The dictionary object
 * @returns The given obj with keys converted to camel case
 */
const snakeToCamelKeys = <T = any>(obj: Dict<T>) => {
  const transformer = new CaseTransformer('snakeToCamel');
  return transformer.transform(obj) as Record<SnakeToCamelCase<keyof T>, any>;
};

/** Converts keys of given dictionary object from camel
 * case to snake case
 * @param obj The dictionary object
 * @returns The given obj with keys converted to snake case
 */
const camelToSnakeKeys = <T = any>(obj: Dict<T>) => {
  const transformer = new CaseTransformer('camelToSnake');
  return transformer.transform(obj) as Record<CamelToSnakeCase<keyof T>, any>;
};

export { snakeToCamel, camelToSnake, snakeToCamelKeys, camelToSnakeKeys };

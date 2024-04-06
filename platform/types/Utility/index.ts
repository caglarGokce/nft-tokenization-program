/** Converts `S` from a camelCase type to a snake_case type */
export type CamelToSnakeCase<S> = S extends `${infer T}${infer U}`
  ? `${T extends Capitalize<T> ? '_' : ''}${Lowercase<T>}${CamelToSnakeCase<U>}`
  : S;

/** Converts `S` from a snake_case type to a camelCase type */
export type SnakeToCamelCase<S> = S extends `${infer T}_${infer U}`
  ? `${T}${Capitalize<SnakeToCamelCase<U>>}`
  : S;

/** Generic key title type */
export type KeyTitleType<T> = {
  key: keyof T;
  title: string;
};

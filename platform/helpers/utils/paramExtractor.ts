/** Extracts nested `key` from `obj`
 * @param obj The object to get data from
 * @param key The key of data to extract
 * @returns Value of the given property
 */
export const extractNestedPropertyValue = (obj: any, key: string) => {
  const keys = key.split('.');
  return keys.reduce((acc, k) => {
    if (k.includes('[')) {
      const [arrayKey, index] = k.split(/[[\]]/).filter(Boolean);
      return acc ? acc[arrayKey][index] : undefined;
    }
    return acc ? acc[k] : undefined;
  }, obj);
};

/** Gets document object safely
 * @returns document when called from client side, null otherwise
 */
export function getServerSafeDocument() {
  return typeof document === 'undefined' ? null : document;
}

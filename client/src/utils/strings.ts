/**
 * Capitalizes the first letter of a given string and returns
 * the result.
 *
 * @param {string} string Input string to be capitalized
 * @returns {string} The input string with the first letter capitalized
 */
export function toCapitalized(string: string): string {
  if (string.charAt(0) === string.charAt(0).toUpperCase()) return string;

  return string.charAt(0).toUpperCase() + string.slice(1);
}

import { useCallback, useState } from 'react';

/**
 * Custom hook similar to useState but gets / stores the state in
 * the windows localstorage so it can be re-used.
 *
 * @param {string} key Identifier to use in localstorage
 * @param {T} initialValue Default value when not found in localstorage
 * @returns Returns a stateful value, and a function to update it.
 */
export function useLocalStorage<T>(
  key: string,
  initialValue: T
): [T, (value: T) => void] {
  const [storedValue, setStoredValue] = useState<T>(() => {
    if (typeof window === 'undefined') return initialValue;

    try {
      const item = window.localStorage.getItem(key);
      return item ? JSON.parse(item) : initialValue;
    } catch (error) {
      console.error(error);
      return initialValue;
    }
  });

  /**
   * Setter function to set the state and save the new value
   * to the windows localstorage.
   *
   * @param {T} value Value to be stored in localstorage
   */
  const setValue = useCallback(
    (value: T) => {
      try {
        const valueToStore =
          value instanceof Function ? value(storedValue) : value;
        setStoredValue(valueToStore);
        if (typeof window !== 'undefined') {
          window.localStorage.setItem(key, JSON.stringify(valueToStore));
        }
      } catch (error) {
        console.error(error);
      }
    },
    [key, storedValue]
  );

  return [storedValue, setValue];
}

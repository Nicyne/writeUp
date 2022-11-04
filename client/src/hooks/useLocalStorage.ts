import { useCallback, useState } from 'react';

export function useLocalStorage<T>(
  key: string,
  initValue: T
): [T, (value: T) => void] {
  const [storedValue, setStoredValue] = useState(() => {
    if (typeof window === 'undefined') {
      return initValue;
    }

    try {
      const item = window.localStorage.getItem(key);
      return item ? JSON.parse(item) : initValue;
    } catch (error) {
      console.error(error);
      return initValue;
    }
  });

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

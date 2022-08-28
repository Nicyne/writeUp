import { createContext, PropsWithChildren, useEffect } from 'react';

export interface IKeyContext {
  /**
   * Record holding the data of all keys
   * press/released.
   */
  keys: Record<string, boolean>;
  /**
   * Checks if a set of keys is pressed.
   * Returns a boolean.
   *
   * @param keys @type {string[]}
   * @returns @type {boolean}
   */
  areKeysDown: (keys: string[]) => {};
  /**
   * Checks if a single key is pressed.
   * Returns a boolean.
   *
   * @param key @type {string}
   * @returns @type {boolean}
   */
  isKeyDown: (key: string) => {};
}

export const KeyContext = createContext<IKeyContext>({
  keys: {},
  areKeysDown: (keys) => {
    return false;
  },
  isKeyDown: (key) => {
    return false;
  },
});

export const KeyContextProvider = (props: PropsWithChildren) => {
  let keyMap: Record<string, boolean> = {};

  const isKeyDown = (key: string) => {
    return keyMap[key.toLowerCase()];
  };

  const areKeysDown = (keys: string[]) => {
    return keys.every((key) => keyMap[key.toLowerCase()]);
  };

  useEffect(() => {
    const event = (e: KeyboardEvent) => {
      keyMap[e.key.toLocaleLowerCase()] = e.type === 'keydown';
    };

    document.addEventListener('keydown', event);
    document.addEventListener('keyup', event);

    return () => {
      document.removeEventListener('keydown', event);
      document.removeEventListener('keyup', event);
    };
  });

  return (
    <KeyContext.Provider value={{ keys: keyMap, areKeysDown, isKeyDown }}>
      {props.children}
    </KeyContext.Provider>
  );
};

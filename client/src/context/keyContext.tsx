import { NextPage } from 'next';
import { createContext, useEffect } from 'react';

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
    return true;
  },
  isKeyDown: (key) => {
    return true;
  },
});

export const KeyContextProvider: NextPage = ({ children }) => {
  let keyMap: Record<string, boolean> = {};

  const isKeyDown = (key: string) => {
    return keyMap[key.toLowerCase()];
  };

  const areKeysDown = (keys: string[]) => {
    return keys.every((key) => keyMap[key.toLowerCase()]);
  };

  useEffect(() => {
    onkeydown = onkeyup = (e) => {
      keyMap[e.key.toLowerCase()] = e.type == 'keydown';
    };
  });

  return (
    <KeyContext.Provider value={{ keys: keyMap, areKeysDown, isKeyDown }}>
      {children}
    </KeyContext.Provider>
  );
};

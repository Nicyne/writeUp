import { KeyContext } from 'context';
import { useContext } from 'react';

export function useKeys() {
  const { isKeyDown, areKeysDown } = useContext(KeyContext);
  return { isKeyDown, areKeysDown };
}

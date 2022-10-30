import { EffectCallback, useEffect } from 'react';

/**
 * This hook is used to run function only ONCE when the component
 * is mounted. If you need a dependency array please use react's
 * useEffect hook.
 *
 * @param effect
 * @returns
 */
export function useMountEffect(effect: EffectCallback) {
  // eslint-disable-next-line
  return useEffect(effect, []);
}

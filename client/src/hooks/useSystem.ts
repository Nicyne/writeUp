import { SystemContext } from 'context';
import { useContext } from 'react';

export function useSystem() {
  const { version } = useContext(SystemContext);
  return { version };
}

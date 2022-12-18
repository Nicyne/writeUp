import { AuthContext } from 'context';
import { useContext } from 'react';

export function useAuth() {
  return { ...useContext(AuthContext) };
}

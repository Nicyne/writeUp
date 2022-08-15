import { AuthContext } from 'context';
import { useContext } from 'react';

export function useAuth() {
  const { user, login, signUp, logout, getUser } = useContext(AuthContext);
  return { user, login, signUp, logout, getUser };
}

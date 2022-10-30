import { AuthContext } from 'context';
import { useContext } from 'react';

export function useAuth() {
  const { user, loading, login, signUp, logout, getUser } =
    useContext(AuthContext);
  return { user, loading, login, signUp, logout, getUser };
}

import { useEffect } from 'react';
import { useNavigate } from 'react-router-dom';
import { useAuth } from './useAuth';

/**
 * Redirects the user to the given path if authenticated
 */
export function useAuthenticatedRedirect(path: string) {
  const { user, loading } = useAuth();
  const navigate = useNavigate();

  useEffect(() => {
    if (user && !loading) {
      navigate(path);
    }
    // eslint-disable-next-line
  }, [user, loading, path]);
}

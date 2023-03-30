import { useEffect } from 'react';
import { useNavigate } from 'react-router-dom';
import { User } from 'types';

interface props {
  user?: User;
  isLoading: boolean;
  children?: React.ReactNode;
}

export function ProtectedRoute(props: props) {
  const navigate = useNavigate();

  useEffect(() => {
    if (props.isLoading) return;
    if (!props.user && !props.isLoading) {
      console.warn('not logged in');
      navigate('/login');
    }
  }, [navigate, props.user, props.isLoading]);

  if (props.isLoading) return <></>;

  return <>{props.children}</>;
}

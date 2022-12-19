import { useEffect, useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { useAuth } from 'hooks';

export function Logout() {
  const { user, logout } = useAuth();
  const [loggedOut, setLoggedOut] = useState(false);
  const [countDown, setCountDown] = useState(5);
  const navigate = useNavigate();

  useEffect(() => {
    let redirectTimer: NodeJS.Timeout;
    if (!user && !loggedOut) {
      console.error('no user');
      navigate('/');
      return;
    }

    logout();
    setLoggedOut(true);
    redirectTimer = setTimeout(() => {
      navigate('/');
    }, 5000);

    return () => clearTimeout(redirectTimer);
  }, [logout, navigate, user, loggedOut]);

  useEffect(() => {
    let countDownTimer: NodeJS.Timeout;

    countDownTimer = setTimeout(() => {
      setCountDown(countDown - 1);
    }, 1000);

    return () => clearTimeout(countDownTimer);
  });

  if (!loggedOut) return <></>;

  return (
    <>
      <h1>You were logged out!</h1>
      <h2>{countDown}</h2>
    </>
  );
}

export default Logout;

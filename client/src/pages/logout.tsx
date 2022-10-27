import { useEffect, useState } from 'react';
import { useTranslation } from 'react-i18next';
import { useNavigate } from 'react-router-dom';
import { useAuth, useMountEffect } from 'hooks';

export function Logout() {
  const { user, logout } = useAuth();
  const [t] = useTranslation();
  const [loggedOut, setLoggedOut] = useState(false);
  const [countDown, setCountDown] = useState(5);
  const navigate = useNavigate();

  const unit = countDown > 1 ? t('time.seconds') : t('time.second');

  useMountEffect(() => {
    let redirectTimer: NodeJS.Timeout;

    if (!user) {
      navigate('/');
      return;
    }

    logout();
    setLoggedOut(true);
    redirectTimer = setTimeout(() => {
      navigate('/');
    }, 5000);
    return () => clearTimeout(redirectTimer);
  });

  useEffect(() => {
    let countdownTimer: NodeJS.Timeout;

    countdownTimer = setTimeout(() => {
      setCountDown(countDown - 1);
    }, 1000);

    return () => clearTimeout(countdownTimer);
  });

  if (!loggedOut) return <></>;

  return (
    <div className="container">
      <div className="center">
        <h1>{t('auth.logout.messageSuccess')}</h1>
        <h2>
          {t('auth.logout.messageRedirectIn', {
            count: countDown,
            unit: unit,
          })}
        </h2>
      </div>
    </div>
  );
}

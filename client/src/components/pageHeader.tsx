import { useAuth, useMountEffect } from 'hooks';
import { useState } from 'react';
import { useTranslation } from 'react-i18next';
import { Link } from 'react-router-dom';
import styles from 'styles/components/pageHeader.module.scss';

export function PageHeader() {
  const { user, loading } = useAuth();
  const [version, setVersion] = useState('');
  const [t] = useTranslation();

  useMountEffect(() => {
    fetch('/api/system')
      .then((res) => res.json())
      .then((res) => {
        if (!res.success) {
          console.error(res.message);
          return;
        }

        if (!res.content.version) {
          console.error('response has no key called "version"');
          return;
        }

        setVersion('v.' + res.content.version);
      })
      .catch((err) => console.error(err));
  });

  return (
    <header className={styles['header']}>
      <nav className={styles['nav']}>
        <ul className={styles['list']}>
          <li>
            <Link to="/">
              writeUp <span className="small">{version}</span>
            </Link>
          </li>
          {user && (
            <li>
              <Link to="/app">App</Link>
            </li>
          )}
        </ul>
        {!loading && (
          <ul>
            {user ? (
              <>
                <li className={styles['user']}>{user.username}</li>
                <li>
                  <Link to="/logout" role="button">
                    {t('auth.logout.name')}
                  </Link>
                </li>
              </>
            ) : (
              <>
                <li>
                  <Link to="/login" className="secondary" role="button">
                    {t('auth.login.name')}
                  </Link>
                </li>
                <li>
                  <Link to="/signup" role="button">
                    {t('auth.signup.name')}
                  </Link>
                </li>
              </>
            )}
          </ul>
        )}
      </nav>
    </header>
  );
}

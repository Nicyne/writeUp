import { useAuth } from 'hooks/useAuth';
import { useTranslation } from 'react-i18next';
import { Link } from 'react-router-dom';
import styles from 'styles/components/pageHeader.module.scss';

export function PageHeader() {
  const { user } = useAuth();
  const [t] = useTranslation();

  return (
    <header className={styles['header']}>
      <nav className={styles['nav']}>
        <ul className={styles['list']}>
          <li>
            <Link to="/">writeUp</Link>
          </li>
          {user && (
            <li>
              <Link to="/app">App</Link>
            </li>
          )}
        </ul>
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
      </nav>
    </header>
  );
}

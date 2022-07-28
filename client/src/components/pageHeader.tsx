import styles from 'styles/components/pageHeader.module.scss';
import { FunctionComponent, SyntheticEvent, useContext } from 'react';
import { UserContext } from '../context/userContext';
import Link from 'next/link';
import { dApi } from 'lib';

export const PageHeader: FunctionComponent = () => {
  const { currentUser, mutate } = useContext(UserContext);

  const deleteUser = async (e: SyntheticEvent) => {
    await dApi.deleteUser();
    mutate();
  };

  return (
    <header className={styles.pageHeader}>
      <div className="container">
        <nav>
          <div className={styles.logo}>
            <Link href={'/'}>writeUp!</Link>
          </div>
          <ul className={styles.navigation}>
            <li>
              <Link href={'/'}>Home</Link>
            </li>
            <li>
              <Link href={'/app'}>App</Link>
            </li>
            {currentUser ? (
              <>
                <li>
                  <Link href={'/auth/logout'}>Logout</Link>
                </li>
                <li>
                  <button onClick={deleteUser}>Delete Account</button>
                </li>
              </>
            ) : (
              <>
                <li>
                  <Link href={'/auth/login'}>Login</Link>
                </li>
                <li>
                  <Link href={'/auth/register'}>Register</Link>
                </li>
              </>
            )}
          </ul>
        </nav>
      </div>
    </header>
  );
};

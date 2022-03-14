import { FunctionComponent, SyntheticEvent, useContext } from 'react';
import { UserContext } from '../providers/userContextProvider';
import Link from 'next/link';
import { dApi } from 'lib';

const PageHeader: FunctionComponent = () => {
  const { currentUser, mutate } = useContext(UserContext);

  const logout = async (e: SyntheticEvent) => {
    await dApi.logout();
    mutate('user');
  };

  return (
    <header className="pageHeader">
      <div className="container">
        <nav>
          <div className="logo">
            <Link href={'/'}>writeUp!</Link>
          </div>
          <ul className="navigation">
            <li>
              <Link href={'/'}>Home</Link>
            </li>
            <li>
              <Link href={'/app'}>App</Link>
            </li>
            {currentUser ? (
              <li>
                <button onClick={logout}>Logout</button>
              </li>
            ) : (
              <li>
                <Link href={'/login'}>Login</Link>
              </li>
            )}
          </ul>
        </nav>
      </div>
    </header>
  );
};

export default PageHeader;

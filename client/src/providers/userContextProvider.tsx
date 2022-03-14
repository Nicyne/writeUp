import { NextPage } from 'next';
import { createContext } from 'react';
import { useUser } from 'lib';
import { useRouter } from 'next/router';
import { IUser } from 'types';

const allowedRoutes = ['/login', '/'];

export interface IUserContext {
  currentUser?: IUser;
  loading?: boolean;
  mutate?: any;
}

export const UserContext = createContext<IUserContext>({});

export const UserContextProvider: NextPage = ({ children }) => {
  const router = useRouter();
  const [user, { loading, mutate }] = useUser();

  if (
    typeof window !== 'undefined' &&
    !user &&
    !loading &&
    !allowedRoutes.includes(router.pathname)
  ) {
    router.push('/login');
  }

  return (
    <UserContext.Provider value={{ currentUser: user, loading, mutate }}>
      {children}
    </UserContext.Provider>
  );
};

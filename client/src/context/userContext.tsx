import { NextPage } from 'next';
import { createContext } from 'react';
import { useUser } from 'hooks';
import { useRouter } from 'next/router';
import { IUser } from 'types';
import { protectedRoutes } from 'values';

export interface IUserContext {
  currentUser?: IUser;
  loading?: boolean;
  mutate?: any;
}

export const UserContext = createContext<IUserContext>({});

export const UserContextProvider: NextPage = ({ children }) => {
  const router = useRouter();
  const [user, { loading, mutate }] = useUser();
  const mutateUser = () => {
    mutate('user');
  };

  if (
    typeof window !== 'undefined' &&
    !user &&
    !loading &&
    protectedRoutes.includes(router.pathname)
  ) {
    /**
     * Replace is used here since it doesn't add
     * a history entry for the source url. So if
     * a user visits /app and gets redirected because
     * he's not logged in only /login will appear in the history
     * and not /app. Using router.push here causes
     * the user to get stuck and disabling
     * the ability to use the 'back' button to navigate
     * to the previous page.
     */
    router.replace('/auth/login');
  }

  return (
    <UserContext.Provider
      value={{ currentUser: user, loading, mutate: mutateUser }}
    >
      {children}
    </UserContext.Provider>
  );
};

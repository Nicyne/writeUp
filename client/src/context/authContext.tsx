import {
  createContext,
  PropsWithChildren,
  useCallback,
  useEffect,
  useState,
} from 'react';
import { User } from 'types';

/* TYPES */

type LoginFunction = (username: string, password: string) => Promise<IResponse>;

type SignUpFunction = (
  username: string,
  password: string,
  betaKey: string
) => Promise<IResponse>;

type LogoutFunction = () => Promise<void>;
type GetUserFunction = () => Promise<void>;

/* INTERFACES */

export interface IResponse {
  success: boolean;
  message: string;
}

export interface IAuthContext {
  user?: User;
  isLoading: boolean;
  login: LoginFunction;
  signUp: SignUpFunction;
  logout: LogoutFunction;
  getUser: GetUserFunction;
}

export const AuthContext = createContext<IAuthContext>({
  isLoading: false,
  login: async () => {
    return { success: false, message: '' };
  },
  signUp: async () => {
    return { success: false, message: '' };
  },
  logout: async () => {},
  getUser: async () => {},
});

export function AuthContextProvider(props: PropsWithChildren) {
  const [user, setUser] = useState<User>();
  const [isLoading, setIsLoading] = useState<boolean>(true);

  const logout = useCallback(async () => {
    if (!user) return;

    return await fetch('/api/auth', {
      method: 'DELETE',
      credentials: 'include',
    })
      .then((res) => res.json())
      .then((res) => {
        console.debug(res);
        if (!res.success) return;

        setUser(undefined);
      })
      .catch((err) => {
        console.error(err);
      });
  }, []);

  const login = useCallback(
    async (username: string, password: string): Promise<IResponse> => {
      return await fetch('/api/auth', {
        method: 'POST',
        credentials: 'include',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ username, password, session_only: false }),
      })
        .then((res) => res.json())
        .then(async (res) => {
          console.debug(res);

          if (!res.success) {
            return { success: false, message: res.message ?? 'Login failed' };
          }

          await getUser();
          return { success: true, message: res.message ?? 'Login successful!' };
        })
        .catch((err) => {
          console.error(err);
          return { success: false, message: err };
        });
    },
    []
  );

  const signUp = useCallback(
    async (username: string, password: string, betaKey: string) => {
      if (user)
        return {
          success: false,
          message: 'Can not signup, user is already authenticated',
        };

      const body = JSON.stringify({ username, password, beta_key: betaKey });

      return await fetch('/api/user', {
        method: 'POST',
        credentials: 'include',
        headers: { 'Content-Type': 'application/json' },
        body: body,
      })
        .then((res) => res.json())
        .then((res) => {
          console.debug(res);

          if (!res.success) {
            return {
              success: false,
              message: res.message ?? 'SignUp failed',
            };
          }

          return {
            success: true,
            message: res.message ?? 'SignUp successful!',
          };
        })
        .catch((err) => {
          console.error(err);
          return {
            success: false,
            message: err,
          };
        });
    },
    []
  );

  const getUser = useCallback(async () => {
    setIsLoading(true);
    return await fetch('/api/auth', {
      credentials: 'include',
    })
      .then((res) => res.json())
      .then((res) => {
        console.debug(res);

        if (!res.success) {
          return;
        }

        setUser(res.content);
      })
      .catch((err) => {
        console.error(err);
        return;
      })
      .finally(() => setIsLoading(false));
  }, []);

  useEffect(() => {
    getUser();
  }, []);

  return (
    <AuthContext.Provider
      value={{ user, isLoading, login, signUp, logout, getUser }}
    >
      {props.children}
    </AuthContext.Provider>
  );
}

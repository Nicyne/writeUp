import axios from 'axios';
import { createContext, PropsWithChildren, useEffect, useState } from 'react';
import { IUser } from 'types';

export interface IResponse {
  success: boolean;
  message: string;
}

export interface IAuthContext {
  user?: IUser;
  loading: boolean;
  login: (username: string, password: string) => Promise<IResponse>;
  signUp: (username: string, password: string) => Promise<IResponse>;
  logout: () => Promise<void>;
  getUser: () => Promise<void>;
}

export const AuthContext = createContext<IAuthContext>({
  loading: false,
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
  const [user, setUser] = useState<IUser>();
  const [loading, setLoading] = useState<boolean>(true);

  /**
   * logout function
   *
   * @returns
   */
  const logout = async () => {
    if (!user) return;

    const res = await axios.delete('/api/auth');

    if (!res.data.success) {
      if (res.status === 200) return;
      console.error(res);
      return;
    }

    setUser(undefined);
  };

  /**
   * Login function
   *
   * @param username
   * @param password
   */
  const login = async (
    username: string,
    password: string
  ): Promise<IResponse> => {
    if (username.length < 3) {
      return { success: false, message: 'provided invalid username' };
    }

    const res = await axios.post('/api/auth', { username, password });

    if (!res.data.success) {
      console.log(res);
      return { success: false, message: res.data.message ?? 'Login failed' };
    }

    await getUser();
    return { success: true, message: '' };
  };

  /**
   * Signup function
   *
   * @param username
   * @param password
   */
  const signUp = async (
    username: string,
    password: string
  ): Promise<IResponse> => {
    if (username.length < 3) {
      return { success: false, message: 'provided invalid username' };
    }
    if (password.length < 8) {
      return { success: false, message: 'provided invalid password' };
    }

    const res = await axios.post('/api/user', { username, password });

    if (!res.data.success) {
      console.log(res);
      return { success: false, message: res.data.message ?? 'SignUp failed' };
    }

    await getUser();
    return { success: true, message: '' };
  };

  /**
   * get User function
   */
  const getUser = async () => {
    setLoading(true);
    const res = await axios.get('/api/auth');

    if (!res.data.success) {
      setUser(undefined);
      setLoading(false);
      if (res.status === 200) return;
      console.error(res);
      return;
    }

    if (!res.data.content) {
      console.error('No user object present on api response data');
      setLoading(false);
      return;
    }
    setUser(res.data.content);
    setLoading(false);
  };

  useEffect(() => {
    getUser();
  }, []);

  return (
    <AuthContext.Provider
      value={{ user, loading, login, signUp, logout, getUser }}
    >
      {props.children}
    </AuthContext.Provider>
  );
}

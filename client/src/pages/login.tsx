import type { NextPage } from 'next';
import { useRouter } from 'next/router';
import Head from 'next/head';
import { SyntheticEvent, useContext, useEffect, useState } from 'react';
import { UserContext } from 'context/userContext';
import { dApi } from '../lib';

const Login: NextPage = () => {
  const router = useRouter();
  const { currentUser, loading, mutate } = useContext(UserContext);
  const [username, setUsername] = useState<string>('');
  const [password, setPassword] = useState<string>('');

  useEffect(() => {
    if (typeof window === 'undefined') return;
    if (currentUser && !loading) router.push('/app');
  }, [currentUser, loading, router]);

  const login = async (e: SyntheticEvent) => {
    e.preventDefault();

    try {
      await dApi.login(username, password);

      await mutate('user');
      await router.push('/app');
    } catch (err: any) {
      console.error(err);
    }
  };

  return (
    <div className="login">
      <Head>
        <title>Login | Writeup</title>
      </Head>

      <form onSubmit={login}>
        <h1>WriteUp</h1>
        <label htmlFor="username">
          Username
          <input
            id="username"
            name="username"
            type="text"
            onChange={({ target }) => setUsername(target.value)}
          />
        </label>
        <label htmlFor="password">
          Password
          <input
            id="password"
            name="password"
            type="password"
            onChange={({ target }) => setPassword(target.value)}
          />
        </label>
        <button type="submit">Login</button>
      </form>
    </div>
  );
};

export default Login;

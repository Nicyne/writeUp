import type { NextPage } from 'next';
import { useRouter } from 'next/router';
import Head from 'next/head';
import { SyntheticEvent, useContext, useEffect, useState } from 'react';
import { UserContext } from 'providers/userContextProvider';

const Login: NextPage = () => {
  const router = useRouter();
  const { currentUser, loading, mutate } = useContext(UserContext);
  const [username, setUsername] = useState<string>('');
  const [password, setPassword] = useState<string>('');

  useEffect(() => {
    if (currentUser && !loading) router.push('/app');
  }, [currentUser, loading]);

  const login = async (e: SyntheticEvent) => {
    e.preventDefault();

    const res = await fetch('http://localhost:8080/api/auth', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      credentials: 'include',
      body: JSON.stringify({
        username: username,
        passwd: password,
      }),
    });

    if (res.ok) {
      await mutate('user');
      await router.push('/app');
    }
  };

  return (
    <div className="login">
      <Head>
        <title>Login | Writeup</title>
      </Head>

      <form onSubmit={login}>
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
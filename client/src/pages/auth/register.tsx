import styles from 'styles/pages/login.module.scss';
import type { NextPage } from 'next';
import { useRouter } from 'next/router';
import Head from 'next/head';
import { SyntheticEvent, useContext, useEffect, useState } from 'react';
import { UserContext } from 'context';
import { dApi } from 'lib';

const Login: NextPage = () => {
  const router = useRouter();
  const { currentUser, loading } = useContext(UserContext);
  const [username, setUsername] = useState<string>('');
  const [password, setPassword] = useState<string>('');

  useEffect(() => {
    if (typeof window === 'undefined') return;
    if (currentUser && !loading) router.push('/app');
  }, [currentUser, loading, router]);

  const register = async (e: SyntheticEvent) => {
    e.preventDefault();

    try {
      await dApi.addUser(username, password);

      await router.push('/auth/login');
    } catch (err: any) {
      console.error(err);
    }
  };

  return (
    <div className="login">
      <Head>
        <title>Register | Writeup</title>
      </Head>

      <form onSubmit={register}>
        <h1>WriteUp</h1>
        <label htmlFor="username">
          Username
          <input
            id="username"
            name="username"
            type="text"
            placeholder="Username"
            onChange={({ target }) => setUsername(target.value)}
          />
        </label>
        <label htmlFor="password">
          Password
          <input
            id="password"
            name="password"
            type="password"
            placeholder="Password"
            onChange={({ target }) => setPassword(target.value)}
          />
        </label>
        <button className={styles.button} type="submit">
          Register
        </button>
      </form>
    </div>
  );
};

export default Login;

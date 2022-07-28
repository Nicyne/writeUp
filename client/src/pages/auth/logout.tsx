import type { NextPage } from 'next';
import Head from 'next/head';
import { UserContext } from 'context';
import { dApi } from 'lib';
import { useRouter } from 'next/router';
import { useContext, useEffect } from 'react';

const Logout: NextPage = () => {
  const { currentUser, loading, mutate } = useContext(UserContext);
  const router = useRouter();

  const logout = async () => {
    await dApi.logout();
    mutate();
  };

  useEffect(() => {
    if (typeof window === 'undefined') return;
    if (!currentUser || loading) return;
    console.log('entered');
    logout();
  }, []);

  return (
    <>
      <Head>
        <title>Logout | writeUp</title>
        <meta name="description" content="Next Page" />
      </Head>
    </>
  );
};

export default Logout;

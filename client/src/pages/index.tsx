import type { NextPage } from 'next';
import Head from 'next/head';
import Link from 'next/link';

const Home: NextPage = () => {
  return (
    <>
      <Head>
        <title>WriteUp</title>
      </Head>

      <h1>Hello World</h1>
      <Link href="app/">To app</Link>
    </>
  );
};

export default Home;

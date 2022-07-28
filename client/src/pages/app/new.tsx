import { Editor } from 'components';
import type { NextPage } from 'next';
import Head from 'next/head';

const New: NextPage = () => {
  return (
    <>
      <Head>
        <title>Next Page</title>
        <meta name="description" content="Next Page" />
      </Head>

      <Editor />
    </>
  );
};

export default New;

import 'styles/globals.scss';
import type { AppProps } from 'next/app';
import { UserContextProvider } from 'context';
import { MainLayout } from 'components';

function MyApp({ Component, pageProps }: AppProps) {
  return (
    <UserContextProvider>
      <MainLayout>
        <Component {...pageProps} />
      </MainLayout>
    </UserContextProvider>
  );
}

export default MyApp;

import 'styles/globals.scss';
import type { AppProps } from 'next/app';
import { KeyContextProvider, UserContextProvider } from 'context';
import { MainLayout } from 'components';

function MyApp({ Component, pageProps }: AppProps) {
  return (
    <UserContextProvider>
      <KeyContextProvider>
        <MainLayout>
          <Component {...pageProps} />
        </MainLayout>
      </KeyContextProvider>
    </UserContextProvider>
  );
}

export default MyApp;

import '../styles/globals.css';
import type { AppProps } from 'next/app';
import { UserContextProvider } from '../providers/userContextProvider';
import { MainLayout } from 'components/layouts';

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

import { App } from 'pages/app';
import { Login } from 'pages/login';
import { Signup } from 'pages/signup';
import { Route, Routes } from 'react-router-dom';
import { DefaultLayout } from 'components';

function withDefaultLayout(element: React.ReactNode) {
  return <DefaultLayout>{element}</DefaultLayout>;
}

export function Router() {
  return (
    <Routes>
      <Route path="/" element={withDefaultLayout(<App />)} />
      <Route path="/login" element={withDefaultLayout(<Login />)} />
      <Route path="/signup" element={withDefaultLayout(<Signup />)} />
      <Route
        path="*"
        element={
          <>
            <h1>Error</h1>
          </>
        }
      />
    </Routes>
  );
}

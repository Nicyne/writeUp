import { Route, Routes } from 'react-router-dom';
import { App } from 'pages/app';
import { Login } from 'pages/login';
import { Logout } from 'pages/logout';
import { Signup } from 'pages/signup';

import { DefaultLayout } from 'components';

function withDefaultLayout(element: React.ReactNode) {
  return <DefaultLayout>{element}</DefaultLayout>;
}

export function Router() {
  return (
    <Routes>
      <Route path="/" element={withDefaultLayout(<App />)} />
      <Route path="/login" element={withDefaultLayout(<Login />)} />
      <Route path="/logout" element={withDefaultLayout(<Logout />)} />
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

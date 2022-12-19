import { Route, Routes } from 'react-router-dom';
import { App, Landing, Login, Logout, NotFound, Signup } from 'pages';

import { DefaultLayout } from 'components';

function withDefaultLayout(element: React.ReactNode) {
  return <DefaultLayout>{element}</DefaultLayout>;
}

export function Router() {
  return (
    <Routes>
      <Route path="/" element={withDefaultLayout(<Landing />)} />
      <Route path="/app" element={withDefaultLayout(<App />)} />
      <Route path="/login" element={withDefaultLayout(<Login />)} />
      <Route path="/logout" element={withDefaultLayout(<Logout />)} />
      <Route path="/signup" element={withDefaultLayout(<Signup />)} />
      <Route path="*" element={withDefaultLayout(<NotFound />)} />
    </Routes>
  );
}

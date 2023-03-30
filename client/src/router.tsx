import { Route, Routes } from 'react-router-dom';
import { useAuth } from 'hooks';
import { App, Landing, Login, Logout, NotFound, Signup } from 'pages';

import { DefaultLayout, ProtectedRoute } from 'components';

function withDefaultLayout(element: React.ReactNode) {
  return <DefaultLayout>{element}</DefaultLayout>;
}

export function Router() {
  const { user, isLoading } = useAuth();

  return (
    <Routes>
      <Route path="/" element={withDefaultLayout(<Landing />)} />
      <Route
        path="/app"
        element={withDefaultLayout(
          <ProtectedRoute user={user} isLoading={isLoading}>
            <App />
          </ProtectedRoute>
        )}
      />
      <Route path="/login" element={withDefaultLayout(<Login />)} />
      <Route path="/logout" element={withDefaultLayout(<Logout />)} />
      <Route path="/signup" element={withDefaultLayout(<Signup />)} />
      <Route path="*" element={withDefaultLayout(<NotFound />)} />
    </Routes>
  );
}

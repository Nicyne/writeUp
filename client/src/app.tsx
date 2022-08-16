import { Route, Routes, Navigate } from 'react-router-dom';
import { Editor, Landing, Login, Logout, NoMatch, SignUp } from 'pages';
import React, { PropsWithChildren } from 'react';
import { DefaultLayout, EditorLayout } from 'components';
import { EditorContextProvider } from 'context';
import { useAuth } from 'hooks';

function withDefaultLayout(element: React.ReactNode) {
  return <DefaultLayout>{element}</DefaultLayout>;
}

export function ProtectedRoute(props: PropsWithChildren) {
  const { user, loading } = useAuth();

  if (!user && !loading && typeof window !== 'undefined') {
    return <Navigate to={'/login'} />;
  }

  return <>{props.children}</>;
}

export function App() {
  return (
    <Routes>
      <Route path="/" element={withDefaultLayout(<Landing />)} />
      <Route path="/login" element={withDefaultLayout(<Login />)} />
      <Route path="/logout" element={withDefaultLayout(<Logout />)} />
      <Route path="/signup" element={withDefaultLayout(<SignUp />)} />
      <Route
        path="/app"
        element={
          <ProtectedRoute>
            <EditorContextProvider>
              <EditorLayout>
                <Editor />
              </EditorLayout>
            </EditorContextProvider>
          </ProtectedRoute>
        }
      />
      <Route path="*" element={withDefaultLayout(<NoMatch />)} />
    </Routes>
  );
}

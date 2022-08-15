import { Route, Routes } from 'react-router-dom';
import { Editor, Landing, Login, Logout, NoMatch, SignUp } from 'pages';
import React from 'react';
import { DefaultLayout, EditorLayout } from 'components';
import { EditorContextProvider } from 'context';

function withDefaultLayout(element: React.ReactNode) {
  return <DefaultLayout>{element}</DefaultLayout>;
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
          <EditorContextProvider>
            <EditorLayout>
              <Editor />
            </EditorLayout>
          </EditorContextProvider>
        }
      />
      <Route path="*" element={withDefaultLayout(<NoMatch />)} />
    </Routes>
  );
}

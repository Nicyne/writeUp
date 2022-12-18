import React from 'react';
import ReactDOM from 'react-dom/client';
import { BrowserRouter } from 'react-router-dom';
import { AuthContextProvider } from 'context';
import { Router } from 'components';
import 'styles/index.css';
import 'i18n';

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
  <React.StrictMode>
    <AuthContextProvider>
      <BrowserRouter>
        <Router />
      </BrowserRouter>
    </AuthContextProvider>
  </React.StrictMode>
);

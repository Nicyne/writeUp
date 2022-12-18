import { useEffect, useState } from 'react';
import { useTranslation } from 'react-i18next';
import reactLogo from 'assets/react.svg';

import { LanguageSelector } from 'components';

import 'styles/App.css';

export function App() {
  const [count, setCount] = useState(0);
  const [t] = useTranslation();

  useEffect(() => {
    fetch('/api/system')
      .then((res) => res.json())
      .then((res) => console.log(res));
  }, []);

  return (
    <div className="App">
      <div>
        <a href="https://vitejs.dev" target="_blank" rel="noreferrer">
          <img src="/vite.svg" className="logo" alt="Vite logo" />
        </a>
        <a href="https://reactjs.org" target="_blank" rel="noreferrer">
          <img src={reactLogo} className="logo react" alt="React logo" />
        </a>
      </div>
      <h1>{t('test', { ns: 'errors' })}</h1>
      <h2>{t('appName')}</h2>
      <div className="card">
        <button onClick={() => setCount((count) => count + 1)}>
          count is {count}
        </button>
        <p>
          Edit <code>src/App.tsx</code> and save to test HMR
        </p>
      </div>
      <p className="read-the-docs">
        Click on the Vite and React logos to learn more
      </p>
      <LanguageSelector />
    </div>
  );
}

export default App;
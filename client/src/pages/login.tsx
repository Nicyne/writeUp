import { FormEvent, useState } from 'react';
import { useTranslation } from 'react-i18next';
import { useNavigate } from 'react-router-dom';
import { useAuth } from 'hooks';

export function Login() {
  const { login } = useAuth();
  const [t] = useTranslation();
  const [username, setUsername] = useState('');
  const [password, setPassword] = useState('');
  const navigate = useNavigate();

  const submit = async (e: FormEvent) => {
    e.preventDefault();

    const result = await login(username, password);
    if (!result.success) {
      console.log(result);
      return;
    }

    navigate('/');
  };

  return (
    <div className="container">
      <div className="center">
        <article className="form">
          <header>
            <h1>{t('auth.login.name')}</h1>
          </header>
          <form onSubmit={submit}>
            <label htmlFor="username">
              {t('auth.username')}
              <input
                type="text"
                name={t('auth.username')}
                id="username"
                placeholder={t('auth.username')}
                value={username}
                onChange={(e) => setUsername(e.target.value)}
              />
            </label>

            <label htmlFor="password">
              {t('auth.password')}
              <input
                type="password"
                name={t('auth.password')}
                id="password"
                placeholder={t('auth.password')}
                value={password}
                onChange={(e) => setPassword(e.target.value)}
              />
            </label>

            <button type="submit" className="w-full">
              {t('auth.login.action')}
            </button>
          </form>
        </article>
      </div>
    </div>
  );
}

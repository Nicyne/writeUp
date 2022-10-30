import { FormEvent, useState } from 'react';
import { useTranslation } from 'react-i18next';
import { useNavigate, useSearchParams } from 'react-router-dom';
import { useAuth } from 'hooks';
import { capitalFirstLetter } from 'utils';

export function Login() {
  const { login } = useAuth();
  const [t] = useTranslation();
  const [username, setUsername] = useState('');
  const [password, setPassword] = useState('');
  const [sessionOnly, setSessionOnly] = useState(true);
  const [error, setError] = useState('');
  const [searchParams] = useSearchParams();
  const navigate = useNavigate();

  const submit = async (e: FormEvent) => {
    e.preventDefault();

    const result = await login(username, password, sessionOnly);
    if (!result.success) {
      console.log(result);
      setError(result.message);
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
            {searchParams.get('success') && (
              <span className="banner success">
                <header>
                  <h3>{t('common.success') + '!'}</h3>
                </header>
                <p>{t('auth.signupSuccess')}</p>
              </span>
            )}

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

            <label htmlFor="rememberMe" className="flex flex-reverse">
              {t('auth.login.rememberMe')}
              <input
                type="checkbox"
                name={t('auth.login.rememberMe')}
                id="rememberMe"
                placeholder={t('auth.login.rememberMe')}
                value={password}
                onChange={(e) => setSessionOnly(!e.target.checked)}
              />
            </label>

            {error && (
              <span className="danger">{capitalFirstLetter(error)}</span>
            )}

            <button type="submit" className="w-full">
              {t('auth.login.action')}
            </button>
          </form>
        </article>
      </div>
    </div>
  );
}

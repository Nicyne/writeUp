import { FormEvent, useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { useAuth } from 'hooks';
import { useTranslation } from 'react-i18next';
import { emptyInputValue, IInputValue } from 'types';

export function SignUp() {
  const { signUp } = useAuth();
  const [username, setUsername] = useState(emptyInputValue());
  const [password, setPassword] = useState(emptyInputValue());
  const [betaKey, setBetaKey] = useState(emptyInputValue());
  const [confirmPassword, setConfirmPassword] = useState(emptyInputValue());
  const [t] = useTranslation();
  const navigate = useNavigate();

  const submit = async (e: FormEvent) => {
    e.preventDefault();

    if (password.value !== confirmPassword.value) {
      setConfirmPassword({
        ...confirmPassword,
        invalid: true,
        error: "Passwords don't match",
      });
      return;
    }

    const result = await signUp(username.value, password.value, betaKey.value);
    if (!result.success) {
      console.log(result);
      return;
    }

    navigate('/login?success=true');
  };

  const onChange = (
    setter: React.Dispatch<React.SetStateAction<IInputValue>>,
    name: string,
    minLength: number,
    value: string
  ) => {
    let invalid = false;
    let error = '';
    if (value.length < minLength && value.length !== 0) {
      invalid = true;
      error = t('error.tooShort', {
        name: t('auth.' + name.toLowerCase()),
        length: minLength,
      });
    }
    setter({ value, invalid, error });
  };

  return (
    <div className="container">
      <div className="center">
        <article className="form">
          <header>
            <h1>{t('auth.signup.name')}</h1>
          </header>
          <form onSubmit={submit}>
            <label htmlFor="username">
              {t('auth.username')}
              <input
                type="text"
                name={t('auth.username')}
                id="username"
                placeholder={t('auth.username')}
                spellCheck="false"
                aria-invalid={username.invalid}
                value={username.value}
                onChange={(e) =>
                  onChange(setUsername, 'Username', 3, e.target.value)
                }
              />
              <span className="danger">{username.error}</span>
            </label>

            <label htmlFor="password">
              {t('auth.password')}
              <input
                type="password"
                name={t('auth.password')}
                id="password"
                placeholder={t('auth.password')}
                spellCheck="false"
                aria-invalid={password.invalid}
                value={password.value}
                onChange={(e) =>
                  onChange(setPassword, 'Password', 8, e.target.value)
                }
              />
              <span className="danger">{password.error}</span>
            </label>

            <label htmlFor="confirm_password">
              {t('auth.confirmPassword')}
              <input
                type="password"
                name={t('auth.confirmPassword')}
                id="confirm_password"
                placeholder={t('auth.confirmPassword')}
                spellCheck="false"
                aria-invalid={confirmPassword.invalid}
                value={confirmPassword.value}
                onChange={(e) =>
                  onChange(setConfirmPassword, 'Password', 8, e.target.value)
                }
              />
              <span className="danger">{confirmPassword.error}</span>
            </label>

            <label htmlFor="betaKey">
              {t('auth.signup.betaKey')}
              <input
                type="password"
                name={t('auth.signup.betaKey')}
                id="betaKey"
                placeholder={t('auth.signup.betaKey')}
                spellCheck="false"
                aria-invalid={betaKey.invalid}
                value={betaKey.value}
                onChange={(e) =>
                  onChange(setBetaKey, 'BetaKey', 0, e.target.value)
                }
              />
              <span className="danger">{betaKey.error}</span>
            </label>

            <button type="submit" className="w-full">
              {t('auth.signup.action')}
            </button>
          </form>
        </article>
      </div>
    </div>
  );
}

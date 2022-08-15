import { useAuth } from 'hooks/useAuth';
import { FormEvent, useState } from 'react';
import { useNavigate } from 'react-router-dom';

interface IInputValue {
  value: string;
  invalid: boolean;
  error: string;
}

function emptyInputValue(): IInputValue {
  return {
    value: '',
    invalid: false,
    error: '',
  };
}

export function SignUp() {
  const { signUp } = useAuth();
  const [username, setUsername] = useState(emptyInputValue());
  const [password, setPassword] = useState(emptyInputValue());
  const [confirmPassword, setConfirmPassword] = useState(emptyInputValue());
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

    const result = await signUp(username.value, password.value);
    if (!result.success) {
      console.log(result);
      return;
    }

    navigate('/login');
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
      error = `${name} must contain ${minLength} characters or more.`;
    }
    setter({ value, invalid, error });
  };

  return (
    <div className="container">
      <div className="center">
        <article className="form">
          <header>
            <h1>Sign up</h1>
          </header>
          <form onSubmit={submit}>
            <label htmlFor="username">
              Username
              <input
                type="text"
                name="username"
                id="username"
                placeholder="Username"
                spellCheck="false"
                pattern=".{3,}"
                aria-invalid={username.invalid}
                value={username.value}
                onChange={(e) =>
                  onChange(setUsername, 'Username', 3, e.target.value)
                }
              />
              <span className="danger">{username.error}</span>
            </label>

            <label htmlFor="password">
              Password
              <input
                type="password"
                name="password"
                placeholder="Password"
                spellCheck="false"
                pattern=".{8,}"
                aria-invalid={password.invalid}
                value={password.value}
                onChange={(e) =>
                  onChange(setPassword, 'Password', 8, e.target.value)
                }
              />
              <span className="danger">{password.error}</span>
            </label>

            <label htmlFor="confirm_password">
              Confirm Password
              <input
                type="password"
                name="confirm_password"
                id="confirm_password"
                placeholder="Confirm Password"
                spellCheck="false"
                pattern=".{8,}"
                aria-invalid={confirmPassword.invalid}
                value={confirmPassword.value}
                onChange={(e) =>
                  onChange(setConfirmPassword, 'Password', 8, e.target.value)
                }
              />
              <span className="danger">{confirmPassword.error}</span>
            </label>

            <button type="submit" className="w-full">
              Sign up
            </button>
          </form>
        </article>
      </div>
    </div>
  );
}

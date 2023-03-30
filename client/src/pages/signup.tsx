import { FormEvent, useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { useAuth } from 'hooks';

export function Signup() {
  const { signUp } = useAuth();
  const [username, setUsername] = useState('');
  const [password, setPassword] = useState('');
  const [betaKey, setBetaKey] = useState('');
  const navigate = useNavigate();

  const submit = async (e: FormEvent) => {
    e.preventDefault();

    const result = await signUp(username, password, betaKey);
    if (!result.success) {
      console.log(result);
      return;
    }
    navigate('/login');
  };

  return (
    <>
      <form onSubmit={submit}>
        <input
          type="text"
          name="username"
          id="username"
          placeholder="username"
          spellCheck="false"
          onChange={({ target }) => setUsername(target.value)}
        />

        <input
          type="password"
          name="password"
          id="password"
          placeholder="password"
          spellCheck="false"
          onChange={({ target }) => setPassword(target.value)}
        />

        <input
          type="password"
          name="betaKey"
          id="betakey"
          placeholder="betaKey"
          spellCheck="false"
          onChange={({ target }) => setBetaKey(target.value)}
        />

        <button type="submit">Signup</button>
      </form>
    </>
  );
}

export default Signup;

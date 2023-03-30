import { FormEvent, useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { useAuth } from 'hooks';

export function Login() {
  const { login } = useAuth();
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
    navigate('/app');
  };

  return (
    <>
      <form onSubmit={submit}>
        <input type="text" onChange={(e) => setUsername(e.target.value)} />
        <input type="password" onChange={(e) => setPassword(e.target.value)} />
        <button type="submit">Login</button>
      </form>
    </>
  );
}

export default Login;

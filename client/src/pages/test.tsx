import type { NextPage } from 'next';
import { SyntheticEvent, useEffect, useState } from 'react';

const Test: NextPage = () => {
  const [username, setUsername] = useState<string | undefined>(undefined);
  const [password, setPassword] = useState<string | undefined>(undefined);

  const [authed, setAuthed] = useState<boolean>(false);

  useEffect(() => {
    try {
      fetch('http://localhost:8080/api/user', {
        method: 'GET',
        credentials: 'include',
      }).then((res) => {
        if (res.status == 200) {
          setAuthed(true);
        }
      });
    } catch (error: any) {
      alert('err');
    }
  }, []);

  const Login = async (e: SyntheticEvent) => {
    e.preventDefault();
    try {
      const res = await fetch('http://localhost:8080/api/auth', {
        method: 'POST',
        credentials: 'include',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          username: username,
          passwd: password,
        }),
      });

      if (res.status == 200) {
        setAuthed(true);
      }
    } catch (error: any) {}
  };

  const Logout = async () => {
    const res = await fetch('http://localhost:8080/api/auth', {
      method: 'DELETE',
      credentials: 'include',
    });

    if (res.status == 200) {
      setAuthed(false);
    }
  };

  return (
    <div className="log">
      <h1>Youre {String(authed)}</h1>
      {!authed ? (
        <form onSubmit={Login}>
          <input
            type="text"
            onChange={({ target }) => setUsername(target.value)}
          />
          <input
            type="password"
            onChange={({ target }) => setPassword(target.value)}
          />
          <button type="submit">Login</button>
        </form>
      ) : (
        <button onClick={Logout}>Logout</button>
      )}
    </div>
  );
};

export default Test;

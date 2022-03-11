import type { NextPage } from 'next';
import { FunctionComponent, SyntheticEvent, useEffect, useState } from 'react';
import { PrismAsync as SyntaxHighlighter } from 'react-syntax-highlighter';
import { vscDarkPlus } from 'react-syntax-highlighter/dist/cjs/styles/prism';
import ReactMarkdown from 'react-markdown';
import remarkGfm from 'remark-gfm';

const Home: NextPage = () => {
  const [token, setToken] = useState('');
  const [username, setUsername] = useState('');
  const [password, setPassword] = useState('');
  const [notes, setNotes] = useState([]);
  const [curNote, setCurNote] = useState<any>({});
  const [user, setUser] = useState(null);

  useEffect(() => {
    try {
      fetch('http://localhost:8080/api/user', {
        method: 'GET',
        credentials: 'include',
      })
        .then((res) => res.json())
        .then((res) => {
          setUser(res);
        });
    } catch (error: any) {}
  }, []);

  const login = async (e: SyntheticEvent) => {
    e.preventDefault();

    const res = await fetch('http://localhost:8080/api/auth', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      credentials: 'include',
      body: JSON.stringify({
        username: username,
        passwd: password,
      }),
    }).then((res) => res.json());

    if (!res.success) return;

    const user = await fetch('http://localhost:8080/api/user', {
      method: 'GET',
      credentials: 'include',
    }).then((res) => res.json());

    setUser(user);

    const data = await fetch('http://localhost:8080/api/notes', {
      method: 'GET',
      credentials: 'include',
    }).then((res) => res.json());

    setNotes(data);
  };

  const loadNote = async (e: SyntheticEvent, id: number) => {
    e.preventDefault();

    const data = await fetch('http://localhost:8080/api/note/' + id, {
      method: 'GET',
      credentials: 'include',
    }).then((res) => res.json());

    if (data) {
      setCurNote(data.note);
    }
  };

  return (
    <>
      {!user ? (
        <form onSubmit={login}>
          <label htmlFor="username">
            Username
            <input
              id="username"
              name="username"
              type="text"
              onChange={({ target }) => setUsername(target.value)}
            />
          </label>
          <label htmlFor="password">
            Password
            <input
              id="password"
              name="password"
              type="password"
              onChange={({ target }) => setPassword(target.value)}
            />
          </label>
          <button type="submit">Submit</button>
        </form>
      ) : (
        <button
          onClick={async () => {
            await fetch('http://localhost:8080/api/auth', {
              method: 'DELETE',
              credentials: 'include',
            });
            setUser(null);
          }}
        >
          Logout
        </button>
      )}

      <ul>
        {notes.map((note: any) => (
          <li key={note.note_id} onClick={(e) => loadNote(e, note.note_id)}>
            {note.title}
          </li>
        ))}
      </ul>

      <div className="grid">
        <textarea
          name="input"
          spellCheck="false"
          id="input"
          value={curNote.content}
          onChange={({ target }) => {
            setCurNote({
              ...curNote,
              content: target.value,
            });
          }}
        />
        <div className="preview">
          <ReactMarkdown
            remarkPlugins={[remarkGfm]}
            components={{
              code({ node, inline, className, children, ...props }) {
                const match = /language-(\w+)/.exec(className || '');
                return !inline && match ? (
                  <Codeblock
                    value={String(children).replace(/\n$/, '')}
                    language={match[1]}
                  />
                ) : (
                  <code className={className} {...props}>
                    {children}
                  </code>
                );
              },
            }}
          >
            {curNote.content ?? ''}
          </ReactMarkdown>
        </div>
      </div>
    </>
  );
};

const Codeblock: FunctionComponent<{ value: string; language: string }> = ({
  value,
  language,
}) => {
  return (
    <SyntaxHighlighter language={language} style={vscDarkPlus}>
      {value}
    </SyntaxHighlighter>
  );
};

export default Home;

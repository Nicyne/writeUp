import type { NextPage } from 'next';
import { FunctionComponent, SyntheticEvent, useContext, useState } from 'react';
import { PrismAsync as SyntaxHighlighter } from 'react-syntax-highlighter';
import { vscDarkPlus } from 'react-syntax-highlighter/dist/cjs/styles/prism';
import ReactMarkdown from 'react-markdown';
import remarkGfm from 'remark-gfm';
import { useRouter } from 'next/router';
import { UserContext } from 'providers/userContextProvider';
import { useAsync } from '../../lib/useAsync';

const Home: NextPage = () => {
  const router = useRouter();
  const [notes, setNotes] = useState([]);
  const [curNote, setCurNote] = useState<any>({});
  const { currentUser, loading, mutate } = useContext(UserContext);

  const getNotes = async () => {
    try {
      const data = await fetch('http://localhost:8080/api/notes', {
        method: 'GET',
        credentials: 'include',
      })
        .then((res) => res.json())
        .catch((err) => {
          throw err;
        });

      setNotes(data);
    } catch (error: any) {
      setNotes([]);
    }
  };

  useAsync(getNotes, () => {});

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

  const logout = async () => {
    await fetch('http://localhost:8080/api/auth', {
      method: 'DELETE',
      credentials: 'include',
    });
    //setUser(false);
    await mutate('user');
    await getNotes();
  };

  if (loading) return <div>loading...</div>;

  return (
    <>
      {currentUser?.username}
      <button onClick={logout}>Logout</button>

      <ul>
        {notes.length > 0 ? (
          notes?.map((note: any) => (
            <li key={note.note_id} onClick={(e) => loadNote(e, note.note_id)}>
              {note.title}
            </li>
          ))
        ) : (
          <></>
        )}
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

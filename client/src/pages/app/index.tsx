import type { NextPage } from 'next';
import { FunctionComponent, SyntheticEvent, useContext, useState } from 'react';
import { PrismAsync as SyntaxHighlighter } from 'react-syntax-highlighter';
import { vscDarkPlus } from 'react-syntax-highlighter/dist/cjs/styles/prism';
import ReactMarkdown from 'react-markdown';
import remarkGfm from 'remark-gfm';
import { useRouter } from 'next/router';
import { UserContext } from 'providers/userContextProvider';

interface INote {
  note_id: string;
  note: {
    title: string;
    content: string;
    owner_id: string;
    tags: string[];
  };
  allowance: string[];
}

const Home: NextPage = () => {
  const router = useRouter();
  const [notes, setNotes] = useState<INote[]>([]);
  const [curNote, setCurNote] = useState<INote | undefined>(undefined);
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

      return setNotes(data);
    } catch (error: any) {
      return setNotes([]);
    }
  };

  const loadNote = async (e: SyntheticEvent, id: number) => {
    e.preventDefault();

    const data = await fetch('http://localhost:8080/api/note/' + id, {
      method: 'GET',
      credentials: 'include',
    }).then((res) => res.json());

    console.log(data);

    if (data) {
      setCurNote(data);
    }
  };

  const saveNote = async (e: SyntheticEvent) => {
    e.preventDefault();

    console.log(curNote);

    if (!curNote) return;

    const res = await fetch(
      'http://localhost:8080/api/note/' + curNote.note_id,
      {
        method: 'PUT',
        headers: { 'Content-Type': 'application/json' },
        credentials: 'include',
        body: JSON.stringify(curNote.note),
      }
    );
    await getNotes();
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
      <button onClick={getNotes}>Load Notes</button>
      <button onClick={saveNote}>Save Note</button>

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
          value={curNote?.note.content}
          onChange={({ target }) => {
            if (!curNote) return;
            setCurNote({
              ...curNote,
              note: {
                ...curNote.note,
                content: target.value,
              },
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
            {curNote?.note.content ?? ''}
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

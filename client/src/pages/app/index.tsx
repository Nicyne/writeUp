import type { NextPage } from 'next';
import { FunctionComponent, SyntheticEvent, useContext, useState } from 'react';
import { PrismAsync as SyntaxHighlighter } from 'react-syntax-highlighter';
import { vscDarkPlus } from 'react-syntax-highlighter/dist/cjs/styles/prism';
import ReactMarkdown from 'react-markdown';
import remarkGfm from 'remark-gfm';
import { UserContext } from 'providers/userContextProvider';
import type { INote } from 'types';
import { dApi } from 'lib';

const Home: NextPage = () => {
  const [notes, setNotes] = useState<INote[]>([]);
  const [curNote, setCurNote] = useState<INote | undefined>(undefined);
  const { currentUser, loading, mutate } = useContext(UserContext);

  const getNotes = async () => {
    const data = await dApi.getNotes();
    setNotes(data);
  };

  const loadNote = async (e: SyntheticEvent, id: string) => {
    const data = await dApi.getNote(id);
    setCurNote(data);
  };

  const saveNote = async (e: SyntheticEvent) => {
    if (!curNote) return;
    await dApi.saveNote(curNote);
    await getNotes();
  };

  const logout = async () => {
    await dApi.logout();
    await mutate('user');
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

import type { NextPage } from 'next';
import { FunctionComponent, SyntheticEvent, useContext, useState } from 'react';
import { PrismAsync as SyntaxHighlighter } from 'react-syntax-highlighter';
import { vscDarkPlus } from 'react-syntax-highlighter/dist/cjs/styles/prism';
import ReactMarkdown from 'react-markdown';
import remarkGfm from 'remark-gfm';
import { UserContext } from 'providers/userContextProvider';
import type { INote, INoteShallow } from 'types';
import { dApi } from 'lib';

const Home: NextPage = () => {
  const [notes, setNotes] = useState<INoteShallow[]>([]);
  const [curNote, setCurNote] = useState<INote | undefined>(undefined);
  const { currentUser, loading, mutate } = useContext(UserContext);
  const [newTitle, setNewTitle] = useState<string>('');

  const getNotes = async () => {
    const data = await dApi.getNotes();
    console.log(data);
    setNotes(data);
  };

  const loadNote = async (e: SyntheticEvent, id: string) => {
    const data = await dApi.getNote(id);
    setCurNote(data);
  };

  const addNote = async (e: SyntheticEvent) => {
    e.preventDefault();

    const note = await dApi.addNote(newTitle, '', []);
    setNotes([
      ...notes,
      {
        title: note.note.title,
        note_id: note.note_id,
        allowance: note.allowance,
        tags: note.note.tags,
      },
    ]);
  };

  const saveNote = async (e: SyntheticEvent) => {
    if (!curNote) return;
    await dApi.updateNote(curNote);
    await getNotes();
  };

  const deleteNote = async (e: SyntheticEvent) => {
    if (!curNote) return;
    await dApi.deleteNote(curNote.note_id);
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
      <button onClick={saveNote} disabled={curNote?.allowance == 'Read'}>
        Save Note
      </button>
      <button onClick={deleteNote}>Delete Note</button>
      <form onSubmit={addNote}>
        <input
          type="text"
          onChange={({ target }) => setNewTitle(target.value)}
        />
        <button type="submit" disabled={newTitle == ''}>
          Create
        </button>
      </form>

      <ul>
        {notes.length > 0 ? (
          notes?.map((note) => (
            <li key={note.note_id} onClick={(e) => loadNote(e, note.note_id)}>
              {note.title} {note.allowance == 'Read' ? '(readonly)' : ''}
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

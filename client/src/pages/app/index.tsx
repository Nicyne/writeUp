import type { NextPage } from 'next';
import { SyntheticEvent, useContext, useEffect, useRef, useState } from 'react';
import ReactMarkdown from 'react-markdown';
import remarkGfm from 'remark-gfm';
import remarkEmoji from 'remark-emoji';
import type { INote, INoteShallow } from 'types';
import { CodeBlock } from 'components';
import { dApi } from 'lib';
import { UserContext } from 'context';

const closing: Record<string, string> = {
  '(': ')',
  '[': ']',
  '{': '}',
  '<': '>',
  '"': '"',
};

const Home: NextPage = () => {
  const [notes, setNotes] = useState<INoteShallow[]>([]);
  const [curNote, setCurNote] = useState<INote | undefined>(undefined);
  const [newTitle, setNewTitle] = useState<string>('');
  const { currentUser, loading } = useContext(UserContext);

  const [shareCode, setShareCode] = useState<string>('');
  const [addShare, setAddShare] = useState<string>('');
  const [delShare, setDelShare] = useState<string>('');

  const addNoteForm = useRef<HTMLFormElement>(null);
  const inputArea = useRef<HTMLTextAreaElement>(null);

  useEffect(() => {
    if (typeof window === 'undefined') return;
    if (currentUser && !loading) {
      getNotes();
    }
  }, [currentUser, loading]);

  const getNotes = async () => {
    const data = await dApi.getNotes();
    setNotes(data);
  };

  const loadNote = async (e: SyntheticEvent, id: string) => {
    console.log('called');
    if (curNote) {
      await saveNote(e);
    }

    const data = await dApi.getNote(id);
    setCurNote(data);
  };

  const addNote = async (e: SyntheticEvent) => {
    e.preventDefault();

    if (addNoteForm.current) {
      addNoteForm.current.reset();
    }

    const response = await dApi.addNote(newTitle, '', []);
    setNotes([
      ...notes,
      {
        title: response.note.title,
        note_id: response.note_id,
        allowance: response.allowance,
        tags: response.note.tags,
      },
    ]);
    await loadNote(e, response.note_id);
    inputArea.current?.focus();
  };

  const saveNote = async (e: SyntheticEvent) => {
    if (!curNote) return;
    if (curNote.allowance == 'Read') return;
    await dApi.updateNote(curNote);
    await getNotes();
  };

  const deleteNote = async (e: SyntheticEvent, noteId: string) => {
    e.stopPropagation();
    if (curNote) setCurNote(undefined);

    await dApi.deleteNote(noteId);
    await getNotes();
  };

  return (
    <>
      <div className="app">
        <div className="sidebar">
          <div className="sidebar-buttons">
            <button
              onClick={async (e) => {
                const token = await dApi.getShareToken();
                setShareCode(token);
              }}
            >
              Get Share Token
            </button>
            <p>{shareCode}</p>
            <form
              onSubmit={async (e) => {
                try {
                  e.preventDefault();
                  const res = await dApi.addRelation(addShare);
                  console.log(res);
                } catch (error: any) {
                  console.error(error);
                }
              }}
            >
              <input
                type="text"
                placeholder="Share Token"
                onChange={({ target }) => setAddShare(target.value)}
              />
              <button type="submit" disabled={!addShare}>
                Create Relation
              </button>
            </form>
            <form
              onSubmit={async (e) => {
                e.preventDefault();
                try {
                  const res = await dApi.deleteRelation(delShare);
                  console.log(res);
                } catch (error: any) {
                  console.error(error);
                }
              }}
            >
              <input
                type="text"
                placeholder="userid"
                onChange={({ target }) => setDelShare(target.value)}
              />
              <button type="submit">Delete Relation</button>
            </form>
            <button onClick={getNotes}>Load Notes</button>
            <button onClick={saveNote} disabled={curNote?.allowance == 'Read'}>
              Save Note
            </button>
            {curNote && (
              <button
                onClick={async (e) => {
                  await saveNote(e);
                  setCurNote(undefined);
                }}
              >
                Close Note
              </button>
            )}
            <form onSubmit={addNote} ref={addNoteForm}>
              <input
                type="text"
                placeholder="New Note Name"
                onChange={({ target }) => setNewTitle(target.value)}
              />
              <button type="submit" disabled={newTitle == ''}>
                Create
              </button>
            </form>
          </div>

          <ul>
            {notes.length > 0 ? (
              notes.map((note) => (
                <li
                  key={note.note_id}
                  onClick={(e) => loadNote(e, note.note_id)}
                >
                  {note.title} {note.allowance == 'Read' ? '(readonly)' : ''}
                  {note.allowance == 'Owner' && (
                    <button onClick={(e) => deleteNote(e, note.note_id)}>
                      &#x2715;
                    </button>
                  )}
                </li>
              ))
            ) : (
              <></>
            )}
          </ul>
        </div>

        {curNote && (
          <div className="grid">
            <textarea
              name="input"
              spellCheck="false"
              id="input"
              value={curNote?.note.content}
              ref={inputArea}
              onKeyUp={(e) => {
                const key = e.key;

                if (!closing[key]) return;

                const pos = e.currentTarget.selectionStart;
                const value = e.currentTarget.value;
                e.currentTarget.value = [
                  value.slice(0, pos),
                  closing[key],
                  value.slice(pos),
                ].join('');
                e.currentTarget.selectionEnd = pos;

                setCurNote({
                  ...curNote,
                  note: {
                    ...curNote.note,
                    content: e.currentTarget.value,
                  },
                });
              }}
              onChange={(e) => {
                if (!curNote) return;
                setCurNote({
                  ...curNote,
                  note: {
                    ...curNote.note,
                    content: e.target.value,
                  },
                });
              }}
            />
            <div className="preview">
              <ReactMarkdown
                remarkPlugins={[remarkGfm, remarkEmoji]}
                components={{
                  code({ node, inline, className, children, ...props }) {
                    const match = /language-(\w+)/.exec(className || '');
                    return !inline && match ? (
                      <CodeBlock
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
        )}
      </div>
    </>
  );
};

export default Home;

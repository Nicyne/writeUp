import type { NextPage } from 'next';
import { SyntheticEvent, useContext, useEffect, useRef, useState } from 'react';
import { useTimer } from 'react-timer-hook';
import ReactMarkdown from 'react-markdown';
import remarkGfm from 'remark-gfm';
import remarkEmoji from 'remark-emoji';

// internal imports
import type { INote, INoteShallow } from 'types';
import { CodeBlock } from 'components';
import { dApi, getHash } from 'lib';
import { UserContext } from 'context';

const closing: Record<string, string> = {
  '(': ')',
  '[': ']',
  '{': '}',
  '<': '>',
  '"': '"',
};

function getSeconds(seconds: number): Date {
  const timestamp = new Date();
  timestamp.setSeconds(timestamp.getSeconds() + seconds);
  return timestamp;
}

const Home: NextPage = () => {
  const { currentUser, loading } = useContext(UserContext);

  const [notes, setNotes] = useState<INoteShallow[]>([]);
  const [curNote, setCurNote] = useState<INote | undefined>(undefined);
  const [newTitle, setNewTitle] = useState<string>('');

  // AUTO SAVE
  const [noteHash, setNoteHash] = useState<string>('');
  const time = getSeconds(5);
  const { pause, restart } = useTimer({
    expiryTimestamp: time,
    onExpire: async () => {
      if (!curNote) {
        pause();
        return;
      }
      const hash = getHash(curNote.note.content);
      if (hash === noteHash) {
        return;
      }
      await saveNote();
    },
  });
  // AUTO SAVE

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
    const data = await dApi.getNote(id);
    setNoteHash(getHash(data.note.content));
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

  const saveNote = async (e?: SyntheticEvent) => {
    if (!curNote) return;
    if (curNote.allowance == 'Read') return;
    await dApi.updateNote(curNote);
    await getNotes();
  };

  const deleteNote = async (e: SyntheticEvent, noteId: string) => {
    e.stopPropagation();
    if (curNote?.note_id === noteId) setCurNote(undefined);

    await dApi.deleteNote(noteId);
    await getNotes();
  };

  const closeNote = async (e: SyntheticEvent) => {
    await saveNote(e);
    setCurNote(undefined);
    pause();
  };

  const insertElement = (
    e: SyntheticEvent,
    element: string,
    cursorPosition?: number
  ) => {
    if (!inputArea.current || !curNote) return;
    const pos = inputArea.current.selectionStart;
    const value = inputArea.current.value;
    inputArea.current.value = [
      value.slice(0, pos),
      element,
      value.slice(pos),
    ].join('');
    inputArea.current.focus();
    cursorPosition = cursorPosition ?? element.length;
    inputArea.current.selectionStart = pos + cursorPosition;

    setCurNote({
      ...curNote,
      note: {
        ...curNote.note,
        content: inputArea.current.value,
      },
    });
  };

  return (
    <div className="app">
      <div className="menuStrip">
        <div className="segment">
          <button className="widget" onClick={getNotes} title="Refresh Notes">
            R
          </button>
          {curNote && (
            <>
              <button className="widget" onClick={closeNote} title="Close Note">
                X
              </button>
              <button className="widget" onClick={saveNote} title="Save Note">
                S
              </button>
            </>
          )}
        </div>

        {curNote && (
          <div className="segment">
            <>
              <button
                className="widget"
                onClick={(e) => insertElement(e, '- [ ] ')}
                title="Insert Todo"
              >
                T
              </button>
              <button
                className="widget"
                onClick={(e) => insertElement(e, '- [x] ')}
                title="Insert Todo Done"
              >
                Tx
              </button>
              <button
                className="widget"
                onClick={(e) => insertElement(e, '```lang\n\n```', 7)}
                title="Insert Code Block"
              >
                Cb
              </button>
            </>
          </div>
        )}
      </div>

      <div className="editor">
        <div className="sidebar">
          <div className="sidebar-buttons">
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
                    <button
                      onClick={(e) => deleteNote(e, note.note_id)}
                      title="Delete note"
                    >
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
                restart(getSeconds(5));
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
    </div>
  );
};

/**
 * <button
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
 */

export default Home;

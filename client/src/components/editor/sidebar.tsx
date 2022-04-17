import {
  FunctionComponent,
  SyntheticEvent,
  useCallback,
  useContext,
  useEffect,
  useState,
} from 'react';
import { EditorContext, UserContext } from 'context';
import { dApi } from 'lib';

export const Sidebar: FunctionComponent = () => {
  const { currentUser, loading } = useContext(UserContext);
  const { setNote, notes, setNotes } = useContext(EditorContext);
  const [tags, setTags] = useState<string[]>([]);

  useEffect(() => {
    let tags = notes.flatMap((note) => note.tags);
    let uniqueTags = tags.filter((tag, index) => tags.indexOf(tag) == index);
    setTags(uniqueTags);
  }, [notes]);

  const getNotes = useCallback(async () => {
    const data = await dApi.getNotes();
    setNotes(data);
  }, [setNotes]);

  const loadNote = async (id: string) => {
    const data = await dApi.getNote(id);
    setNote(data);
  };

  const deleteNote = async (e: SyntheticEvent, id: string) => {
    e.stopPropagation();
    await dApi.deleteNote(id);
    getNotes();
  };

  useEffect(() => {
    if (typeof window === 'undefined') return;
    if (!currentUser && !loading) return;
    getNotes();
  }, [currentUser, loading, getNotes]);

  return (
    <div>
      {tags.map((tag) => (
        <div key={tag}>
          <h2>{tag}</h2>
          <ul>
            {notes.map((note) => {
              if (note.tags.includes(tag))
                return (
                  <li
                    key={note.note_id}
                    onClick={(e) => loadNote(note.note_id)}
                  >
                    {note.title}
                    <button onClick={(e) => deleteNote(e, note.note_id)}>
                      X
                    </button>
                  </li>
                );
            })}
          </ul>
        </div>
      ))}
    </div>
  );
};

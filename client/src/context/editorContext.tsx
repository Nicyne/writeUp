import axios from 'axios';
import {
  createContext,
  Dispatch,
  PropsWithChildren,
  RefObject,
  SetStateAction,
  useRef,
  useState,
} from 'react';
import { INote, INoteShallow } from 'types';

export interface IEditorContext {
  currentNote?: INote;
  setNote: Dispatch<SetStateAction<INote | undefined>>;
  notes: INoteShallow[];
  setNotes: Dispatch<SetStateAction<INoteShallow[]>>;
  refs: {
    bodyEditor: RefObject<HTMLDialogElement> | null;
    newNoteDialog: RefObject<HTMLDialogElement> | null;
  };
  getNotes: () => Promise<void>;
  getNote: (id: string) => Promise<void>;
  deleteNote: (id: string) => Promise<boolean>;
}

export const EditorContext = createContext<IEditorContext>({
  currentNote: undefined,
  setNote: () => {},
  notes: [],
  setNotes: () => {},
  refs: {
    bodyEditor: null,
    newNoteDialog: null,
  },
  getNotes: async () => {},
  getNote: async () => {},
  deleteNote: async () => false,
});

export const EditorContextProvider = (props: PropsWithChildren) => {
  const [currentNote, setNote] = useState<INote>();
  const [notes, setNotes] = useState<INoteShallow[]>([]);
  const bodyEditor = useRef(null);
  const newNoteDialog = useRef(null);

  const getNote = async (id: string) => {
    const res = await axios('/api/note/' + id);
    if (!res.data.success) {
      console.error(res.data.code);
    }

    setNote(res.data.content);
  };

  const getNotes = async () => {
    const res = await axios('/api/notes');
    if (!res.data.success) {
      console.error(res.data.code);
    }

    setNotes(res.data.content);
  };

  const deleteNote = async (id: string): Promise<boolean> => {
    const res = await axios.delete('/api/note/' + id);
    if (!res.data.success) {
      console.error(res.data.message);
      return false;
    }

    setNotes(notes.filter((note) => note.note_id !== id));
    return true;
  };

  return (
    <EditorContext.Provider
      value={{
        currentNote,
        setNote,
        notes,
        setNotes,
        refs: { bodyEditor, newNoteDialog },
        getNotes,
        getNote,
        deleteNote,
      }}
    >
      {props.children}
    </EditorContext.Provider>
  );
};

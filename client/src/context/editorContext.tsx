import { NextPage } from 'next';
import { createContext, RefObject, useRef, useState } from 'react';
import { INote, INoteShallow } from 'types';

export interface IEditorContext {
  note: INote | undefined;
  setNote: Function;
  notes: INoteShallow[];
  setNotes: Function;
  refs: {
    bodyEditor: RefObject<HTMLTextAreaElement> | null;
  };
}

export const EditorContext = createContext<IEditorContext>({
  note: undefined,
  setNote: () => {},
  notes: [],
  setNotes: () => {},
  refs: {
    bodyEditor: null,
  },
});

export const EditorContextProvider: NextPage = ({ children }) => {
  const [note, setNote] = useState<INote>();
  const [notes, setNotes] = useState<INoteShallow[]>([]);
  const bodyEditor = useRef<HTMLTextAreaElement>(null);

  return (
    <EditorContext.Provider
      value={{ note, setNote, notes, setNotes, refs: { bodyEditor } }}
    >
      {children}
    </EditorContext.Provider>
  );
};

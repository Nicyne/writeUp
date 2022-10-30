import { EditorContext } from 'context';
import { useContext } from 'react';

export function useEditor() {
  const {
    currentNote,
    notes,
    setNote,
    setNotes,
    refs,
    getNote,
    getNotes,
    deleteNote,
  } = useContext(EditorContext);
  return {
    currentNote,
    notes,
    setNote,
    setNotes,
    refs,
    getNote,
    getNotes,
    deleteNote,
  };
}

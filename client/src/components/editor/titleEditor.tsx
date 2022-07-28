import { EditorContext } from 'context';
import { FunctionComponent, useContext } from 'react';

export const TitleEditor: FunctionComponent = () => {
  const { note, setNote } = useContext(EditorContext);

  const updateTitle = (title: string) => {
    let oldNote = Object.assign({}, note);
    oldNote.note.title = title;
    setNote(oldNote);
  };

  if (!note) return <></>;

  return (
    <input
      type="text"
      value={note?.note.title}
      onChange={(e) => updateTitle(e.target.value)}
    />
  );
};

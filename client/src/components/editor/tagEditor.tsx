import { EditorContext } from 'context';
import { FunctionComponent, useContext } from 'react';

export const TagEditor: FunctionComponent = () => {
  const { note, setNote } = useContext(EditorContext);

  const updateTags = (tags: string) => {
    let oldNote = Object.assign({}, note);
    oldNote.note.tags = tags.split(';');
    setNote(oldNote);
  };

  if (!note) return <></>;

  return (
    <input
      type="text"
      value={note?.note.tags.join(';')}
      onChange={(e) => updateTags(e.target.value)}
    />
  );
};

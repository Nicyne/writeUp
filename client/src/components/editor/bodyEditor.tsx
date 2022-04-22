import { FunctionComponent, KeyboardEvent, useContext } from 'react';
import { EditorContext } from 'context';
import { Widget } from './';

export const BodyEditor: FunctionComponent = () => {
  const { note, setNote, refs } = useContext(EditorContext);

  const closing: Record<string, string> = {
    '(': ')',
    '[': ']',
    '{': '}',
    '<': '>',
    '"': '"',
  };

  const updateContent = (content: string) => {
    if (!note) return;
    let oldNote = Object.assign({}, note);
    oldNote.note.content = content;
    setNote(oldNote);
  };

  const keyUp = (e: KeyboardEvent<HTMLTextAreaElement>) => {
    if (!note) return;
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

    updateContent(e.currentTarget.value);
  };

  if (!note) return <></>;

  return (
    <>
      <Widget label="T" snippet="- [ ] " />
      <Widget label="Tx" snippet="- [x] " />
      <textarea
        name="body"
        id="body"
        ref={refs.bodyEditor}
        value={note?.note.content}
        onKeyUp={keyUp}
        onChange={(e) => updateContent(e.target.value)}
      ></textarea>
    </>
  );
};

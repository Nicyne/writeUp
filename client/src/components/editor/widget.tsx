import { EditorContext } from 'context';
import { FunctionComponent, SyntheticEvent, useContext } from 'react';

interface IWidgetProps {
  label: string;
  snippet: string;
}

export const Widget: FunctionComponent<IWidgetProps> = ({ label, snippet }) => {
  const { note, setNote, refs } = useContext(EditorContext);
  const ref = refs.bodyEditor;

  const insertSnippet = (e: SyntheticEvent, snippet: string) => {
    if (!ref) return;
    if (!ref.current || !note) return;
    const pos = ref.current.selectionStart;
    const value = ref.current.value;
    ref.current.value = [value.slice(0, pos), snippet, value.slice(pos)].join(
      ''
    );
    ref.current.focus();
    const cursorPosition = snippet.length;
    ref.current.selectionStart = pos + cursorPosition;

    setNote({
      ...note,
      note: {
        ...note.note,
        content: ref.current.value,
      },
    });
  };

  return <button onClick={(e) => insertSnippet(e, snippet)}>{label}</button>;
};

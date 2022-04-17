import { EditorContext } from 'context';
import { dApi } from 'lib';
import { FunctionComponent, useContext, MouseEvent } from 'react';

interface IWidget {
  label: string;
  function: (e: MouseEvent<HTMLButtonElement>) => {};
  tooltip: string;
  requiresOpenNote: boolean;
}

export const MenuStrip: FunctionComponent = () => {
  const { note, setNote, notes, setNotes } = useContext(EditorContext);

  const saveNote = async () => {
    if (!note) return;
    const updatedNote = await dApi.updateNote(note);
    const notesCopy = [...notes];
    const index = notesCopy.findIndex(
      (note) => note.note_id == updatedNote.note_id
    );
    if (index === -1) return;
    notesCopy[index] = {
      note_id: updatedNote.note_id,
      title: updatedNote.note.title,
      tags: updatedNote.note.tags,
      allowance: updatedNote.allowance,
    };
    setNotes(notesCopy);
  };

  const refreshNotes = async () => {
    const data = await dApi.getNotes();
    setNotes(data);
  };

  const closeNote = async () => {
    if (!note) return;
    setNote(undefined);
  };

  const widgets: IWidget[] = [
    {
      label: 'R',
      function: refreshNotes,
      tooltip: 'Refresh notes',
      requiresOpenNote: false,
    },
    {
      label: 'S',
      function: saveNote,
      tooltip: 'Save note',
      requiresOpenNote: true,
    },
    {
      label: 'X',
      function: closeNote,
      tooltip: 'Close note',
      requiresOpenNote: true,
    },
  ];

  return (
    <ul>
      {widgets.map((widget, index) => {
        if (!widget.requiresOpenNote || (widget.requiresOpenNote && note))
          return (
            <button
              key={index}
              title={widget.tooltip}
              onClick={widget.function}
            >
              {widget.label}
            </button>
          );
      })}
    </ul>
  );
};

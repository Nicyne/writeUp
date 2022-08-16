import axios from 'axios';
import { useEditor } from 'hooks';
import { FormEvent, SyntheticEvent, useRef, useState } from 'react';
import { useTranslation } from 'react-i18next';
import { INote } from 'types';

export function NewNoteForm() {
  const [title, setTitle] = useState('');
  const [t] = useTranslation();
  const { refs, notes, setNotes, getNote } = useEditor();
  const formRef = useRef<HTMLFormElement>(null);

  const cancel = (e: SyntheticEvent) => {
    e.stopPropagation();
    e.preventDefault();
    refs.newNoteDialog?.current?.close();
  };

  const createNote = async (e: FormEvent) => {
    e.stopPropagation();
    e.preventDefault();
    if (!title) return;

    const res = await axios.post('/api/note', { title, content: '', tags: [] });
    if (!res.data.success) {
      console.error(res.data.message);
      return;
    }

    const note: INote = res.data.content;
    setNotes([
      ...notes,
      {
        note_id: note.note_id,
        allowance: note.allowance,
        title: note.note.title,
        tags: note.note.tags,
      },
    ]);

    formRef.current?.reset();
    setTitle('');
    getNote(note.note_id);
    refs.newNoteDialog?.current?.close();
  };

  const click = (e: SyntheticEvent) => {
    e.stopPropagation();
    const el = e.target as HTMLElement;
    if (el.id === 'newNoteDialog') {
      cancel(e);
    }
  };

  return (
    <dialog onClick={click} ref={refs.newNoteDialog} id="newNoteDialog">
      <article className="form">
        <header>
          <h1>{t('notes.newNote')}</h1>
        </header>
        <form ref={formRef} onSubmit={createNote}>
          <label htmlFor="title">
            {t('notes.title')}
            <input
              type="text"
              value={title}
              onChange={(e) => setTitle(e.target.value)}
            />
          </label>
          <span className="flex">
            <button type="submit">{t('notes.create')}</button>
            <button className="secondary" onClick={cancel}>
              {t('notes.cancel')}
            </button>
          </span>
        </form>
      </article>
    </dialog>
  );
}

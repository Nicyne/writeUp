import axios from 'axios';
import { useEditor } from 'hooks';
import { FormEvent, SyntheticEvent, useEffect, useRef, useState } from 'react';
import { useTranslation } from 'react-i18next';
import { INote } from 'types';
import styles from 'styles/components/editor/editorPane.module.scss';

export function EditorPane() {
  const [shadowNote, setShadowNote] = useState<INote>();
  const { currentNote, refs, notes, setNotes, setNote } = useEditor();
  const [t] = useTranslation();
  const formRef = useRef<HTMLFormElement>(null);

  const saveNote = async (e: FormEvent) => {
    e.stopPropagation();
    e.preventDefault();

    if (!currentNote || !shadowNote) return;

    shadowNote.note.tags = shadowNote.note.tags.filter((tag) => tag !== '');
    const res = await axios.put(
      '/api/note/' + currentNote.note_id,
      shadowNote.note
    );

    if (!res.data.success) {
      console.error(res.data.message);
      return;
    }

    const responseNote: INote = res.data.content;

    if (!responseNote) {
      console.error('No note was returned');
      return;
    }

    setNotes(
      notes.map((note) => {
        if (note.note_id === responseNote.note_id) {
          return {
            note_id: responseNote.note_id,
            allowance: responseNote.allowance,
            title: responseNote.note.title,
            tags: responseNote.note.tags.filter((tag) => tag !== ''),
          };
        }

        return note;
      })
    );
    setNote(responseNote);

    refs.bodyEditor?.current?.close();
    formRef.current?.reset();
  };

  const cancel = (e: SyntheticEvent) => {
    e.stopPropagation();
    e.preventDefault();

    setShadowNote(currentNote);

    refs.bodyEditor?.current?.close();
    formRef.current?.reset();
  };

  const click = (e: SyntheticEvent) => {
    e.stopPropagation();
    const el = e.target as HTMLElement;
    if (el.id === 'editorDialog') {
      cancel(e);
    }
  };

  useEffect(() => {
    setShadowNote(currentNote);
  }, [currentNote]);

  if (!currentNote || !shadowNote) {
    return <></>;
  }

  return (
    <dialog onClick={click} id="editorDialog" ref={refs.bodyEditor}>
      <article className={`${styles['editorForm']} form`}>
        <header>
          <h1>{t('notes.editNote')}</h1>
        </header>
        <form ref={formRef} onSubmit={saveNote}>
          <label htmlFor="title">
            {t('notes.title')}
            <input
              type="text"
              value={shadowNote.note.title}
              name={t('notes.title')}
              id="title"
              onChange={(e) =>
                setShadowNote({
                  ...shadowNote,
                  note: { ...shadowNote.note, title: e.target.value },
                })
              }
            />
          </label>

          <label htmlFor="content">
            {t('notes.content')}
            <textarea
              autoFocus
              value={shadowNote.note.content}
              name={t('notes.content')}
              id="content"
              onChange={(e) =>
                setShadowNote({
                  ...shadowNote,
                  note: { ...shadowNote.note, content: e.target.value },
                })
              }
            />
          </label>

          <label htmlFor="tags">
            {t('notes.tags')}
            <input
              type="text"
              value={shadowNote.note.tags.join(';')}
              name={t('notes.tags')}
              id="tags"
              onChange={(e) =>
                setShadowNote({
                  ...shadowNote,
                  note: {
                    ...shadowNote.note,
                    tags: e.target.value.split(';'),
                  },
                })
              }
            />
          </label>

          <span className="flex">
            <button type="submit">{t('notes.save')}</button>
            <button onClick={cancel} className="secondary">
              {t('notes.cancel')}
            </button>
          </span>
        </form>
      </article>
    </dialog>
  );
}

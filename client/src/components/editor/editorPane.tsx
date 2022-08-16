import axios from 'axios';
import { useEditor } from 'hooks';
import { FormEvent, SyntheticEvent, useRef } from 'react';
import { useTranslation } from 'react-i18next';
import styles from 'styles/components/editor/editorPane.module.scss';

export function EditorPane() {
  const { currentNote, refs, setNote, getNotes } = useEditor();
  const [t] = useTranslation();
  const formRef = useRef<HTMLFormElement>(null);

  const saveNote = async (e: FormEvent) => {
    e.stopPropagation();
    e.preventDefault();

    if (!currentNote) return;

    const res = await axios.put(
      '/api/note/' + currentNote.note_id,
      currentNote.note
    );

    if (!res.data.success) {
      console.error(res.data.message);
      return;
    }
    getNotes();
    refs.bodyEditor?.current?.close();
    formRef.current?.reset();
  };

  const cancel = (e: SyntheticEvent) => {
    console.log(e);
    e.stopPropagation();
    e.preventDefault();
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

  if (!currentNote) {
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
              value={currentNote.note.title}
              name={t('notes.title')}
              id="title"
              onChange={(e) =>
                setNote({
                  ...currentNote,
                  note: { ...currentNote.note, title: e.target.value },
                })
              }
            />
          </label>

          <label htmlFor="content">
            {t('notes.content')}
            <textarea
              value={currentNote.note.content}
              name={t('notes.content')}
              id="content"
              onChange={(e) =>
                setNote({
                  ...currentNote,
                  note: { ...currentNote.note, content: e.target.value },
                })
              }
            />
          </label>

          <label htmlFor="tags">
            {t('notes.tags')}
            <input
              type="text"
              value={currentNote.note.tags.join(';')}
              name={t('notes.tags')}
              id="tags"
              onChange={(e) =>
                setNote({
                  ...currentNote,
                  note: {
                    ...currentNote.note,
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

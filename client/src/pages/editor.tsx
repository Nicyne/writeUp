import { EditorPane, NewNoteForm, Preview, Sidebar } from 'components';
import { useEditor } from 'hooks';
import { useEffect } from 'react';
import { useTranslation } from 'react-i18next';
import styles from 'styles/components/editor/editorPage.module.scss';

export function Editor() {
  const { currentNote, getNotes, refs } = useEditor();
  const [t] = useTranslation();

  useEffect(() => {
    getNotes();
  }, []);

  const edit = () => {
    if (!currentNote) return;

    if (!refs.bodyEditor?.current) return;

    refs.bodyEditor?.current?.showModal();
  };

  return (
    <div className={styles['editor-page']}>
      <Sidebar />
      <Preview />
      <EditorPane />
      <NewNoteForm />

      {currentNote && (
        <button onClick={edit} className={styles['editButton']} title="Edit">
          {t('notes.edit')}
        </button>
      )}
    </div>
  );
}

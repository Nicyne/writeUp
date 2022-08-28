import { EditorPane, NewNoteForm, Preview, Sidebar } from 'components';
import { useEditor, useMountEffect } from 'hooks';
import { useTranslation } from 'react-i18next';
import styles from 'styles/components/editor/editorPage.module.scss';

export function Editor() {
  const { currentNote, getNotes, refs } = useEditor();
  const [t] = useTranslation();

  useMountEffect(() => {
    getNotes();
  });

  const edit = () => {
    if (!currentNote) return;

    if (!refs.bodyEditor?.current) return;

    refs.bodyEditor?.current?.showModal();
  };

  return (
    <div className={styles['editor-page']}>
      <Sidebar />
      <Preview />
      <NewNoteForm />
      <EditorPane />

      {currentNote && (
        <button onClick={edit} className={styles['editButton']} title="Edit">
          {t('notes.edit')}
        </button>
      )}
    </div>
  );
}

import { EditorPane, NewNoteForm, Preview, Sidebar } from 'components';
import { useEditor, useMnemonic, useMountEffect } from 'hooks';
import { useTranslation } from 'react-i18next';
import { Edit } from 'react-feather';
import styles from 'styles/components/editor/editorPage.module.scss';

export function Editor() {
  const { currentNote, getNotes, refs } = useEditor();
  const [t] = useTranslation();

  useMountEffect(() => {
    getNotes();
  });

  useMnemonic(['control', 'e'], () => {
    if (refs.bodyEditor?.current?.open) return;
    if (refs.newNoteDialog?.current?.open) return;
    edit();
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
        <button
          onClick={edit}
          className={`${styles['editButton']} svgButton round`}
          title={t('notes.edit')}
        >
          <Edit />
        </button>
      )}
    </div>
  );
}

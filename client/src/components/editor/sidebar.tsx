import { useEditor, useMnemonic } from 'hooks';
import { useTranslation } from 'react-i18next';
import { SidebarElement } from './sidebarElement';
import { Plus } from 'react-feather';
import styles from 'styles/components/editor/sidebar.module.scss';

export function Sidebar() {
  const { notes, refs } = useEditor();
  const [t] = useTranslation();

  useMnemonic(['control', 'n'], () => {
    if (refs.newNoteDialog?.current?.open) return;
    showNewNotesDialog();
  });

  const showNewNotesDialog = () => {
    if (!refs.newNoteDialog?.current) return;
    if (refs.bodyEditor?.current?.open) return;
    refs.newNoteDialog.current.showModal();
  };

  return (
    <article className={styles['sidebar']}>
      <ul className={styles['noteList']}>
        <li className={styles['addButton']}>
          <button
            onClick={showNewNotesDialog}
            title={t('notes.new')}
            className="svgButton round"
          >
            {<Plus />}
          </button>
        </li>
        {notes.map((note) => (
          <SidebarElement note={note} key={note.note_id} />
        ))}
      </ul>
    </article>
  );
}

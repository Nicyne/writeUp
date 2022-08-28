import { useEditor, useKeyboardShortcut } from 'hooks';
import { useTranslation } from 'react-i18next';
import styles from 'styles/components/editor/sidebar.module.scss';
import { SidebarElement } from './sidebarElement';

export function Sidebar() {
  const { notes, refs } = useEditor();
  const [t] = useTranslation();

  useKeyboardShortcut(['control', 'n'], () => {
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
        <li>
          <button onClick={showNewNotesDialog}>{t('notes.new')}</button>
        </li>
        {notes.map((note) => (
          <SidebarElement note={note} key={note.note_id} />
        ))}
      </ul>
    </article>
  );
}

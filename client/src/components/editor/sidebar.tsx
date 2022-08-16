import { useEditor } from 'hooks';
import { useTranslation } from 'react-i18next';
import styles from 'styles/components/editor/sidebar.module.scss';
import { SidebarElement } from './sidebarElement';

export function Sidebar() {
  const { notes, refs } = useEditor();
  const [t] = useTranslation();

  return (
    <article className={styles['sidebar']}>
      <ul className={styles['noteList']}>
        <li>
          <button onClick={() => refs.newNoteDialog?.current?.showModal()}>
            {t('notes.new')}
          </button>
        </li>
        {notes.map((note) => (
          <SidebarElement note={note} key={note.note_id} />
        ))}
      </ul>
    </article>
  );
}

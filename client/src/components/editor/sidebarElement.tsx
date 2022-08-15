import { useEditor } from 'hooks/useEditor';
import { SyntheticEvent, useRef } from 'react';
import { useTranslation } from 'react-i18next';
import { INoteShallow } from 'types';
import { ConfirmDeletionDialog } from './confirmDeletionDialog';
import styles from 'styles/components/editor/sidebarElement.module.scss';

interface IProps {
  note: INoteShallow;
}

export function SidebarElement(props: IProps) {
  const { note } = props;
  const { currentNote, getNote, setNote } = useEditor();
  const deletionDialog = useRef<HTMLDialogElement>(null);
  const [t] = useTranslation();

  const selectNote = async (id: string) => {
    if (id === currentNote?.note_id) {
      // check for changes later
      setNote(undefined);
      return;
    }

    await getNote(id);
  };

  const deleteNote = async (e: SyntheticEvent, id: string) => {
    e.stopPropagation();
    if (!deletionDialog.current) return;

    deletionDialog.current.showModal();
  };

  return (
    <li
      key={note.note_id}
      className={`${styles['note']} ${
        note.note_id === currentNote?.note_id ? styles['active'] : ''
      }`}
      onClick={() => selectNote(note.note_id)}
    >
      <span>
        {note.title}
        <button
          className={styles['button']}
          title={t('notes.deleteTooltip')}
          onClick={(e) => deleteNote(e, note.note_id)}
        >
          &#x2715;
        </button>
      </span>
      {note.tags.length !== 0 ? (
        <span>
          <ul className={styles['tags']}>
            {note.tags.map((tag) => (
              <li key={tag}>{tag}</li>
            ))}
          </ul>
        </span>
      ) : (
        <></>
      )}
      <ConfirmDeletionDialog ref={deletionDialog} id={note.note_id} />
    </li>
  );
}

import { useEditor, useKeys, useLocalStorage, useMountEffect } from 'hooks';
import { SyntheticEvent, useRef } from 'react';
import { useTranslation } from 'react-i18next';
import { INoteShallow } from 'types';
import { ConfirmDeletionDialog } from './confirmDeletionDialog';
import { Trash } from 'react-feather';
import styles from 'styles/components/editor/sidebarElement.module.scss';

interface IProps {
  note: INoteShallow;
}

export function SidebarElement(props: IProps) {
  const { note } = props;
  const { isKeyDown } = useKeys();
  const { currentNote, getNote, setNote, deleteNote } = useEditor();
  const [lastNote, setLastNote] = useLocalStorage<string>('lastNote', '');
  const deletionDialog = useRef<HTMLDialogElement>(null);
  const [t] = useTranslation();

  const selectNote = async (id: string) => {
    if (id === currentNote?.note_id) {
      // check for changes later
      setNote(undefined);
      return;
    }
    setLastNote(id);

    await getNote(id);
  };

  const deleteSelectedNote = async (e: SyntheticEvent, id: string) => {
    e.stopPropagation();
    if (!deletionDialog.current) return;

    if (!isKeyDown('shift')) {
      deletionDialog.current.showModal();
      return;
    }

    if (currentNote && currentNote.note_id === id) {
      setNote(undefined);
    }

    const success = await deleteNote(id);

    if (!success) return;
  };

  useMountEffect(() => {
    if (lastNote === note.note_id) {
      selectNote(lastNote);
    }
  });

  return (
    <>
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
            onClick={(e) => deleteSelectedNote(e, note.note_id)}
          >
            <Trash />
          </button>
        </span>
        {note.tags.length !== 0 ? (
          <span>
            <ul className={styles['tags']}>
              {note.tags.map((tag, index) => (
                <li key={tag + index}>{tag}</li>
              ))}
            </ul>
          </span>
        ) : (
          <></>
        )}
      </li>
      <ConfirmDeletionDialog ref={deletionDialog} id={note.note_id} />
    </>
  );
}

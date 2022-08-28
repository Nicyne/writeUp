import { useEditor } from 'hooks';
import { FormEvent, forwardRef, RefObject, SyntheticEvent } from 'react';
import { useTranslation } from 'react-i18next';

interface IDialogProps {
  id: string;
}

export const ConfirmDeletionDialog = forwardRef((props: IDialogProps, ref) => {
  const { deleteNote } = useEditor();
  const [t] = useTranslation();
  const dialogRef = ref as RefObject<HTMLDialogElement>;

  const cancel = (e: SyntheticEvent) => {
    e.stopPropagation();
    e.preventDefault();
    if (!dialogRef.current) return;

    dialogRef.current.close();
  };

  const finallyDeleteNote = async (e: FormEvent) => {
    e.stopPropagation();
    e.preventDefault();
    if (!dialogRef.current) return;

    const success = await deleteNote(props.id);

    if (!success) return;

    dialogRef.current.close();
  };

  const click = (e: SyntheticEvent) => {
    e.stopPropagation();
    const el = e.target as HTMLElement;
    if (el.id === 'deletionDialog') {
      cancel(e);
    }
  };

  return (
    <dialog onClick={click} ref={dialogRef} id="deletionDialog">
      <article className="form">
        <header>
          <h1>{t('notes.deleteTitle')}</h1>
          <p>{t('notes.deleteWarning')}</p>
          <small>{t('notes.deleteSkipTip', { key: 'shift' })}</small>
        </header>
        <form onSubmit={finallyDeleteNote}>
          <span className="flex-end">
            <button type="submit" className="danger">
              {t('notes.delete')}
            </button>
            <button onClick={cancel} className="secondary">
              {t('notes.cancel')}
            </button>
          </span>
        </form>
      </article>
    </dialog>
  );
});

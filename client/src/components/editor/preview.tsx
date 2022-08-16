import { useEditor } from 'hooks';
import { CodeBlock } from 'components';
import Markdown from 'react-markdown';
import remarkGfm from 'remark-gfm';
import remarkEmoji from 'remark-emoji';
import styles from 'styles/components/editor/preview.module.scss';
import { useTranslation } from 'react-i18next';

export function Preview() {
  const { currentNote } = useEditor();
  const [t] = useTranslation();

  return (
    <div className={styles['preview']}>
      {currentNote ? (
        <>
          {currentNote.note.content ? (
            <Markdown
              remarkPlugins={[remarkGfm, remarkEmoji]}
              components={{
                code({ node, inline, className, children, ...props }) {
                  const match = /language-(\w+)/.exec(className || '');
                  return !inline && match ? (
                    <CodeBlock
                      value={String(children).replace(/\n$/, '')}
                      language={match[1]}
                    />
                  ) : (
                    <code className={className} {...props}>
                      {children}
                    </code>
                  );
                },
              }}
            >
              {currentNote?.note.content ?? ''}
            </Markdown>
          ) : (
            <div className="center">
              <h2>{t('notes.noNoteBody')}</h2>
            </div>
          )}
        </>
      ) : (
        <div className="center">
          <h2>{t('notes.selectNote')}</h2>
        </div>
      )}
    </div>
  );
}

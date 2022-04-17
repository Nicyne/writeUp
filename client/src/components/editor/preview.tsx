import { FunctionComponent, useContext } from 'react';

import ReactMarkdown from 'react-markdown';
import remarkGfm from 'remark-gfm';
import remarkEmoji from 'remark-emoji';
import { CodeBlock } from 'components';
import { EditorContext } from 'context';

export const Preview: FunctionComponent = () => {
  const { note } = useContext(EditorContext);

  if (!note) return <></>;

  return (
    <ReactMarkdown
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
      {note?.note.content ?? ''}
    </ReactMarkdown>
  );
};

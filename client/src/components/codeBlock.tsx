import { useEffect, useState } from 'react';
import { PrismAsync as SyntaxHighlighter } from 'react-syntax-highlighter';
import {
  vscDarkPlus,
  vs,
} from 'react-syntax-highlighter/dist/cjs/styles/prism';
import styles from 'styles/components/codeblock.module.scss';

interface IProps {
  value: string;
  language: string;
}

export function CodeBlock(props: IProps) {
  const [prefersLight, setPrefersLight] = useState(false);

  useEffect(() => {
    if (typeof window === 'undefined') return;

    const colorSchemePreference = window.matchMedia(
      '(prefers-color-scheme: light)'
    );
    setPrefersLight(colorSchemePreference.matches);
  }, []);

  return (
    <code className={styles['codeBlock']}>
      <SyntaxHighlighter
        language={props.language}
        style={prefersLight ? vs : vscDarkPlus}
      >
        {props.value}
      </SyntaxHighlighter>
    </code>
  );
}

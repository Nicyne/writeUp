import type { NextPage } from 'next';
import Head from 'next/head';
import { FunctionComponent, useState } from 'react';
import { PrismAsync as SyntaxHighlighter } from 'react-syntax-highlighter';
import {vscDarkPlus} from 'react-syntax-highlighter/dist/cjs/styles/prism';
import ReactMarkdown from 'react-markdown';
import remarkGfm from 'remark-gfm';
import json from '../../posts.json';

const Home: NextPage = () => {
  const [markdown, setMarkdown] = useState<string>(json[0].body);

  return (
    <>
      <Head>
        <title>writeUp</title>
        <meta name="description" content="writeUp" />
        <link rel="icon" href="/favicon.ico" />
      </Head>

      <div className="grid">
        <textarea name="input" id="input" value={markdown} onChange={({target}) => setMarkdown(target.value)} />
        <div>
          <ReactMarkdown remarkPlugins={[remarkGfm]} components={
            { code({node, inline, className, children, ...props}) {
            const match = /language-(\w+)/.exec(className || '');
            return !inline && match ? (
              <Codeblock value={String(children).replace(/\n$/, '')} language={match[1]} />
            ) : (
              <code className={className} {...props}>
                {children}
              </code>
            );
          } }
          }>
            {markdown ?? ''}
          </ReactMarkdown>
        </div>
      </div>
    </>
  );
};

const Codeblock: FunctionComponent<{value: string, language: string}> = ({value, language}) => {
  return (
    <SyntaxHighlighter language={language} style={vscDarkPlus}>
      {value}
    </SyntaxHighlighter>
  );
};

export default Home;

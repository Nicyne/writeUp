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
  const [posts, setPosts] = useState(json);
  const [currentPost, setCurrentPost] = useState<number>(-1);

  const savePost = () => {
    let index = posts.findIndex((p) => p.id == currentPost);
    if (index == -1) return;
    posts[index].body = markdown;
    setPosts(posts);
  };

  const setPost = (post: any) => {
    setCurrentPost(post.id); 
    setMarkdown(post.body); 
  };

  const deletePost = () => {
    let prevPost = posts.findIndex((p) => p.id == currentPost) - 1;
    setPosts(posts.filter((p) => p.id != currentPost));
    if (posts.length == 0) return;
    let index = (prevPost != -1) ? prevPost : 0;
    setPost(posts[index]);
  };

  return (
    <main className='app'>
      <Head>
        <title>writeUp</title>
        <meta name="description" content="writeUp" />
        <link rel="icon" href="/favicon.ico" />
      </Head>

      <ul className='list'>
        <button onClick={() => savePost() }>Save</button>
        <button onClick={() => {
          let npost = { id: posts.length+1, title: `Post #${posts.length}`, body: '', tags: [] };
          setPosts([...posts, npost]);
          setPost(npost);
        }}>New</button>
        <button onClick={() => deletePost()}>Delete</button>
        {
          posts.map((post) => (
            <li key={post.id} onClick={() => setPost(post) }>{post.title}</li>
          ))
        }
      </ul>

      <div className="grid">
        <textarea name="input" spellCheck="false" id="input" value={markdown} onChange={({target}) => setMarkdown(target.value)} />
        <div className='preview'>
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
    </main>
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

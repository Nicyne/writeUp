import styles from 'styles/pages/landing.module.scss';
import type { NextPage } from 'next';
import Head from 'next/head';
import Link from 'next/link';

const Home: NextPage = () => {
  return (
    <>
      <Head>
        <title>Home | writeUp</title>
      </Head>

      <section aria-labelledby="landing-title" className={styles.landing}>
        <div className={`container ${styles.flCenter}`}>
          <h1 id="landing-title">Open Source Note Taking</h1>
          <div className={styles.text}>
            <p>
              Take your markdown notes <u>everywhere</u>!
            </p>
          </div>
          <Link href="/app">Learn More</Link>
        </div>
      </section>

      <section aria-labelledby="features-title" className={styles.main}>
        <div className="container">
          <h2 id="features-title">Features</h2>

          <div className={styles.cards}>
            <article className={`${styles.card} ${styles.two}`}>
              <h3>🌍 Share your notes with your friends.</h3>
              <p>
                One of the primary goals of writeUp was to allow users to write
                notes in markdown and share those notes with their friends. With
                writeUp you can not only share your notes with everyone you
                want, you can also make use of our public api to write your own
                frontend if you dislike ours or miss some features.
              </p>
            </article>

            <article className={styles.card}>
              <h3>🔧 The power of Markdown</h3>
              <p>
                Use markdown to give your note the extra touch. With elements
                like hyperlinks, todo lists, headers or code blocks you can
                write down literally anything.
              </p>
            </article>

            <article className={styles.card}>
              <h3>🔮 Almost like magic</h3>
              <p>
                Honestly, it is magic. We even got auto saves with hash
                comparison.
              </p>
            </article>
          </div>
        </div>
      </section>

      <section className="hero">
        <div className="container">
          <h3>Alpha Software - There should be a hero section here</h3>
        </div>
      </section>

      <section aria-labelledby="technology-title" className={styles.main}>
        <div className="container">
          <h2 id="technology-title">Modern Technology</h2>

          <div className={styles.cards}>
            <article className={`${styles.card} ${styles.two}  ${styles.end}`}>
              <h3>🌍 Open source note taking</h3>
              <p>
                Lorem ipsum dolor sit amet, consectetur adipisicing elit. Dicta,
                veritatis. Lorem ipsum dolor sit amet consectetur adipisicing
                elit. Sunt doloremque labore aperiam sequi, obcaecati sed
                exercitationem asperiores voluptatum quo nemo.
              </p>
              <br />
              <p>
                Lorem ipsum dolor sit amet consectetur adipisicing elit.
                Quibusdam quasi qui iusto saepe, iste veniam dolores ut mollitia
                voluptatem deleniti.
              </p>
            </article>

            <article className={styles.card}>
              <h3>⭐ React user interface</h3>
              <p>
                Lorem ipsum dolor sit amet consectetur adipisicing elit.
                Expedita, natus?
              </p>
            </article>

            <article className={styles.card}>
              <h3>🚀 Powered by Rust</h3>
              <p>
                Lightning fast backend thanks to Rust and the powerful actix web
                framework. Lorem ipsum, dolor sit amet consectetur adipisicing
                elit. Suscipit, aliquam praesentium. Veniam expedita fugit
                officiis ea doloribus eaque quae provident.
              </p>
            </article>
          </div>
        </div>
      </section>
    </>
  );
};

export default Home;

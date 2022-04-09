import styles from 'styles/pages/landing.module.scss';
import type { NextPage } from 'next';
import Head from 'next/head';
import Link from 'next/link';

const Home: NextPage = () => {
  return (
    <>
      <Head>
        <title>WriteUp</title>
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

      <section className={styles.main}>
        <div className="container">
          <h2>Modern Technology</h2>

          <div className={styles.cards}>
            <article className={`${styles.card} ${styles.two}`}>
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

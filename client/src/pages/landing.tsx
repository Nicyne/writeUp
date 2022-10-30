import { Link } from 'react-router-dom';
import { AlertTriangle } from 'react-feather';
import styles from 'styles/components/landing.module.scss';
import { useTranslation } from 'react-i18next';
import { Helmet } from 'react-helmet-async';
import { toPageTitle } from 'utils';

export function Landing() {
  const [t] = useTranslation();

  return (
    <>
      <Helmet>
        <title>{toPageTitle(t('common.home'))}</title>
      </Helmet>
      <section
        className={styles['landingHero']}
        aria-labelledby="landing-title"
        id="landing"
      >
        <header>
          <div className="container">
            <div className={styles['landingHeroText']}>
              <h1 id="landing-title">
                {t('landing.welcomeTo')}{' '}
                <span className="accent-text">{t('common.appName')}!</span>
              </h1>
              <p>{t('landing.subtitle')}</p>
            </div>
            <div>
              <span className="flex-center">
                <Link to={'/signup'} role="button">
                  {t('landing.tryNow')}*
                </Link>
              </span>
              <p className={styles['betaDisclaimer']}>
                *{t('landing.betaKeyNotice')}
              </p>
            </div>
          </div>
        </header>
      </section>
      <article className={styles['landingPage']}>
        <header>
          <div className="container">{t('landing.whyWriteUp')}</div>
        </header>
        <section id="support-for-markdown" aria-labelledby="markdown-title">
          <div className="container">
            <article className={styles['gridBox']}>
              <div className={styles['card']}>
                <header>
                  <h2 id="markdown-title">{t('landing.markdown.title')}</h2>
                </header>
                <p>{t('landing.markdown.body')}</p>
              </div>
              <div className={styles['image']}>
                <img
                  src="https://images.unsplash.com/photo-1498050108023-c5249f4df085?ixlib=rb-4.0.3&ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&auto=format&fit=crop&w=1744&q=80"
                  alt="unsplash"
                />
              </div>
            </article>
          </div>
        </section>
        <section id="easy-organization" aria-labelledby="organization-title">
          <div className="container">
            <article className={styles['gridBox']}>
              <div className={styles['card']}>
                <header>
                  <h2 id="organization-title">
                    {t('landing.organization.title')}
                  </h2>
                </header>
                <p>{t('landing.organization.body')}</p>
                <p>{t('landing.organization.note')}</p>
              </div>
              <div className={styles['image']}>
                <img
                  src="https://images.unsplash.com/photo-1498050108023-c5249f4df085?ixlib=rb-4.0.3&ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&auto=format&fit=crop&w=1744&q=80"
                  alt="unsplash"
                />
              </div>
            </article>
          </div>
        </section>
        <section id="simple-sharing" aria-labelledby="simple-sharing-title">
          <div className="container">
            <article className={styles['gridBox']}>
              <div className={styles['card']}>
                <header>
                  <h2 id="simple-sharing-title">
                    {t('landing.sharing.title')}
                    <span className={styles['wipBanner']}>
                      <AlertTriangle /> {t('common.inDevelopment')}*
                    </span>
                  </h2>
                </header>
                <p>{t('landing.sharing.body')}</p>
                <p>{t('landing.sharing.note')}</p>
                <p className={styles['footnote']}>
                  *{t('landing.sharing.footnote')}
                </p>
              </div>
              <div className={styles['image']}>
                <img
                  src="https://images.unsplash.com/photo-1498050108023-c5249f4df085?ixlib=rb-4.0.3&ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&auto=format&fit=crop&w=1744&q=80"
                  alt="unsplash"
                />
              </div>
            </article>
          </div>
        </section>
        <section id="fast-and-reliable" aria-labelledby="far-title">
          <div className="container">
            <article className={styles['gridBox']}>
              <div className={styles['card']}>
                <header>
                  <h2 id="far-title">{t('landing.far.title')}</h2>
                </header>
                <p>{t('landing.far.body')}</p>
              </div>
              <div className={styles['image']}>
                <img
                  src="https://images.unsplash.com/photo-1498050108023-c5249f4df085?ixlib=rb-4.0.3&ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&auto=format&fit=crop&w=1744&q=80"
                  alt="unsplash"
                />
              </div>
            </article>
          </div>
        </section>
      </article>
    </>
  );
}

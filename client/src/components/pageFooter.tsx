import { useState } from 'react';
import { useTranslation } from 'react-i18next';
import { Link } from 'react-router-dom';
import { ExternalLink } from 'react-feather';
import styles from 'styles/components/pageFooter.module.scss';

export function PageFooter() {
  const [t, i18n] = useTranslation();
  const [lang, setLang] = useState(i18n.language);

  const changeLang = (lang: string) => {
    i18n.changeLanguage(lang);
    setLang(lang);
  };

  return (
    <footer className={styles['footer']}>
      <nav className={styles['nav']}>
        <ul className={styles['list']}>
          <li>
            <select
              name="language"
              id="language"
              defaultValue={lang}
              onChange={(e) => changeLang(e.target.value)}
            >
              <option value="de">🇩🇪 Deutsch</option>
              <option value="en">🇺🇸 English</option>
            </select>
          </li>
        </ul>
        <ul className={`${styles['list']} center`}>
          <li>
            {t('common.appName')} &copy; {new Date().getFullYear()}
          </li>
        </ul>
        <ul className={styles['list']}>
          <li>
            <a
              href="https://github.com/nicyne/writeup"
              target="_blank"
              rel="noreferrer"
            >
              GitHub
              <ExternalLink />
            </a>
          </li>
          <li>
            <Link to="/about">{t('nav.about')}</Link>
          </li>
        </ul>
      </nav>
    </footer>
  );
}

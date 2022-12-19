import { initReactI18next } from 'react-i18next';
import i18n from 'i18next';
import detector from 'i18next-browser-languagedetector';
import backend from 'i18next-http-backend';

export const fallbackLocale = 'en';

/**
 * Note to future-me:
 *
 * Should you find yourself stuck read this:
 *
 * setup: https://www.i18next.com/overview/typescript
 * namespaces: https://www.i18next.com/principles/namespaces?q=namespace
 * lazy loading: https://www.i18next.com/how-to/add-or-load-translations#load-using-a-backend-plugin
 */

i18n
  .use(detector)
  .use(backend)
  .use(initReactI18next)
  .init({
    ns: ['common', 'errors'],
    defaultNS: 'common',
    backend: {
      // This settings is important for lazy loading the translation files
      // for all available options read the backend's repository readme file
      loadPath: './locales/{{lng}}/{{ns}}.json',
    },
    supportedLngs: ['de', 'en'],
    fallbackLng: fallbackLocale,
    interpolation: { escapeValue: false },
  });

export default i18n;

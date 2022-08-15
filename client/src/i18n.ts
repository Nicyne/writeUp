import i18n from 'i18next';
import detector from 'i18next-browser-languagedetector';
import backend from 'i18next-http-backend';
import { initReactI18next } from 'react-i18next';

import de from 'locales/de/common.json';
import en from 'locales/en/common.json';

const resources = {
  en: {
    translation: en,
  },
  de: {
    translation: de,
  },
};

i18n
  .use(detector)
  .use(backend)
  .use(initReactI18next)
  .init({
    resources,
    fallbackLng: 'de',
    interpolation: { escapeValue: false },
  });

export default i18n;

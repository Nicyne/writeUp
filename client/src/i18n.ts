import i18n from 'i18next';
import detector from 'i18next-browser-languagedetector';
import backend from 'i18next-http-backend';
import { initReactI18next } from 'react-i18next';

export const fallbackLocale = 'en';

const resources = {
  en: {
    translation: require('locales/en/common.json'),
  },
  de: {
    translation: require('locales/de/common.json'),
  },
};

i18n
  .use(detector)
  .use(backend)
  .use(initReactI18next)
  .init({
    resources,
    supportedLngs: ['de', 'en'],
    fallbackLng: fallbackLocale,
    interpolation: { escapeValue: false },
  });

export default i18n;

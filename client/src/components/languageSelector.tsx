import { useState } from 'react';
import { useTranslation } from 'react-i18next';

export function LanguageSelector() {
  const [t, i18n] = useTranslation();
  const [lang, setLang] = useState(i18n.language);

  const changeLocale = (locale: string) => {
    i18n.changeLanguage(locale);
    setLang(locale);
  };

  return (
    <select
      name={t('language')}
      id="language"
      defaultValue={lang}
      onChange={(e) => changeLocale(e.target.value)}
    >
      <option value="de">Deutsch</option>
      <option value="en">English</option>
    </select>
  );
}

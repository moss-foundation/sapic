import i18next from "i18next";
import { initReactI18next } from "react-i18next";

import I18nTauriBackend from "../lib/backend/nls";

export const initializeI18n = async (language?: string) => {
  const lng = language || "en";

  await i18next
    .use(I18nTauriBackend)
    .use(initReactI18next)
    .init({
      lng,
      fallbackLng: "en",
      ns: ["ns1", "ns2"],
      defaultNS: "ns1",
      interpolation: {
        escapeValue: false,
      },
      react: {
        useSuspense: true,
      },
    });
};

export const changeLanguage = async (language: string) => {
  await i18next.changeLanguage(language);
  await i18next.reloadResources();
};

initializeI18n();

export default i18next;

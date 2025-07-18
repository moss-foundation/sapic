import i18next from "i18next";
import { initReactI18next } from "react-i18next";

import I18nTauriBackend from "../lib/backend/nls";

// Track initialization to prevent multiple inits
let isInitialized = false;

export const initializeI18n = async (language?: string) => {
  if (isInitialized) {
    if (language && language !== i18next.language) {
      await i18next.changeLanguage(language);
    }
    return;
  }

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
        useSuspense: false,
      },
      initImmediate: true,
      load: "languageOnly",
      // Backend cache settings
      backend: {
        loadPath: function (_lngs: string[], _namespaces: string[]) {
          // This won't be used since we have custom backend, but good to be explicit
          return "";
        },
      },
    });

  isInitialized = true;
};

export const changeLanguage = async (language: string) => {
  await i18next.changeLanguage(language);
};

initializeI18n();

export default i18next;

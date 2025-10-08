import i18next from "i18next";
import { initReactI18next } from "react-i18next";

import I18nTauriBackend from "../lib/backend/nls";

export const initializeI18n = async (languageCode: string) => {
  if (i18next.isInitialized) return;

  try {
    await i18next
      .use(I18nTauriBackend)
      .use(initReactI18next)
      .init({
        lng: languageCode,
        fallbackLng: "en",
        ns: ["main", "bootstrap"],
        defaultNS: "main",
        interpolation: {
          escapeValue: false,
        },
        react: {
          useSuspense: true,
        },
        initImmediate: true,
        load: "languageOnly",
      });
  } catch (error) {
    console.error("Failed to initialize i18n:", error);
  }
};

export default i18next;

import i18next from "i18next";
import { initReactI18next } from "react-i18next";

import I18nTauriBackend from "../lib/backend/nls";

export const initializeI18n = async (languageCode: string) => {
  await i18next
    .use(I18nTauriBackend)
    .use(initReactI18next)
    .init({
      lng: languageCode,
      fallbackLng: "en",
      ns: ["ns1", "ns2"],
      defaultNS: "ns1",
      interpolation: {
        escapeValue: false,
      },
      react: {
        useSuspense: true,
      },
      initImmediate: true,
      load: "languageOnly",
    });
};

initializeI18n("en");

export default i18next;

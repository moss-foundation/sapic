import i18next from "i18next";

import { changeLanguage } from "@/app/i18n";
import { clearTranslationCache } from "@/lib/backend/nls";
import { LocaleInfo } from "@repo/moss-app";

export const applyLanguagePack = async (languagePack: LocaleInfo) => {
  try {
    const currentLanguage = i18next.language;
    const newLanguage = languagePack.code;

    // Only change language if it's actually different
    if (currentLanguage !== newLanguage) {
      // Clear cache for old language to free memory (optional optimization)
      if (currentLanguage && currentLanguage !== newLanguage) {
        clearTranslationCache(currentLanguage);
      }
      await changeLanguage(newLanguage);
    }
  } catch (error) {
    console.error("Failed to apply language pack:", error);
  }
};

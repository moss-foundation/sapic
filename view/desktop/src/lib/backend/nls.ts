import { BackendModule, ReadCallback } from "i18next";

import { AppService } from "../services";

interface I18nDictionary {
  [key: string]: string;
}

// Cache for translations to prevent duplicate API calls
const translationCache = new Map<string, I18nDictionary>();

const getCacheKey = (language: string, namespace: string): string => {
  return `${language}:${namespace}`;
};

const I18nTauriBackend: BackendModule = {
  type: "backend",
  init: () => {},
  read: async (language: string, namespace: string, callback: ReadCallback) => {
    const cacheKey = getCacheKey(language, namespace);
    if (translationCache.has(cacheKey)) {
      callback(null, translationCache.get(cacheKey)!);
      return;
    }

    try {
      const result = await AppService.getTranslationNamespace({ language, namespace });

      if (result.status === "ok") {
        const translations = result.data.contents as I18nDictionary;
        translationCache.set(cacheKey, translations);
        callback(null, translations);
      } else {
        throw new Error(String(result.error));
      }
    } catch (error) {
      callback(String(error), false);
    }
  },
};

export default I18nTauriBackend;

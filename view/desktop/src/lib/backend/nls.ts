import { BackendModule, ReadCallback } from "i18next";

import { GetTranslationsInput, GetTranslationsOutput } from "@repo/moss-app";

import { invokeTauriIpc, IpcResult } from "./tauri";

interface I18nDictionary {
  [key: string]: string;
}

// Cache for translations to prevent duplicate API calls
const translationCache = new Map<string, I18nDictionary>();

// Generate cache key from language and namespace
const getCacheKey = (language: string, namespace: string): string => {
  return `${language}:${namespace}`;
};

// Clear cache for a specific language (useful for language changes)
export const clearTranslationCache = (language?: string) => {
  if (language) {
    // Clear cache for specific language
    const keysToDelete = Array.from(translationCache.keys()).filter((key) => key.startsWith(`${language}:`));
    keysToDelete.forEach((key) => translationCache.delete(key));
  } else {
    // Clear entire cache
    translationCache.clear();
  }
};

const getTranslationsFn = async (input: GetTranslationsInput): Promise<IpcResult<GetTranslationsOutput, string>> => {
  return await invokeTauriIpc<GetTranslationsOutput, string>("get_translations", {
    input: input,
  });
};

const I18nTauriBackend: BackendModule = {
  type: "backend",
  init: () => {},
  read: async (language: string, namespace: string, callback: ReadCallback) => {
    const cacheKey = getCacheKey(language, namespace);

    // Check cache first - prevent duplicate API calls
    if (translationCache.has(cacheKey)) {
      callback(null, translationCache.get(cacheKey)!);
      return;
    }

    const result = await getTranslationsFn({ language, namespace });

    if (result.status === "ok") {
      const translations = result.data as I18nDictionary;
      // Cache the translations
      translationCache.set(cacheKey, translations);
      callback(null, translations);
    } else {
      callback(result.error, false);
    }
  },
};

export default I18nTauriBackend;

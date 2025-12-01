import { useMemo } from "react";
import { useTranslation } from "react-i18next";

import type { LocalizedString } from "@repo/base";

const NO_TRANSLATE_KEY = "__NO_TRANSLATE__";

/**
 * Hook to convert a LocalizedString to an actual translated string.
 *
 * If the key is NO_TRANSLATE_KEY, returns the fallback directly.
 * Otherwise, attempts to get the translation from i18n, falling back to the origin/fallback value.
 *
 * @param localizedString - The LocalizedString object from the backend
 * @returns The translated string or fallback
 */
export const useLocalizedString = (localizedString: LocalizedString): string => {
  const { t, i18n } = useTranslation();

  return useMemo(() => {
    if (!localizedString) return "";

    const { key, fallback } = localizedString;

    // If the key indicates no translation should be attempted, return the fallback
    if (key === NO_TRANSLATE_KEY) {
      return fallback;
    }

    // Check if translation exists for this key
    const translated = t(key, { defaultValue: "" });

    // If no translation found or i18n not ready, use the fallback
    if (!translated || !i18n.isInitialized) {
      return fallback;
    }

    return translated;
  }, [localizedString, t, i18n.isInitialized]);
};

import { useEffect, useEffectEvent, useState } from "react";

import { useGetBatchValue } from "@/adapters";
import { useDescribeColorTheme } from "@/adapters/tanstackQuery/colorTheme/useDescribeColorTheme";
import { initializeI18n } from "@/app/i18n";

import { applyThemeCSS } from "./utils";

export const useSyncSettings = () => {
  const { data: settings } = useGetBatchValue<{
    language: string;
    colorTheme: string;
  }>(["language", "colorTheme"]);

  const { isLanguageSynced } = useSyncLanguage(settings?.language);
  const { isColorThemeSynced } = useSyncColorTheme(settings?.colorTheme);

  return {
    isSynced: isLanguageSynced && isColorThemeSynced,
  };
};

const useSyncLanguage = (language: string | undefined | null) => {
  const [isLanguageSynced, setIsLanguageSynced] = useState(false);

  useEffect(() => {
    if (language) {
      initializeI18n(language).then(() => {
        setIsLanguageSynced(true);
      });
    }
  }, [language]);

  return {
    isLanguageSynced,
  };
};

const useSyncColorTheme = (colorThemeId: string | undefined | null) => {
  const [isColorThemeSynced, setIsColorThemeSynced] = useState(false);

  const { data: colorTheme } = useDescribeColorTheme({
    themeId: colorThemeId ?? "",
  });

  const applyTheme = useEffectEvent((colorThemeId: string, cssContent: string) => {
    applyThemeCSS(colorThemeId, cssContent);
    setIsColorThemeSynced(true);
  });

  useEffect(() => {
    if (colorThemeId && colorTheme?.cssContent) {
      applyTheme(colorThemeId, colorTheme.cssContent);
    }
  }, [colorThemeId, colorTheme?.cssContent]);

  return {
    isColorThemeSynced,
  };
};

import { useEffect, useState } from "react";

import { useDescribeColorTheme } from "@/app/adapters/tanstackQuery/colorTheme/useDescribeColorTheme";
import { useGetBatchSettingsValueWithDefaults } from "@/app/adapters/tanstackQuery/settings/useGetBatchSettingsValueWithDefaults";
import { initializeI18n } from "@/app/i18n";

import { applyThemeCSS } from "./utils";

export const useSyncSettings = () => {
  const [isColorThemeLoaded, setIsColorThemeLoaded] = useState(false);
  const [isLanguageLoaded, setIsLanguageLoaded] = useState(false);

  const { data: settings } = useGetBatchSettingsValueWithDefaults<{
    language: string;
    colorTheme: string;
  }>(["language", "colorTheme"], {
    language: "en",
    colorTheme: "default",
  });

  const { data: colorTheme } = useDescribeColorTheme({
    themeId: settings?.colorTheme as string,
  });

  useEffect(() => {
    if (settings?.language) {
      initializeI18n(settings.language);
      setIsLanguageLoaded(true);
    }
    if (settings?.colorTheme) {
      applyThemeCSS(settings.colorTheme as string, colorTheme?.cssContent ?? "");
      setIsColorThemeLoaded(true);
    }
  }, [settings, colorTheme]);

  return {
    isSynced: isLanguageLoaded && isColorThemeLoaded,
  };
};

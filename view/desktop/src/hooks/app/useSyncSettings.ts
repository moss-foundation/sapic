import { useEffect, useState } from "react";

import { useGetBatchValue } from "@/adapters";
import { useDescribeColorTheme } from "@/adapters/tanstackQuery/colorTheme/useDescribeColorTheme";
import { initializeI18n } from "@/app/i18n";

import { applyThemeCSS } from "./utils";

export const useSyncSettings = () => {
  const [isColorThemeLoaded, setIsColorThemeLoaded] = useState(false);
  const [isLanguageLoaded, setIsLanguageLoaded] = useState(false);

  const { data: settings } = useGetBatchValue<{
    language: string;
    colorTheme: string;
  }>(["language", "colorTheme"]);

  const { data: colorTheme } = useDescribeColorTheme({
    themeId: settings?.colorTheme ?? "",
  });

  useEffect(() => {
    if (settings?.language) {
      initializeI18n(settings.language).then(() => {
        setIsLanguageLoaded(true);
      });
    }
  }, [settings?.language]);

  useEffect(() => {
    if (settings?.colorTheme) {
      applyThemeCSS(settings.colorTheme, colorTheme?.cssContent ?? "").then(() => {
        setIsColorThemeLoaded(true);
      });
    }
  }, [colorTheme?.cssContent, settings?.colorTheme]);

  return {
    isSynced: isLanguageLoaded && isColorThemeLoaded,
  };
};

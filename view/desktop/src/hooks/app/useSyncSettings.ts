import { useEffect } from "react";

import { initializeI18n } from "@/app/i18n";

import { useDescribeColorTheme } from "./colorTheme";
import { useGetSettings } from "./settings/useGetSettings";
import { applyThemeCSS } from "./utils";

export const SETTINGS_QUERY_KEY = "application.settings" as const;

export const useSyncSettings = () => {
  const { data: settings, isSuccess: isSuccessSettings } = useGetSettings<{ language: string; colorTheme: string }>([
    "language",
    "colorTheme",
  ]);

  const { data: colorTheme, isSuccess: isSuccessColorTheme } = useDescribeColorTheme({
    themeId: settings?.colorTheme as string,
  });

  useEffect(() => {
    if (settings?.language) {
      initializeI18n(settings.language);
    }
    if (settings?.colorTheme) {
      applyThemeCSS(settings.colorTheme as string, colorTheme?.cssContent ?? "");
    }
  }, [settings, colorTheme]);

  return {
    isSynced: isSuccessSettings && isSuccessColorTheme,
  };
};

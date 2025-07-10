import { ReactNode, useEffect } from "react";

import ErrorBoundary from "@/components/ErrorBoundary";
import { useDescribeAppState } from "@/hooks/appState/useDescribeAppState";
import { applyLanguagePack } from "@/utils/applyLanguagePack";
import { applyColorThemeFromCache } from "@/utils/applyTheme";
import { initializeI18n } from "@/app/i18n";
import { useQueryClient } from "@tanstack/react-query";

import LanguageProvider from "./LanguageProvider";
import ThemeProvider from "./ThemeProvider";

const Provider = ({ children }: { children: ReactNode }) => {
  useInitializeAppState();

  return (
    <ErrorBoundary>
      <LanguageProvider>
        <ThemeProvider>{children}</ThemeProvider>
      </LanguageProvider>
    </ErrorBoundary>
  );
};

const useInitializeAppState = () => {
  const { data } = useDescribeAppState();
  const queryClient = useQueryClient();

  useEffect(() => {
    if (data) {
      const theme = data.preferences?.theme ?? data.defaults.theme;
      const languagePack = data.preferences?.locale ?? data.defaults.locale;

      document.querySelector("html")?.setAttribute("data-theme", theme.mode);

      applyColorThemeFromCache(theme.identifier, queryClient);

      initializeI18n(languagePack.code)
        .then(() => {
          applyLanguagePack(languagePack).catch(console.error);
        })
        .catch(console.error);
    }
  }, [data, queryClient]);
};

export default Provider;

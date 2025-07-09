import { ReactNode, useEffect } from "react";

import ErrorBoundary from "@/components/ErrorBoundary";
import { useDescribeAppState } from "@/hooks/appState/useDescribeAppState";
import { applyLanguagePack } from "@/utils/applyLanguagePack";
import { applyColorTheme } from "@/utils/applyTheme";

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

  useEffect(() => {
    if (data) {
      const theme = data.preferences?.theme ?? data.defaults.theme;
      const languagePack = data.preferences?.locale ?? data.defaults.locale;

      document.querySelector("html")?.setAttribute("data-theme", theme.mode);

      applyColorTheme(theme.identifier);
      applyLanguagePack(languagePack).catch(console.error);
    }
  }, [data]);
};

export default Provider;

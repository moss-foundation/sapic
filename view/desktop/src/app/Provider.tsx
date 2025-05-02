import { ReactNode, useEffect } from "react";

import ErrorBoundary from "@/components/ErrorBoundary";
import { ActivityEventsProvider } from "@/context/ActivityEventsContext";
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
        <ThemeProvider>
          <ActivityEventsProvider>{children}</ActivityEventsProvider>
        </ThemeProvider>
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
      applyLanguagePack(languagePack);
    }
  }, [data]);
};

export default Provider;

import { ReactNode, useEffect } from "react";

import { ActivityEventsProvider } from "@/context/ActivityEventsContext";
import { useDescribeAppState } from "@/hooks/useDescribeAppState";
import { applyLanguagePack } from "@/utils/applyLanguagePack";
import { applyColorTheme } from "@/utils/applyTheme";

import LanguageProvider from "./LanguageProvider";
import ThemeProvider from "./ThemeProvider";

const Provider = ({ children }: { children: ReactNode }) => {
  useInitializeAppState();

  return (
    <LanguageProvider>
      <ThemeProvider>
        <ActivityEventsProvider>{children}</ActivityEventsProvider>
      </ThemeProvider>
    </LanguageProvider>
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

import { ReactNode, useEffect } from "react";

import { useDescribeAppState, useSetLocale } from "@/hooks";
import { applyLanguagePack } from "@/utils/applyLanguagePack";
import { LocaleInfo } from "@repo/moss-app";
import { listen, UnlistenFn } from "@tauri-apps/api/event";

import { initializeI18n } from "./i18n";

interface LanguageProviderProps {
  children: ReactNode;
}

const LanguageProvider = ({ children }: LanguageProviderProps) => {
  const { data } = useDescribeAppState();
  const { mutateAsync: setLocale } = useSetLocale();

  // Initialize language
  useEffect(() => {
    if (data) {
      const languagePack = data.preferences?.locale ?? data.defaults.locale;

      initializeI18n(languagePack.code)
        .then(() => {
          applyLanguagePack(languagePack).catch(console.error);
        })
        .catch(console.error);
    }
  }, [data]);

  // Listen for language pack changes
  useEffect(() => {
    let unlisten: UnlistenFn | undefined;

    const handleLanguageChange = (event: { payload: LocaleInfo }) => {
      setLocale({ localeInfo: event.payload });
    };

    const setupListener = async () => {
      try {
        unlisten = await listen("core://language-pack-changed", handleLanguageChange);
      } catch (error) {
        console.error("Failed to set up language pack change listener:", error);
      }
    };

    setupListener();

    return () => {
      unlisten?.();
    };
  }, [setLocale]);

  return <>{children}</>;
};

export default LanguageProvider;

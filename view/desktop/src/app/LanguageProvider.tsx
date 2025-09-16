import { ReactNode, useEffect, useRef } from "react";

import { useSetLocale } from "@/hooks";
import { useGetLocale } from "@/hooks/app/locales/useGetLocale";
import { useDescribeApp } from "@/hooks/useDescribeApp";
import { LocaleInfo } from "@repo/moss-app";
import { listen, UnlistenFn } from "@tauri-apps/api/event";

import { initializeI18n } from "./i18n";

interface LanguageProviderProps {
  children: ReactNode;
}

const LanguageProvider = ({ children }: LanguageProviderProps) => {
  const isInitialized = useRef(false);

  const { data: appState, isSuccess } = useDescribeApp();
  const { setLocaleLocally } = useSetLocale();

  const localeId = appState?.configuration.contents.locale as string;

  const { data: locale, isSuccess: isLocaleSuccess } = useGetLocale({
    identifier: localeId,
    options: { enabled: !!localeId },
  });

  // Initialize language
  useEffect(() => {
    const initialize = async () => {
      if (!locale || isInitialized.current) {
        return;
      }

      try {
        initializeI18n(locale.code);

        isInitialized.current = true;
      } catch (error) {
        console.error("Failed to initialize locale:", error);
      }
    };

    if (appState && isSuccess && isLocaleSuccess) {
      initialize();
    }
  }, [appState, setLocaleLocally, isSuccess, isLocaleSuccess, locale]);

  // Listen for language pack changes
  useEffect(() => {
    let unlisten: UnlistenFn | undefined;

    const handleLanguageChange = (event: { payload: LocaleInfo }) => {
      const eventLang = event.payload;

      setLocaleLocally({
        localeInfo: {
          identifier: eventLang.identifier,
          displayName: eventLang.displayName,
          code: eventLang.code,
        },
      });
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
  }, [setLocaleLocally]);

  return <>{children}</>;
};

export default LanguageProvider;

import { ReactNode, useEffect } from "react";

import { useSetLocale } from "@/hooks";
import { useGetLocale } from "@/hooks/app/locales/useGetLocale";
import { useDescribeApp } from "@/hooks/app/useDescribeApp";
import { LocaleInfo } from "@repo/moss-app";
import { listen, UnlistenFn } from "@tauri-apps/api/event";

import { initializeI18n } from "./i18n";

interface LanguageProviderProps {
  children: ReactNode;
}

const LanguageProvider = ({ children }: LanguageProviderProps) => {
  const { data: appState } = useDescribeApp();
  const { setLocaleLocally } = useSetLocale();

  const localeId = appState?.configuration.contents.locale as string;

  const { data: locale } = useGetLocale({
    identifier: localeId,
    options: { enabled: !!localeId },
  });

  useEffect(() => {
    if (!locale) return;
    initializeI18n(locale.code);
  }, [locale]);

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

import { ReactNode, useEffect } from "react";

import { useSetLocale } from "@/hooks";
import { useGetLocale } from "@/hooks/app/locales/useGetLocale";
import { useDescribeApp } from "@/hooks/useDescribeApp";
import { LocaleInfo } from "@repo/moss-app";
import { listen, UnlistenFn } from "@tauri-apps/api/event";

interface LanguageProviderProps {
  children: ReactNode;
}

const LanguageProvider = ({ children }: LanguageProviderProps) => {
  const { data: appState, isSuccess } = useDescribeApp();
  const { applyLocaleById } = useSetLocale();
  const { getLocaleById } = useGetLocale({ identifier: appState?.configuration.contents.locale as string });

  // Initialize language
  useEffect(() => {
    const initialize = async () => {
      if (!appState?.configuration.contents.locale) {
        throw new Error("Locale not found");
      }

      const localePackId = appState.configuration.contents.locale as string;

      const locale = await getLocaleById(localePackId);
      applyLocaleById(locale.code);
    };

    if (appState && isSuccess) {
      initialize();
    }
  }, [appState, applyLocaleById, getLocaleById, isSuccess]);

  // Listen for language pack changes
  useEffect(() => {
    let unlisten: UnlistenFn | undefined;

    const handleLanguageChange = (event: { payload: LocaleInfo }) => {
      applyLocaleById(event.payload.code);
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
  }, [applyLocaleById]);

  return <>{children}</>;
};

export default LanguageProvider;

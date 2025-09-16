import { ReactNode, useEffect, useRef } from "react";

import { useSetLocale } from "@/hooks";
import { useGetLocale } from "@/hooks/app/locales/useGetLocale";
import { useDescribeApp } from "@/hooks/useDescribeApp";
import { LocaleInfo } from "@repo/moss-app";
import { listen, UnlistenFn } from "@tauri-apps/api/event";

interface LanguageProviderProps {
  children: ReactNode;
}

const LanguageProvider = ({ children }: LanguageProviderProps) => {
  const isInitialized = useRef(false);

  const { data: appState, isSuccess } = useDescribeApp();
  const { mutateAsync: mutateChangeLanguagePack } = useSetLocale();

  const localeId = appState?.configuration.contents.locale as string;

  const { data: locale, isSuccess: isLocaleSuccess } = useGetLocale({
    identifier: localeId,
    options: { enabled: !!localeId },
  });

  // Initialize language
  useEffect(() => {
    const initialize = async () => {
      if (!localeId || !locale || isInitialized.current) {
        return;
      }

      try {
        mutateChangeLanguagePack({
          localeInfo: {
            identifier: localeId,
            displayName: locale.displayName,
            code: locale.code,
          },
        });

        isInitialized.current = true;
      } catch (error) {
        console.error("Failed to initialize locale:", error);
      }
    };

    if (appState && isSuccess && isLocaleSuccess) {
      initialize();
    }
  }, [appState, mutateChangeLanguagePack, isSuccess, isLocaleSuccess, locale, localeId]);

  // Listen for language pack changes
  useEffect(() => {
    let unlisten: UnlistenFn | undefined;

    const handleLanguageChange = (event: { payload: LocaleInfo }) => {
      const eventLang = event.payload;

      mutateChangeLanguagePack({
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
  }, [mutateChangeLanguagePack]);

  return <>{children}</>;
};

export default LanguageProvider;

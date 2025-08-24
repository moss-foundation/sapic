import { ReactNode, useEffect } from "react";

import { useSetLocale } from "@/hooks";
import { LocaleInfo } from "@repo/moss-app";
import { listen, UnlistenFn } from "@tauri-apps/api/event";

interface LanguageProviderProps {
  children: ReactNode;
}

const LanguageProvider = ({ children }: LanguageProviderProps) => {
  const { mutateAsync: setLocale } = useSetLocale();

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

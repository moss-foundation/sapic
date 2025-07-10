import { useEffect } from "react";

import { USE_DESCRIBE_APP_STATE_QUERY_KEY } from "@/hooks/appState/useDescribeAppState";
import { applyLanguagePack } from "@/utils/applyLanguagePack";
import { LocaleInfo } from "@repo/moss-app";
import { useQueryClient } from "@tanstack/react-query";
import { listen, UnlistenFn } from "@tauri-apps/api/event";

const LanguageProvider = ({ children }: { children: React.ReactNode }) => {
  const queryClient = useQueryClient();

  useEffect(() => {
    let unlisten: UnlistenFn | undefined;

    const handleLanguageChange = (event: { payload: LocaleInfo }) => {
      applyLanguagePack(event.payload).catch(console.error);
      queryClient.invalidateQueries({ queryKey: [USE_DESCRIBE_APP_STATE_QUERY_KEY] });
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
      if (unlisten) {
        unlisten();
      }
    };
  }, [queryClient]);

  return <>{children}</>;
};

export default LanguageProvider;

import { useEffect } from "react";

import { USE_DESCRIBE_APP_STATE_QUERY_KEY } from "@/hooks/appState/useDescribeAppState";
import { applyColorThemeFromCache } from "@/utils/applyTheme";
import { ColorThemeInfo } from "@repo/moss-app";
import { useQueryClient } from "@tanstack/react-query";
import { listen, UnlistenFn } from "@tauri-apps/api/event";

const ThemeProvider = ({ children }: { children: React.ReactNode }) => {
  const queryClient = useQueryClient();

  useEffect(() => {
    let unlisten: UnlistenFn | undefined;

    const handleThemeChange = (event: { payload: ColorThemeInfo }) => {
      applyColorThemeFromCache(event.payload.identifier, queryClient);
      queryClient.invalidateQueries({ queryKey: [USE_DESCRIBE_APP_STATE_QUERY_KEY] });
    };

    const setupListener = async () => {
      try {
        unlisten = await listen("core://color-theme-changed", handleThemeChange);
      } catch (error) {
        console.error("Failed to set up color theme change listener:", error);
      }
    };

    setupListener();

    return () => {
      unlisten?.();
    };
  }, [queryClient]);

  return <>{children}</>;
};

export default ThemeProvider;

import { ReactNode, useEffect } from "react";

import { applyTheme } from "@/utils/applyTheme";
import { ColorThemeChangeEventPayload } from "@repo/moss-theme";
import { useQueryClient } from "@tanstack/react-query";
import { listen, UnlistenFn } from "@tauri-apps/api/event";

const ThemeProvider = ({ children }: { children: ReactNode }) => {
  const queryClient = useQueryClient();

  useEffect(() => {
    let unlisten: UnlistenFn | undefined;

    const handleColorThemeChanged = (event: { payload: ColorThemeChangeEventPayload }) => {
      applyTheme(event.payload.id);
      queryClient.invalidateQueries({ queryKey: ["getState"] });
    };

    const setupListener = async () => {
      try {
        unlisten = await listen("core://color-theme-changed", handleColorThemeChanged);
      } catch (error) {
        console.error("Failed to set up theme change listener:", error);
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

export default ThemeProvider;

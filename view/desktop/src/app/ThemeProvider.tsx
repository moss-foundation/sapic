import { useEffect } from "react";

import { useDescribeColorTheme } from "@/hooks";
import { useDescribeApp } from "@/hooks/useDescribeApp";
import { ColorThemeInfo } from "@repo/moss-app";
import { useQueryClient } from "@tanstack/react-query";
import { listen, UnlistenFn } from "@tauri-apps/api/event";

const ThemeProvider = ({ children }: { children: React.ReactNode }) => {
  const queryClient = useQueryClient();

  const { data: appState } = useDescribeApp();
  const { data: colorTheme } = useDescribeColorTheme({
    themeId: (appState?.configuration.contents.colorTheme as string) ?? "",
  });

  useEffect(() => {
    if (appState && colorTheme) {
      const theme = appState.configuration.contents.colorTheme as string;

      setThemeStyle(theme, colorTheme.cssContent);
    }
  }, [appState, colorTheme, queryClient]);

  useEffect(() => {
    let unlisten: UnlistenFn | undefined;

    const handleThemeChange = (event: { payload: ColorThemeInfo }) => {
      setThemeStyle(event.payload.identifier, colorTheme?.cssContent ?? "");
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
  }, [queryClient, colorTheme]);

  return <>{children}</>;
};

export default ThemeProvider;

const setThemeStyle = (id: string, css: string): void => {
  let styleTag = document.getElementById("theme-style") as HTMLStyleElement | null;

  if (!styleTag) {
    styleTag = document.createElement("style");
    styleTag.id = "theme-style";
    document.head.appendChild(styleTag);
  }
  document.querySelector("html")?.setAttribute("data-theme", id);

  styleTag.innerHTML = css;
};

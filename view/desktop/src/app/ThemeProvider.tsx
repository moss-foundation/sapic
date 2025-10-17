import { useEffect } from "react";

import { useDescribeColorTheme } from "@/hooks";
import { useDescribeApp } from "@/hooks/app/useDescribeApp";
import { useQueryClient } from "@tanstack/react-query";

const ThemeProvider = ({ children }: { children: React.ReactNode }) => {
  const queryClient = useQueryClient();

  const { data: appState } = useDescribeApp();
  const { data: colorThemeCss } = useDescribeColorTheme({
    themeId: (appState?.configuration.contents.colorTheme as string) ?? "",
    enabled: !!appState?.configuration.contents.colorTheme,
  });

  useEffect(() => {
    const colorThemeId = appState?.configuration.contents.colorTheme; //TODO this should be able to handle JSON value in the future
    if (colorThemeId && colorThemeCss) {
      applyThemeStyles(colorThemeId as string, colorThemeCss.cssContent);
    }
  }, [appState, colorThemeCss, queryClient]);

  return <>{children}</>;
};

export default ThemeProvider;

const applyThemeStyles = (id: string, css: string): void => {
  let styleTag = document.getElementById("theme-style") as HTMLStyleElement | null;

  if (!styleTag) {
    styleTag = document.createElement("style");
    styleTag.id = "theme-style";
    document.head.appendChild(styleTag);
  }
  document.querySelector("html")?.setAttribute("data-theme", id);

  styleTag.innerHTML = css;
};

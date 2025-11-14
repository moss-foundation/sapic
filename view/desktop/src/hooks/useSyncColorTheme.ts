import { useEffect, useEffectEvent, useState } from "react";

import { useDescribeApp, useDescribeColorTheme } from "./app";

export const useSyncColorTheme = () => {
  const { data: appState } = useDescribeApp();
  const { data: colorThemeCss, isSuccess } = useDescribeColorTheme({
    themeId: (appState?.configuration.contents.colorTheme as string) ?? "",
  });

  const [isInit, setIsInit] = useState(false);

  const updateTheme = useEffectEvent(() => {
    if (!isSuccess) return;
    const colorThemeId = appState?.configuration.contents.colorTheme; //TODO this should be able to handle JSON value in the future

    if (colorThemeId && colorThemeCss) {
      applyThemeStyles(colorThemeId as string, colorThemeCss.cssContent);
    }

    setIsInit(true);
  });

  useEffect(() => {
    updateTheme();
  }, [appState?.configuration.contents.colorTheme, colorThemeCss]);

  return { isInit: isInit };
};

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

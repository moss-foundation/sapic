import { useEffect, useState } from "react";

import { useDescribeApp, useDescribeColorTheme, useDescribeWorkspaceState } from "@/hooks";
import { useGetLayout } from "@/hooks/sharedStorage/layout/useGetLayout";

import { initializeI18n } from "./i18n";

export const LoadingBoundary = ({ children }: { children: React.ReactNode }) => {
  const { data: appState, isFetching: isFetchingApp } = useDescribeApp();
  const {
    data: colorThemeCss,
    isSuccess,
    isFetching: isFetchingTheme,
  } = useDescribeColorTheme({
    themeId: (appState?.configuration.contents.colorTheme as string) ?? "",
    enabled: !!appState?.configuration.contents.colorTheme,
  });
  const { isFetching: isFetchingWorkspace } = useDescribeWorkspaceState();
  const { isLoading: isLoadingLayout } = useGetLayout();

  const langCode = appState?.configuration.contents.language as string;

  useEffect(() => {
    if (!langCode) return;
    initializeI18n(langCode);
  }, [langCode]);

  const [isFirstWorkspaceFetch, setIsFirstWorkspaceFetch] = useState(true);

  const isLoading =
    isFetchingApp || isFetchingTheme || (isFetchingWorkspace && isFirstWorkspaceFetch) || isLoadingLayout;

  if (isSuccess) {
    const colorThemeId = appState?.configuration.contents.colorTheme; //TODO this should be able to handle JSON value in the future

    if (colorThemeId && colorThemeCss) {
      applyThemeStyles(colorThemeId as string, colorThemeCss.cssContent);
    }
  }

  if (isLoading) {
    return null;
  }

  if (isFirstWorkspaceFetch) {
    setIsFirstWorkspaceFetch(false);
  }

  return <>{children}</>;
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

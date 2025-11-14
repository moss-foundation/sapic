import { useEffect, useEffectEvent, useState } from "react";

import { useDescribeApp, useDescribeColorTheme } from "@/hooks";
import { useGetLayout } from "@/hooks/workbench/layout/useGetLayout";
import { PageLoader } from "@/workbench/ui/components";

import { initializeI18n } from "./i18n";

export const LoadingBoundary = ({ children }: { children: React.ReactNode }) => {
  const [isInitializing, setIsInitializing] = useState(true);

  const { data: appState, isPending: isPendingApp } = useDescribeApp();

  const {
    data: colorThemeCss,
    isSuccess: isSuccessTheme,
    isPending: isPendingTheme,
  } = useDescribeColorTheme({
    themeId: (appState?.configuration.contents.colorTheme as string) ?? "",
    enabled: !!appState?.configuration.contents.colorTheme,
  });

  const { isPending: isPendingLayout } = useGetLayout();

  const langCode = appState?.configuration.contents.language as string;

  useEffect(() => {
    if (!langCode) return;
    initializeI18n(langCode);
  }, [langCode]);

  const isPending = isPendingApp || isPendingTheme || isPendingLayout;

  const handleInitializing = useEffectEvent(() => {
    setIsInitializing(false);
  });

  useEffect(() => {
    if (isPending) return;
    handleInitializing();
  }, [isPending]);

  if (isSuccessTheme) {
    const colorThemeId = appState?.configuration.contents.colorTheme; //TODO this should be able to handle JSON value in the future

    if (colorThemeId && colorThemeCss) {
      applyThemeStyles(colorThemeId as string, colorThemeCss.cssContent);
    }
  }

  if (isInitializing && isPending) {
    return <PageLoader className="bg-green-200" />;
  } else {
    return <>{children}</>;
  }
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

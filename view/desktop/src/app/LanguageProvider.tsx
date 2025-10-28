import { ReactNode, useEffect } from "react";

import { useDescribeApp } from "@/hooks/app/useDescribeApp";

import { initializeI18n } from "./i18n";

interface LanguageProviderProps {
  children: ReactNode;
}

const LanguageProvider = ({ children }: LanguageProviderProps) => {
  const { data: appState } = useDescribeApp();
  const langCode = appState?.configuration.contents.language as string;

  useEffect(() => {
    if (!langCode) return;
    initializeI18n(langCode);
  }, [langCode]);

  return <>{children}</>;
};

export default LanguageProvider;

import { useEffect, useEffectEvent, useState } from "react";

import { initializeI18n } from "@/app/i18n";

import { useDescribeApp } from "./app";

export const useSyncLanguage = () => {
  const { data: appState, isSuccess: isSuccessApp } = useDescribeApp();
  const langCode = appState?.configuration.contents.language as string;

  const [isInit, setIsInit] = useState(false);

  const updateLanguage = useEffectEvent(() => {
    if (!langCode) return;
    initializeI18n(langCode);
    setIsInit(true);
  });

  useEffect(() => {
    updateLanguage();
  }, [langCode]);

  return { isInit: isSuccessApp && isInit };
};

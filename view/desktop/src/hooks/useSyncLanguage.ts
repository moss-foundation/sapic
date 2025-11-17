import { useEffect, useEffectEvent, useState } from "react";

import { initializeI18n } from "@/app/i18n";
import { settingsStorageService } from "@/app/services/settingsStorage";

import { useDescribeApp } from "./app";

export const useSyncLanguage = () => {
  const { data: _, isSuccess: isSuccessApp } = useDescribeApp();
  const [langCode, setLangCode] = useState<string | null>(null);
  const [isInit, setIsInit] = useState(false);

  useEffect(() => {
    settingsStorageService.getValue<string>("language").then((value) => {
      setLangCode(value);
    });
  }, []);

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

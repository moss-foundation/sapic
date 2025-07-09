import { changeLanguage } from "@/app/i18n";
import { LocaleInfo } from "@repo/moss-app";

export const applyLanguagePack = async (languagePack: LocaleInfo) => {
  try {
    await changeLanguage(languagePack.code);
  } catch (error) {
    console.error("Failed to apply language pack:", error);
  }
};

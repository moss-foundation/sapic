import i18n from "@/app/i18n";
import { LocaleInfo } from "@repo/moss-app";

export const applyLanguagePack = (languagePack: LocaleInfo) => {
  i18n.changeLanguage(languagePack.code);
};

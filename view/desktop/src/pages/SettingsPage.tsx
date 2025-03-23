import { useTranslation } from "react-i18next";

import { useDescribeAppState } from "@/hooks/useDescribeAppState";
import { useListColorThemes } from "@/hooks/useListColorThemes";
import { useListLocales } from "@/hooks/useListLocales";
import { useSetColorTheme } from "@/hooks/useSetColorTheme";
import { useSetLocale } from "@/hooks/useSetLocale";
import { ColorThemeInfo } from "@repo/moss-theme";

export const Settings = () => {
  const { t } = useTranslation(["ns1", "ns2"]);

  const { data: appState } = useDescribeAppState();

  const { data: themes } = useListColorThemes();
  const { mutate: mutateChangeColorTheme } = useSetColorTheme();

  const { data: languages } = useListLocales();
  const { mutate: mutateChangeLanguagePack } = useSetLocale();

  const handleLanguageChange = (event: React.ChangeEvent<HTMLSelectElement>) => {
    const selectedLocaleCode = event.target.value;
    const selectedLocaleInfo = languages?.contents.find((lang: { code: string }) => lang.code === selectedLocaleCode);
    if (selectedLocaleInfo) {
      mutateChangeLanguagePack({
        localeInfo: selectedLocaleInfo,
      });
    }
  };

  const handleThemeChange = (event: React.ChangeEvent<HTMLSelectElement>) => {
    const selectedId = event.target.value;
    const selectedTheme = themes?.find(
      (theme: { identifier: string; displayName: string }) => theme.identifier === selectedId
    );
    if (selectedTheme) {
      mutateChangeColorTheme({
        themeInfo: selectedTheme,
      });
    }
  };

  return (
    <main>
      <div className="p-5">
        <h1 className="mb-3 text-2xl">{t("settings")}</h1>

        <div>
          <h3>{t("selectLanguage")}</h3>
          <select
            id="lang-select"
            className="rounded border bg-gray-400 p-2"
            value={appState?.preferences.locale?.code || appState?.defaults.locale?.code}
            onChange={handleLanguageChange}
          >
            {languages?.contents.map((lang: { code: string; displayName: string }) => (
              <option key={lang.code} value={lang.code}>
                {lang.displayName}
              </option>
            ))}
          </select>
        </div>

        <div>
          <h3>{t("selectTheme")}</h3>
          <select
            id="theme-select"
            className="rounded border bg-gray-400 p-2"
            value={appState?.preferences.theme?.identifier || appState?.defaults.theme?.identifier}
            onChange={handleThemeChange}
          >
            {themes?.map((theme_info: ColorThemeInfo) => (
              <option key={theme_info.identifier} value={theme_info.identifier}>
                {theme_info.displayName}
              </option>
            ))}
          </select>
        </div>
      </div>
    </main>
  );
};

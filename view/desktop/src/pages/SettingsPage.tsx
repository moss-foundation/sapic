import { useTranslation } from "react-i18next";

import { useChangeColorTheme, useListColorThemes } from "@/hooks/useColorTheme";
import { useGetAppState } from "@/hooks/useGetAppState";
import { useChangeLanguagePack, useGetLanguagePacks } from "@/hooks/useLanguagePack";
import { ColorThemeDescriptor } from "@repo/moss-theme";

export const Settings = () => {
  const { t } = useTranslation(["ns1", "ns2"]);

  const { data: appState } = useGetAppState();

  const { data: themes } = useListColorThemes();
  const { mutate: mutateChangeColorTheme } = useChangeColorTheme();

  const { data: languages } = useGetLanguagePacks();
  const { mutate: mutateChangeLanguagePack } = useChangeLanguagePack();

  const handleLanguageChange = (event: React.ChangeEvent<HTMLSelectElement>) => {
    const selectedCode = event.target.value;
    const selectedLang = languages?.contents.find((lang: { code: string }) => lang.code === selectedCode);
    if (selectedLang) {
      mutateChangeLanguagePack(selectedLang);
    }
  };

  const handleThemeChange = (event: React.ChangeEvent<HTMLSelectElement>) => {
    const selectedId = event.target.value;
    const selectedTheme = themes?.find(
      (theme: { identifier: string; displayName: string }) => theme.identifier === selectedId
    );
    if (selectedTheme) {
      mutateChangeColorTheme(selectedTheme);
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
            {themes?.map((theme: ColorThemeDescriptor) => (
              <option key={theme.identifier} value={theme.identifier}>
                {theme.displayName}
              </option>
            ))}
          </select>
        </div>
      </div>
    </main>
  );
};

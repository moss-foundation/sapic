import { useTranslation } from "react-i18next";

import { useDescribeAppState } from "@/hooks/useDescribeAppState";
import { useListColorThemes } from "@/hooks/useListColorThemes";
import { useListLocales } from "@/hooks/useListLocales";
import { useSetColorTheme } from "@/hooks/useSetColorTheme";
import { useSetLocale } from "@/hooks/useSetLocale";
import { ColorThemeInfo } from "@repo/moss-theme";

import { ActivityBarPosition, useActivityBarStore } from "../store/activityBarStore";
import { SideBarPosition, useSideBarStore } from "../store/sideBarStore";

export const Settings = () => {
  const { t } = useTranslation(["ns1", "ns2"]);

  const { data: appState } = useDescribeAppState();

  const { data: themes } = useListColorThemes();
  const { mutate: mutateChangeColorTheme } = useSetColorTheme();

  const { data: languages } = useListLocales();
  const { mutate: mutateChangeLanguagePack } = useSetLocale();

  const { sideBarPosition, setSideBarPosition } = useSideBarStore();
  const { setActivityBarPosition, activityBarPosition } = useActivityBarStore();

  const handleLanguageChange = (event: React.ChangeEvent<HTMLSelectElement>) => {
    const selectedLocaleCode = event.target.value;
    const selectedLocaleInfo = languages?.find(
      (lang: { code: string; displayName: string }) => lang.code === selectedLocaleCode
    );
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

  const handleSideBarPositionChange = (event: React.ChangeEvent<HTMLSelectElement>) => {
    const position = event.target.value as SideBarPosition;
    setSideBarPosition(position);
    // Update ActivityBar position only if it's currently set to left or right
    if (activityBarPosition === "left" || activityBarPosition === "right") {
      setActivityBarPosition(position);
    }
  };

  const handleActivityBarPositionChange = (event: React.ChangeEvent<HTMLSelectElement>) => {
    const position = event.target.value as ActivityBarPosition;
    if (position === "default") {
      // Set ActivityBar position to match SideBar position
      setActivityBarPosition(sideBarPosition);
    } else {
      setActivityBarPosition(position);
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
            {languages?.map((lang: { code: string; displayName: string }) => (
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

        <div>
          <h3>SideBar Position</h3>
          <select
            id="sidebar-position-select"
            className="rounded border bg-gray-400 p-2"
            onChange={handleSideBarPositionChange}
          >
            <option value="left">Left</option>
            <option value="right">Right</option>
          </select>
        </div>

        <div>
          <h3>ActivityBar Position</h3>
          <select
            id="activitybar-position-select"
            className="rounded border bg-gray-400 p-2"
            onChange={handleActivityBarPositionChange}
          >
            <option value="default">Default</option>
            <option value="top">Top</option>
            <option value="bottom">Bottom</option>
            <option value="hidden">Hidden</option>
          </select>
        </div>
      </div>
    </main>
  );
};

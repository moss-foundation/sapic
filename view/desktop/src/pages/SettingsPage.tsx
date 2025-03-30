import { ReactNode, ChangeEvent } from "react";
import { useTranslation } from "react-i18next";

import { useDescribeAppState } from "@/hooks/useDescribeAppState";
import { useGetAppLayoutState } from "@/hooks/useGetAppLayoutState";
import { useChangeAppLayoutState } from "@/hooks/useChangeAppLayoutState";
import { ActivityBarState } from "@/hooks/useActivityBarState";
import { useGetActivityBarState } from "@/hooks/useGetActivityBarState";
import { useChangeActivityBarState } from "@/hooks/useChangeActivityBarState";
import { useListColorThemes } from "@/hooks/useListColorThemes";
import { useListLocales } from "@/hooks/useListLocales";
import { useSetColorTheme } from "@/hooks/useSetColorTheme";
import { useSetLocale } from "@/hooks/useSetLocale";
import { useAppResizableLayoutStore } from "@/store/appResizableLayout";
import { ColorThemeInfo } from "@repo/moss-theme";

interface SettingsDropdownProps {
  id: string;
  label: string;
  value: string;
  onChange: (event: ChangeEvent<HTMLSelectElement>) => void;
  children: ReactNode;
}

const SettingsDropdown = ({ id, label, value, onChange, children }: SettingsDropdownProps) => (
  <div className="mt-4">
    <h3 className="mb-2 font-medium text-[var(--moss-select-text-outlined)]">{label}</h3>
    <select
      id={id}
      className="w-full rounded border border-[var(--moss-select-border-outlined)] bg-[var(--moss-select-bg-outlined)] p-2 text-[var(--moss-select-text-outlined)] shadow-sm hover:bg-[var(--moss-select-hover-bg)] focus:border-[var(--moss-select-focus-border)] focus:ring-1 focus:ring-[var(--moss-select-focus-border)] focus:outline-none"
      value={value}
      onChange={onChange}
    >
      {children}
    </select>
  </div>
);

export const Settings = () => {
  const { t } = useTranslation(["ns1", "ns2"]);

  const { data: appState } = useDescribeAppState();
  const { data: appLayoutState } = useGetAppLayoutState();
  const { mutate: changeAppLayoutState } = useChangeAppLayoutState();
  const { bottomPane } = useAppResizableLayoutStore();

  const { data: themes } = useListColorThemes();
  const { mutate: mutateChangeColorTheme } = useSetColorTheme();

  const { data: languages } = useListLocales();
  const { mutate: mutateChangeLanguagePack } = useSetLocale();

  const { data: activityBarStateData } = useGetActivityBarState();
  const { mutate: changeActivityBarState } = useChangeActivityBarState();

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

  const handleSidebarTypeChange = (event: React.ChangeEvent<HTMLSelectElement>) => {
    const sidebarType = event.target.value as "left" | "right";

    changeAppLayoutState({
      sidebarSetting: sidebarType,
      activeSidebar: appLayoutState?.activeSidebar !== "none" ? sidebarType : "none",
    });
  };

  const handleBottomPaneVisibilityChange = (event: React.ChangeEvent<HTMLSelectElement>) => {
    const visibility = event.target.value === "visible";
    bottomPane.setVisibility(visibility);
  };

  const handleActivityBarPositionChange = (event: React.ChangeEvent<HTMLSelectElement>) => {
    const position = event.target.value as ActivityBarState["position"];

    changeActivityBarState({ position });
  };

  return (
    <main className="min-h-screen bg-[var(--moss-page-background)]">
      <div className="p-5">
        <h1 className="mb-5 text-2xl font-bold text-[var(--moss-text)]">{t("settings")}</h1>

        <SettingsDropdown
          id="lang-select"
          label={t("selectLanguage")}
          value={appState?.preferences.locale?.code || appState?.defaults.locale?.code || ""}
          onChange={handleLanguageChange}
        >
          {languages?.map((lang: { code: string; displayName: string }) => (
            <option key={lang.code} value={lang.code} className="text-[var(--moss-select-text-outlined)]">
              {lang.displayName}
            </option>
          ))}
        </SettingsDropdown>

        <SettingsDropdown
          id="theme-select"
          label={t("selectTheme")}
          value={appState?.preferences.theme?.identifier || appState?.defaults.theme?.identifier || ""}
          onChange={handleThemeChange}
        >
          {themes?.map((theme_info: ColorThemeInfo) => (
            <option
              key={theme_info.identifier}
              value={theme_info.identifier}
              className="text-[var(--moss-select-text-outlined)]"
            >
              {theme_info.displayName}
            </option>
          ))}
        </SettingsDropdown>

        <SettingsDropdown
          id="sidebar-type-select"
          label="Sidebar Type"
          value={appLayoutState?.sidebarSetting || "left"}
          onChange={handleSidebarTypeChange}
        >
          <option value="left" className="text-[var(--moss-select-text-outlined)]">
            Left
          </option>
          <option value="right" className="text-[var(--moss-select-text-outlined)]">
            Right
          </option>
        </SettingsDropdown>

        <SettingsDropdown
          id="sidebar-position-select"
          label="Sidebar Visibility"
          value={appLayoutState?.activeSidebar === "none" ? "hidden" : "visible"}
          onChange={(event) => {
            const isVisible = event.target.value === "visible";
            changeAppLayoutState({
              activeSidebar: isVisible ? appLayoutState?.sidebarSetting || "left" : "none",
              sidebarSetting: appLayoutState?.sidebarSetting || "left",
            });
          }}
        >
          <option value="visible" className="text-[var(--moss-select-text-outlined)]">
            Visible
          </option>
          <option value="hidden" className="text-[var(--moss-select-text-outlined)]">
            Hidden
          </option>
        </SettingsDropdown>

        <SettingsDropdown
          id="bottom-pane-select"
          label="Bottom Pane Visibility"
          value={bottomPane.visibility ? "visible" : "hidden"}
          onChange={handleBottomPaneVisibilityChange}
        >
          <option value="visible" className="text-[var(--moss-select-text-outlined)]">
            Visible
          </option>
          <option value="hidden" className="text-[var(--moss-select-text-outlined)]">
            Hidden
          </option>
        </SettingsDropdown>

        <SettingsDropdown
          id="activitybar-position-select"
          label="ActivityBar Position"
          value={activityBarStateData?.position || "default"}
          onChange={handleActivityBarPositionChange}
        >
          <option value="default" className="text-[var(--moss-select-text-outlined)]">
            Default
          </option>
          <option value="top" className="text-[var(--moss-select-text-outlined)]">
            Top
          </option>
          <option value="bottom" className="text-[var(--moss-select-text-outlined)]">
            Bottom
          </option>
          <option value="hidden" className="text-[var(--moss-select-text-outlined)]">
            Hidden
          </option>
        </SettingsDropdown>
      </div>
    </main>
  );
};

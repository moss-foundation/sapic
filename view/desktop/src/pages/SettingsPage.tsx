import React, { ChangeEvent, ReactNode, useState } from "react";
import { useTranslation } from "react-i18next";

import { ActivityBarState } from "@/hooks";
import { useDescribeAppState } from "@/hooks/useDescribeAppState";
import { useListColorThemes } from "@/hooks/useListColorThemes";
import { useListLocales } from "@/hooks/useListLocales";
import { useSetColorTheme } from "@/hooks/useSetColorTheme";
import { useSetLocale } from "@/hooks/useSetLocale";
import { useActivityBarStore } from "@/store/activityBar";
import { useAppResizableLayoutStore } from "@/store/appResizableLayout";
import { ColorThemeInfo } from "@repo/moss-theme";

interface SettingsDropdownProps {
  id: string;
  label: string;
  value: string;
  onChange: (event: ChangeEvent<HTMLSelectElement>) => void;
  children: ReactNode;
}

const SettingsDropdown = ({ id, label, value, onChange, children }: SettingsDropdownProps) => {
  const [isOpen, setIsOpen] = useState(false);

  return (
    <div className="mt-4">
      <h3 className="mb-2 font-medium text-[var(--moss-select-text-outlined)]">{label}</h3>
      <div className="relative">
        <div
          className="background-(--moss-select-bg-outlined) hover:background-(--moss-select-hover-bg) flex w-full cursor-pointer items-center justify-between rounded border border-[var(--moss-select-border-outlined)] p-2 text-[var(--moss-select-text-outlined)] shadow-sm"
          onClick={() => setIsOpen(!isOpen)}
        >
          <span>
            {children instanceof Array
              ? (children as React.ReactElement[]).find((child) => child.props.value === value)?.props.children
              : value}
          </span>
          <span className="ml-2 h-5 w-5 text-[var(--moss-select-text-outlined)]">{isOpen ? "▲" : "▼"}</span>
        </div>

        {isOpen && (
          <div className="background-(--moss-select-bg-outlined) absolute right-0 left-0 z-10 mt-1 max-h-60 overflow-auto rounded-md border border-[var(--moss-select-border-outlined)] shadow-lg">
            <div className="py-1">
              {React.Children.map(children, (child) => {
                if (React.isValidElement(child)) {
                  return (
                    <div
                      className={`hover:background-(--moss-select-hover-bg) cursor-pointer px-4 py-2 ${
                        child.props.value === value ? "background-(--moss-select-hover-bg) font-medium" : ""
                      }`}
                      onClick={() => {
                        onChange({
                          target: { value: child.props.value },
                        } as ChangeEvent<HTMLSelectElement>);
                        setIsOpen(false);
                      }}
                    >
                      {child.props.children}
                    </div>
                  );
                }
                return null;
              })}
            </div>
          </div>
        )}

        <select id={id} className="sr-only" value={value} onChange={onChange}>
          {children}
        </select>
      </div>
    </div>
  );
};

export const Settings = () => {
  const { t } = useTranslation(["ns1", "ns2"]);

  const { data: appState } = useDescribeAppState();
  const { bottomPane, sideBar } = useAppResizableLayoutStore();

  const { data: themes } = useListColorThemes();
  const { mutate: mutateChangeColorTheme } = useSetColorTheme();

  const { data: languages } = useListLocales();
  const { mutate: mutateChangeLanguagePack } = useSetLocale();

  const { setPosition, position } = useActivityBarStore();
  const { setSideBarPosition, sideBarPosition } = useAppResizableLayoutStore();

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
    setSideBarPosition(sidebarType);
  };

  const handleBottomPaneVisibilityChange = (event: React.ChangeEvent<HTMLSelectElement>) => {
    const visibility = event.target.value === "visible";
    bottomPane.setVisible(visibility);
  };

  const handleActivityBarPositionChange = (event: React.ChangeEvent<HTMLSelectElement>) => {
    const position = event.target.value as ActivityBarState["position"];

    setPosition(position);
  };

  return (
    <main className="">
      <div className="p-5">
        <h1 className="mb-5 text-2xl font-bold">{t("settings")}</h1>

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
          value={sideBarPosition || "left"}
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
          value={sideBar.visible ? "visible" : "hidden"}
          onChange={(event) => {
            const isVisible = event.target.value === "visible";

            sideBar.setVisible(isVisible);
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
          value={bottomPane.visible ? "visible" : "hidden"}
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
          value={position || "default"}
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

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
import { ActionMenu, MenuItemProps } from "@/components/ActionMenu/ActionMenu";

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

  // Menu open states
  const [languageMenuOpen, setLanguageMenuOpen] = useState(false);
  const [themeMenuOpen, setThemeMenuOpen] = useState(false);
  const [sidebarTypeMenuOpen, setSidebarTypeMenuOpen] = useState(false);
  const [sidebarVisibilityMenuOpen, setSidebarVisibilityMenuOpen] = useState(false);
  const [bottomPaneMenuOpen, setBottomPaneMenuOpen] = useState(false);
  const [activityBarMenuOpen, setActivityBarMenuOpen] = useState(false);

  const handleLanguageChange = (item: MenuItemProps) => {
    const selectedLocaleCode = item.value;
    const selectedLocaleInfo = languages?.find(
      (lang: { code: string; displayName: string }) => lang.code === selectedLocaleCode
    );
    if (selectedLocaleInfo) {
      mutateChangeLanguagePack({
        localeInfo: selectedLocaleInfo,
      });
    }
  };

  const handleThemeChange = (item: MenuItemProps) => {
    const lastIdentifier = item.value || "";
    // Find the theme that ends with the selected value
    const selectedTheme = themes?.find((theme: { identifier: string; displayName: string }) =>
      theme.identifier.endsWith(lastIdentifier)
    );
    if (selectedTheme) {
      mutateChangeColorTheme({
        themeInfo: selectedTheme,
      });
    }
  };

  const handleSidebarTypeChange = (item: MenuItemProps) => {
    const sidebarType = item.value as "left" | "right";
    setSideBarPosition(sidebarType);
  };

  const handleBottomPaneVisibilityChange = (item: MenuItemProps) => {
    const visibility = item.value === "visible";
    bottomPane.setVisible(visibility);
  };

  const handleActivityBarPositionChange = (item: MenuItemProps) => {
    const position = item.value as ActivityBarState["position"];
    setPosition(position);
  };

  const handleSidebarVisibilityChange = (item: MenuItemProps) => {
    const isVisible = item.value === "visible";
    sideBar.setVisible(isVisible);
  };

  // Convert languages to menu items
  const languageItems: MenuItemProps[] =
    languages?.map((lang: { code: string; displayName: string }) => ({
      id: lang.code,
      type: "radio" as const,
      label: lang.displayName,
      value: lang.code,
    })) || [];

  // Convert themes to menu items
  const themeItems: MenuItemProps[] =
    themes?.map((theme_info: ColorThemeInfo) => {
      // Extract the last part of the identifier (after the last dot or slash)
      const lastIdentifier = theme_info.identifier.split(/[./]/).pop() || theme_info.identifier;

      return {
        id: theme_info.identifier,
        type: "radio" as const,
        label: theme_info.displayName,
        value: lastIdentifier,
      };
    }) || [];

  // Sidebar position items
  const sidebarTypeItems: MenuItemProps[] = [
    {
      id: "sidebar-left",
      type: "radio",
      label: "Left",
      value: "left",
    },
    {
      id: "sidebar-right",
      type: "radio",
      label: "Right",
      value: "right",
    },
  ];

  // Visibility options
  const visibilityItems: MenuItemProps[] = [
    {
      id: "visible",
      type: "radio",
      label: "Visible",
      value: "visible",
    },
    {
      id: "hidden",
      type: "radio",
      label: "Hidden",
      value: "hidden",
    },
  ];

  // Activity bar position items
  const activityBarPositionItems: MenuItemProps[] = [
    {
      id: "default",
      type: "radio",
      label: "Default",
      value: "default",
    },
    {
      id: "top",
      type: "radio",
      label: "Top",
      value: "top",
    },
    {
      id: "bottom",
      type: "radio",
      label: "Bottom",
      value: "bottom",
    },
    {
      id: "hidden",
      type: "radio",
      label: "Hidden",
      value: "hidden",
    },
  ];

  return (
    <main className="">
      <div className="p-5">
        <h1 className="mb-5 text-2xl font-bold">{t("settings")}</h1>

        <div className="mt-4">
          <h3 className="mb-2 font-medium text-[var(--moss-select-text-outlined)]">{t("selectLanguage")}</h3>
          <div className="w-[200px]">
            <ActionMenu
              type="dropdown"
              items={languageItems}
              open={languageMenuOpen}
              onOpenChange={setLanguageMenuOpen}
              onSelect={handleLanguageChange}
              selectedValue={appState?.preferences.locale?.code || appState?.defaults.locale?.code || ""}
              placeholder="Select language"
            />
          </div>
        </div>

        <div className="mt-4">
          <h3 className="mb-2 font-medium text-[var(--moss-select-text-outlined)]">{t("selectTheme")}</h3>
          <div className="w-[200px]">
            <ActionMenu
              type="dropdown"
              items={themeItems}
              open={themeMenuOpen}
              onOpenChange={setThemeMenuOpen}
              onSelect={handleThemeChange}
              selectedValue={(appState?.preferences.theme?.identifier || appState?.defaults.theme?.identifier || "")
                .split(/[./]/)
                .pop()}
              placeholder="Select theme"
            />
          </div>
        </div>

        <div className="mt-4">
          <h3 className="mb-2 font-medium text-[var(--moss-select-text-outlined)]">Sidebar Type</h3>
          <div className="w-[200px]">
            <ActionMenu
              type="dropdown"
              items={sidebarTypeItems}
              open={sidebarTypeMenuOpen}
              onOpenChange={setSidebarTypeMenuOpen}
              onSelect={handleSidebarTypeChange}
              selectedValue={sideBarPosition || "left"}
              placeholder="Select sidebar type"
            />
          </div>
        </div>

        <div className="mt-4">
          <h3 className="mb-2 font-medium text-[var(--moss-select-text-outlined)]">Sidebar Visibility</h3>
          <div className="w-[200px]">
            <ActionMenu
              type="dropdown"
              items={visibilityItems}
              open={sidebarVisibilityMenuOpen}
              onOpenChange={setSidebarVisibilityMenuOpen}
              onSelect={handleSidebarVisibilityChange}
              selectedValue={sideBar.visible ? "visible" : "hidden"}
              placeholder="Select visibility"
            />
          </div>
        </div>

        <div className="mt-4">
          <h3 className="mb-2 font-medium text-[var(--moss-select-text-outlined)]">Bottom Pane Visibility</h3>
          <div className="w-[200px]">
            <ActionMenu
              type="dropdown"
              items={visibilityItems}
              open={bottomPaneMenuOpen}
              onOpenChange={setBottomPaneMenuOpen}
              onSelect={handleBottomPaneVisibilityChange}
              selectedValue={bottomPane.visible ? "visible" : "hidden"}
              placeholder="Select visibility"
            />
          </div>
        </div>

        <div className="mt-4">
          <h3 className="mb-2 font-medium text-[var(--moss-select-text-outlined)]">ActivityBar Position</h3>
          <div className="w-[200px]">
            <ActionMenu
              type="dropdown"
              items={activityBarPositionItems}
              open={activityBarMenuOpen}
              onOpenChange={setActivityBarMenuOpen}
              onSelect={handleActivityBarPositionChange}
              selectedValue={position || "default"}
              placeholder="Select position"
            />
          </div>
        </div>
      </div>
    </main>
  );
};

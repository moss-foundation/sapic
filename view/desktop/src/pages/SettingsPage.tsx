import { useTranslation } from "react-i18next";

import SelectOutlined from "@/components/SelectOutlined";
import { ACTIVITYBAR_POSITION, SIDEBAR_POSITION } from "@/constants/layoutPositions";
import { useDescribeAppState } from "@/hooks/appState/useDescribeAppState";
import { useListColorThemes } from "@/hooks/colorTheme/useListColorThemes";
import { useSetColorTheme } from "@/hooks/colorTheme/useSetColorTheme";
import { useListLocales } from "@/hooks/locales/useListLocales";
import { useSetLocale } from "@/hooks/locales/useSetLocale";
import { useActiveWorkspace } from "@/hooks/workspace/useActiveWorkspace";
import { useActivityBarStore } from "@/store/activityBar";
import { useAppResizableLayoutStore } from "@/store/appResizableLayout";
import { MenuItemProps } from "@/utils/renderActionMenuItem";
import { ColorThemeInfo } from "@repo/moss-app";
import { ActivitybarPosition, SidebarPosition } from "@repo/moss-workspace";

export const Settings = () => {
  const { t } = useTranslation(["ns1", "ns2"]);

  const { data: appState } = useDescribeAppState();
  const workspace = useActiveWorkspace();
  const hasWorkspace = !!workspace;
  const { bottomPane, sideBar } = useAppResizableLayoutStore();

  const { data: themes } = useListColorThemes();
  const { mutate: mutateChangeColorTheme } = useSetColorTheme();

  const { data: languages } = useListLocales();
  const { mutate: mutateChangeLanguagePack } = useSetLocale();

  const { setPosition, position } = useActivityBarStore();
  const { setSideBarPosition, sideBarPosition } = useAppResizableLayoutStore();

  const handleLanguageChange = (value: string) => {
    const selectedLocaleCode = value;
    const selectedLocaleInfo = languages?.find(
      (lang: { code: string; displayName: string }) => lang.code === selectedLocaleCode
    );
    if (selectedLocaleInfo) {
      mutateChangeLanguagePack({
        localeInfo: selectedLocaleInfo,
      });
    }
  };

  const handleThemeChange = (value: string) => {
    const lastIdentifier = value || "";
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

  const handleSidebarTypeChange = (value: string) => {
    const sidebarType = value as SidebarPosition;
    setSideBarPosition(sidebarType);
  };

  const handleBottomPaneVisibilityChange = (value: string) => {
    const visibility = value === "visible";
    bottomPane.setVisible(visibility);
  };

  const handleActivityBarPositionChange = (value: string) => {
    const position = value as ActivitybarPosition;
    setPosition(position);
  };

  const handleSidebarVisibilityChange = (value: string) => {
    const isVisible = value === "visible";
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
      value: SIDEBAR_POSITION.LEFT,
    },
    {
      id: "sidebar-right",
      type: "radio",
      label: "Right",
      value: SIDEBAR_POSITION.RIGHT,
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
      id: ACTIVITYBAR_POSITION.DEFAULT,
      type: "radio",
      label: "Default",
      value: ACTIVITYBAR_POSITION.DEFAULT,
    },
    {
      id: ACTIVITYBAR_POSITION.TOP,
      type: "radio",
      label: "Top",
      value: ACTIVITYBAR_POSITION.TOP,
    },
    {
      id: ACTIVITYBAR_POSITION.BOTTOM,
      type: "radio",
      label: "Bottom",
      value: ACTIVITYBAR_POSITION.BOTTOM,
    },
    {
      id: ACTIVITYBAR_POSITION.HIDDEN,
      type: "radio",
      label: "Hidden",
      value: ACTIVITYBAR_POSITION.HIDDEN,
    },
  ];

  return (
    <div className="space-y-6">
      <div className="mt-4">
        <h3 className="mb-2 font-medium text-[var(--moss-select-text-outlined)]">{t("selectLanguage")}</h3>
        <div className="w-[200px]">
          <SelectOutlined.Root
            value={appState?.preferences.locale?.code || appState?.defaults.locale?.code || ""}
            onValueChange={handleLanguageChange}
          >
            <SelectOutlined.Trigger />
            <SelectOutlined.Content>
              {languageItems.map((item) => {
                if (item.type === "separator") {
                  return <SelectOutlined.Separator key={item.id} />;
                }

                return (
                  <SelectOutlined.Item key={item.id} value={item.value!}>
                    {item.label}
                  </SelectOutlined.Item>
                );
              })}
            </SelectOutlined.Content>
          </SelectOutlined.Root>
        </div>
      </div>

      <div className="mt-4">
        <h3 className="mb-2 font-medium text-[var(--moss-select-text-outlined)]">{t("selectTheme")}</h3>
        <div className="w-[200px]">
          <SelectOutlined.Root
            value={(appState?.preferences.theme?.identifier || appState?.defaults.theme?.identifier || "")
              .split(/[./]/)
              .pop()}
            onValueChange={handleThemeChange}
          >
            <SelectOutlined.Trigger />
            <SelectOutlined.Content>
              {themeItems.map((item) => {
                if (item.type === "separator") {
                  return <SelectOutlined.Separator key={item.id} />;
                }

                return (
                  <SelectOutlined.Item key={item.id} value={item.value!}>
                    {item.label}
                  </SelectOutlined.Item>
                );
              })}
            </SelectOutlined.Content>
          </SelectOutlined.Root>
        </div>
      </div>

      <div className="mt-6">
        <h2 className="mb-4 text-lg font-semibold">Workspace Layout</h2>
        {!hasWorkspace && (
          <div className="mb-4 rounded-md bg-yellow-50 p-3 text-sm text-yellow-800">
            <p>
              Sidebar and panel settings are only available when a workspace is active. These settings are saved per
              workspace.
            </p>
          </div>
        )}

        <div className="mt-4">
          <h3 className="mb-2 font-medium text-[var(--moss-select-text-outlined)]">Sidebar Type</h3>
          <div className="w-[200px]">
            <SelectOutlined.Root
              value={sideBarPosition || SIDEBAR_POSITION.LEFT}
              onValueChange={handleSidebarTypeChange}
              disabled={!hasWorkspace}
            >
              <SelectOutlined.Trigger />
              <SelectOutlined.Content>
                {sidebarTypeItems.map((item) => {
                  if (item.type === "separator") {
                    return <SelectOutlined.Separator key={item.id} />;
                  }

                  return (
                    <SelectOutlined.Item key={item.id} value={item.value!}>
                      {item.label}
                    </SelectOutlined.Item>
                  );
                })}
              </SelectOutlined.Content>
            </SelectOutlined.Root>
          </div>
        </div>
        <div className="mt-4">
          <h3 className="mb-2 font-medium text-[var(--moss-select-text-outlined)]">Sidebar Visibility</h3>
          <div className="w-[200px]">
            <SelectOutlined.Root
              value={sideBar.visible ? "visible" : "hidden"}
              onValueChange={handleSidebarVisibilityChange}
              disabled={!hasWorkspace}
            >
              <SelectOutlined.Trigger />
              <SelectOutlined.Content>
                {visibilityItems.map((item) => {
                  if (item.type === "separator") {
                    return <SelectOutlined.Separator key={item.id} />;
                  }

                  return (
                    <SelectOutlined.Item key={item.id} value={item.value!}>
                      {item.label}
                    </SelectOutlined.Item>
                  );
                })}
              </SelectOutlined.Content>
            </SelectOutlined.Root>
          </div>
        </div>

        <div className="mt-4">
          <h3 className="mb-2 font-medium text-[var(--moss-select-text-outlined)]">Bottom Pane Visibility</h3>
          <div className="w-[200px]">
            <SelectOutlined.Root
              value={bottomPane.visible ? "visible" : "hidden"}
              onValueChange={handleBottomPaneVisibilityChange}
              disabled={!hasWorkspace}
            >
              <SelectOutlined.Trigger />
              <SelectOutlined.Content>
                {visibilityItems.map((item) => {
                  if (item.type === "separator") {
                    return <SelectOutlined.Separator key={item.id} />;
                  }

                  return (
                    <SelectOutlined.Item key={item.id} value={item.value!}>
                      {item.label}
                    </SelectOutlined.Item>
                  );
                })}
              </SelectOutlined.Content>
            </SelectOutlined.Root>
          </div>
        </div>

        <div className="mt-4">
          <h3 className="mb-2 font-medium text-[var(--moss-select-text-outlined)]">ActivityBar Position</h3>
          <div className="w-[200px]">
            <SelectOutlined.Root
              value={position || ACTIVITYBAR_POSITION.DEFAULT}
              onValueChange={handleActivityBarPositionChange}
            >
              <SelectOutlined.Trigger />
              <SelectOutlined.Content>
                {activityBarPositionItems.map((item) => {
                  if (item.type === "separator") {
                    return <SelectOutlined.Separator key={item.id} />;
                  }

                  return (
                    <SelectOutlined.Item key={item.id} value={item.value!}>
                      {item.label}
                    </SelectOutlined.Item>
                  );
                })}
              </SelectOutlined.Content>
            </SelectOutlined.Root>
          </div>
        </div>
      </div>
    </div>
  );
};

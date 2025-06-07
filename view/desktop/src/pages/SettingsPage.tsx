import { useTranslation } from "react-i18next";

import { MenuItemProps } from "@/utils/renderActionMenuItem";
import SelectOutlined from "@/components/SelectOutlined";
import { useDescribeAppState } from "@/hooks/appState/useDescribeAppState";
import { useListColorThemes } from "@/hooks/colorTheme/useListColorThemes";
import { useSetColorTheme } from "@/hooks/colorTheme/useSetColorTheme";
import { useListLocales } from "@/hooks/locales/useListLocales";
import { useSetLocale } from "@/hooks/locales/useSetLocale";
import { useActivityBarStore } from "@/store/activityBar";
import { ActivitybarPosition } from "@repo/moss-workspace";
import { useAppResizableLayoutStore } from "@/store/appResizableLayout";
import { ColorThemeInfo } from "@repo/moss-theme";

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
    const sidebarType = value as "left" | "right";
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
      id: "DEFAULT",
      type: "radio",
      label: "Default",
      value: "DEFAULT",
    },
    {
      id: "TOP",
      type: "radio",
      label: "Top",
      value: "TOP",
    },
    {
      id: "BOTTOM",
      type: "radio",
      label: "Bottom",
      value: "BOTTOM",
    },
    {
      id: "HIDDEN",
      type: "radio",
      label: "Hidden",
      value: "HIDDEN",
    },
  ];

  return (
    <main className="">
      <div className="p-5">
        <h1 className="mb-5 text-2xl font-bold">{t("settings")}</h1>

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

        <div className="mt-4">
          <h3 className="mb-2 font-medium text-[var(--moss-select-text-outlined)]">Sidebar Type</h3>
          <div className="w-[200px]">
            <SelectOutlined.Root value={sideBarPosition || "left"} onValueChange={handleSidebarTypeChange}>
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
            <SelectOutlined.Root value={position || "DEFAULT"} onValueChange={handleActivityBarPositionChange}>
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
    </main>
  );
};

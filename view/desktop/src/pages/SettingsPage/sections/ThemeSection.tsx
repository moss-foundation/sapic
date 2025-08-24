import { useTranslation } from "react-i18next";

import SelectOutlined from "@/components/SelectOutlined";
import { useDescribeAppState, useListColorThemes, useSetColorTheme } from "@/hooks";
import { MenuItemProps } from "@/utils/renderActionMenuItem";
import { ColorThemeInfo } from "@repo/moss-app";

import { Section } from "../Section";

export const ThemeSection = () => {
  const { t } = useTranslation(["ns1", "ns2"]);

  const { data: appState } = useDescribeAppState();
  const { data: themes } = useListColorThemes();

  const { mutate: mutateChangeColorTheme } = useSetColorTheme();

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

  return (
    <Section title={t("selectTheme")}>
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
    </Section>
  );
};

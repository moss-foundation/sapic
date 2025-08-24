import { useTranslation } from "react-i18next";

import SelectOutlined from "@/components/SelectOutlined";
import { useDescribeAppState, useListColorThemes, useSetColorTheme } from "@/hooks";

import { Section } from "../Section";

export const ThemeSection = () => {
  const { t } = useTranslation(["ns1", "ns2"]);

  const { data: appState } = useDescribeAppState();
  const { data: themes } = useListColorThemes();

  const { mutate: mutateChangeColorTheme } = useSetColorTheme();

  const handleThemeChange = (newIdentifier: string) => {
    const selectedTheme = themes?.find((theme) => theme.identifier === newIdentifier);

    if (selectedTheme) {
      mutateChangeColorTheme({
        themeInfo: selectedTheme,
      });
    }
  };

  const defaultTheme = appState?.preferences.theme?.identifier || appState?.defaults.theme?.identifier || "";

  return (
    <Section title={t("selectTheme")}>
      <SelectOutlined.Root value={defaultTheme} onValueChange={handleThemeChange}>
        <SelectOutlined.Trigger />

        <SelectOutlined.Content>
          {themes?.map((item) => {
            return (
              <SelectOutlined.Item key={item.identifier} value={item.identifier}>
                {item.displayName}
              </SelectOutlined.Item>
            );
          })}
        </SelectOutlined.Content>
      </SelectOutlined.Root>
    </Section>
  );
};

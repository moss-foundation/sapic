import { useTranslation } from "react-i18next";

import SelectOutlined from "@/components/SelectOutlined";
import { useListColorThemes, useSetColorTheme } from "@/hooks";
import { useDescribeApp } from "@/hooks/useDescribeApp";

import { Section } from "../Section";

export const ThemeSection = () => {
  const { t } = useTranslation(["ns1", "ns2"]);

  const { data: appState } = useDescribeApp();
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

  const defaultTheme = appState?.configuration.contents.colorTheme as string;

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

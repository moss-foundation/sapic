import { useTranslation } from "react-i18next";

import SelectOutlined from "@/components/SelectOutlined";
import { useListColorThemes } from "@/hooks";
import { useDescribeApp } from "@/hooks/app/useDescribeApp";
import { useUpdateConfiguration } from "@/hooks/useUpdateConfiguration";

import { Section } from "../Section";

export const ThemeSection = () => {
  const { t } = useTranslation(["main", "bootstrap"]);

  const { data: appState } = useDescribeApp();
  const { data: themes } = useListColorThemes();
  const { mutate: updateConfiguration } = useUpdateConfiguration();

  const handleThemeChange = (newIdentifier: string) => {
    const selectedTheme = themes?.find((theme) => theme.identifier === newIdentifier);
    if (selectedTheme) {
      updateConfiguration({
        key: "colorTheme",
        value: selectedTheme.identifier,
        target: "PROFILE",
      });
    }
  };

  const currentThemeId = appState?.configuration.contents.colorTheme as string;

  return (
    <Section title={t("selectTheme")}>
      <SelectOutlined.Root value={currentThemeId} onValueChange={handleThemeChange}>
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

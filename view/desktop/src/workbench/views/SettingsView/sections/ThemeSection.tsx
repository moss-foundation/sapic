import { useTranslation } from "react-i18next";

import { useListColorThemes } from "@/hooks";
import { useGetSettings } from "@/hooks/app/settings/useGetSettings";
import { useUpdateSettingsValue } from "@/hooks/app/settings/useUpdateSettingsValue";
import SelectOutlined from "@/workbench/ui/components/SelectOutlined";

import { Section } from "../Section";

export const ThemeSection = () => {
  const { t } = useTranslation(["main", "bootstrap"]);

  const { data: settings } = useGetSettings<{ colorTheme: string }>(["colorTheme"]);
  const { data: themes } = useListColorThemes();
  const { mutate: updateSettingsValue } = useUpdateSettingsValue<string>();

  const handleThemeChange = (newIdentifier: string) => {
    const selectedTheme = themes?.find((theme) => theme.identifier === newIdentifier);
    if (selectedTheme) {
      updateSettingsValue({ key: "colorTheme", value: selectedTheme.identifier });
    }
  };

  const currentThemeId = settings?.colorTheme || "";

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

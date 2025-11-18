import { useTranslation } from "react-i18next";

import i18next from "@/app/i18n";
import { useListLanguages } from "@/hooks";
import { useGetSettings } from "@/hooks/app/settings/useGetSettings";
import { useUpdateSettingsValue } from "@/hooks/app/settings/useUpdateSettingsValue";
import SelectOutlined from "@/workbench/ui/components/SelectOutlined";

import { Section } from "../Section";

export const LanguageSection = () => {
  const { t } = useTranslation(["main", "bootstrap"]);

  const { data: settings } = useGetSettings<{ language: string }>(["language"]);
  const { mutate: updateSettingsValue } = useUpdateSettingsValue<string>();

  const { data: languages } = useListLanguages();

  const handleLanguageChange = (newCode: string) => {
    newCode = newCode === "default" ? "en" : newCode;

    updateSettingsValue({ key: "language", value: newCode });
    i18next.changeLanguage(newCode).catch(console.error);
  };

  const currentLanguage = settings?.language || "en";

  return (
    <Section title={t("selectLanguage")}>
      <SelectOutlined.Root value={currentLanguage} onValueChange={handleLanguageChange}>
        <SelectOutlined.Trigger />
        <SelectOutlined.Content>
          <SelectOutlined.Item key="en" value="en">
            Default
          </SelectOutlined.Item>

          <SelectOutlined.Separator />

          {languages?.map((item) => {
            return (
              <SelectOutlined.Item key={item.code} value={item.code}>
                {item.displayName}
              </SelectOutlined.Item>
            );
          })}
        </SelectOutlined.Content>
      </SelectOutlined.Root>
    </Section>
  );
};

import { useTranslation } from "react-i18next";

import { useGetBatchValue } from "@/adapters";
import { useListLanguages } from "@/adapters/tanstackQuery/language/useListLanguages";
import { useUpdateSettingsValue } from "@/adapters/tanstackQuery/settingsStorage/useUpdateValue";
import i18next from "@/app/i18n";
import SelectOutlined from "@/workbench/ui/components/SelectOutlined";

import { Section } from "../Section";

export const LanguageSection = () => {
  const { t } = useTranslation(["main", "bootstrap"]);

  const { data: settings } = useGetBatchValue<{ language: string }>(["language"]);
  const { mutate: updateSettingsValue } = useUpdateSettingsValue();

  const { data: languages } = useListLanguages();

  const handleLanguageChange = (newCode: string) => {
    newCode = newCode === "default" ? "en" : newCode;

    updateSettingsValue({ key: "language", value: newCode });
    i18next.changeLanguage(newCode).catch(console.error);
  };

  const currentLanguage = settings?.language ?? "";

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

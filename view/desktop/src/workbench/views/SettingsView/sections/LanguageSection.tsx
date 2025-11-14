import { useTranslation } from "react-i18next";

import i18next from "@/app/i18n";
import { useListLanguages } from "@/hooks";
import { useDescribeApp } from "@/hooks/app/useDescribeApp";
import { useUpdateConfiguration } from "@/hooks/useUpdateConfiguration";
import SelectOutlined from "@/workbench/ui/components/SelectOutlined";

import { Section } from "../Section";

export const LanguageSection = () => {
  const { t } = useTranslation(["main", "bootstrap"]);

  const { data: appState } = useDescribeApp();
  const { data: languages } = useListLanguages();
  const { mutate: updateConfiguration } = useUpdateConfiguration();

  const handleLanguageChange = (newCode: string) => {
    updateConfiguration({
      key: "language",
      value: newCode === "default" ? "en" : newCode,
      target: "PROFILE",
    });

    i18next.changeLanguage(newCode).catch(console.error);
  };

  const currentLanguage = appState?.configuration.contents.language as string;

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

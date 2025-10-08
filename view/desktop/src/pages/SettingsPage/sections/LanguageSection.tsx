import { useTranslation } from "react-i18next";

import SelectOutlined from "@/components/SelectOutlined";
import { useListLocales, useSetLocale } from "@/hooks";
import { useDescribeApp } from "@/hooks/app/useDescribeApp";

import { Section } from "../Section";

export const LanguageSection = () => {
  const { t } = useTranslation(["main", "bootstrap"]);

  const { data: appState } = useDescribeApp();
  const { data: languages } = useListLocales();
  const { mutate: mutateSetLocalePack } = useSetLocale();

  const handleLanguageChange = (newCode: string) => {
    const selectedLocaleInfo = languages?.find((lang) => lang.identifier === newCode);

    if (selectedLocaleInfo) {
      mutateSetLocalePack({
        localeInfo: selectedLocaleInfo,
      });
    }
  };

  const currentLanguage = appState?.configuration.contents.locale as string;

  return (
    <Section title={t("selectLanguage")}>
      <SelectOutlined.Root value={currentLanguage} onValueChange={handleLanguageChange}>
        <SelectOutlined.Trigger />
        <SelectOutlined.Content>
          {languages?.map((item) => {
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

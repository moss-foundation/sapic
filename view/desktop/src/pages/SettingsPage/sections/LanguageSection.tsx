import { useTranslation } from "react-i18next";

import SelectOutlined from "@/components/SelectOutlined";
import { useDescribeAppState, useListLocales, useSetLocale } from "@/hooks";

import { Section } from "../Section";

export const LanguageSection = () => {
  const { t } = useTranslation(["ns1", "ns2"]);

  const { data: appState } = useDescribeAppState();
  const { data: languages } = useListLocales();
  const { mutate: mutateChangeLanguagePack } = useSetLocale();

  const handleLanguageChange = (newCode: string) => {
    const selectedLocaleInfo = languages?.find((lang: { code: string; displayName: string }) => lang.code === newCode);

    if (selectedLocaleInfo) {
      mutateChangeLanguagePack({
        localeInfo: selectedLocaleInfo,
      });
    }
  };

  const defaultLanguage = appState?.preferences.locale?.code || appState?.defaults.locale?.code || "";

  return (
    <Section title={t("selectLanguage")}>
      <SelectOutlined.Root value={defaultLanguage} onValueChange={handleLanguageChange}>
        <SelectOutlined.Trigger />
        <SelectOutlined.Content>
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

import { useTranslation } from "react-i18next";

import SelectOutlined from "@/components/SelectOutlined";
import { useDescribeAppState, useListLocales, useSetLocale } from "@/hooks";
import { MenuItemProps } from "@/utils/renderActionMenuItem";

import { Section } from "../Section";

export const LanguageSection = () => {
  const { t } = useTranslation(["ns1", "ns2"]);

  const { data: appState } = useDescribeAppState();
  const { data: languages } = useListLocales();
  const { mutate: mutateChangeLanguagePack } = useSetLocale();

  const languageItems: MenuItemProps[] =
    languages?.map((lang: { code: string; displayName: string }) => ({
      id: lang.code,
      type: "radio" as const,
      label: lang.displayName,
      value: lang.code,
    })) || [];

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
          {languageItems.map((item) => {
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

import { useTranslation } from "react-i18next";

import SelectOutlined from "@/components/SelectOutlined";
import { useDescribeAppState, useListLocales, useSetLocale } from "@/hooks";
import { MenuItemProps } from "@/utils/renderActionMenuItem";

import { Section } from "../Section";

export const LanguageSection = () => {
  const { t } = useTranslation(["ns1", "ns2"]);
  const { data: languages } = useListLocales();
  const { mutate: mutateChangeLanguagePack } = useSetLocale();
  const { data: appState } = useDescribeAppState();

  const languageItems: MenuItemProps[] =
    languages?.map((lang: { code: string; displayName: string }) => ({
      id: lang.code,
      type: "radio" as const,
      label: lang.displayName,
      value: lang.code,
    })) || [];

  const handleLanguageChange = (value: string) => {
    const selectedLocaleCode = value;
    const selectedLocaleInfo = languages?.find(
      (lang: { code: string; displayName: string }) => lang.code === selectedLocaleCode
    );
    if (selectedLocaleInfo) {
      mutateChangeLanguagePack({
        localeInfo: selectedLocaleInfo,
      });
    }
  };

  return (
    <Section title={t("selectLanguage")}>
      <SelectOutlined.Root
        value={appState?.preferences.locale?.code || appState?.defaults.locale?.code || ""}
        onValueChange={handleLanguageChange}
      >
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

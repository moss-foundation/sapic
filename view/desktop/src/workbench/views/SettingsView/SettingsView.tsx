import { PageContent } from "@/workbench/ui/components";
import { DefaultViewProps } from "@/workbench/ui/parts/TabbedPane/types";

import { AppLayoutSection } from "./sections/AppLayoutSection";
import { LanguageSection } from "./sections/LanguageSection";
import { ThemeSection } from "./sections/ThemeSection";

export type SettingsViewProps = DefaultViewProps;

export const SettingsView = ({}: SettingsViewProps) => {
  return (
    <PageContent className="flex flex-col gap-4">
      <LanguageSection />
      <ThemeSection />
      <AppLayoutSection />
    </PageContent>
  );
};

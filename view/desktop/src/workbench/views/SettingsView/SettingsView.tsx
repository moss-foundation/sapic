import { PageContent } from "@/workbench/ui/components";

import { AppLayoutSection } from "./sections/AppLayoutSection";
import { LanguageSection } from "./sections/LanguageSection";
import { ThemeSection } from "./sections/ThemeSection";

export const SettingsView = () => {
  return (
    <PageContent className="flex flex-col gap-4">
      <LanguageSection />
      <ThemeSection />
      <AppLayoutSection />
    </PageContent>
  );
};

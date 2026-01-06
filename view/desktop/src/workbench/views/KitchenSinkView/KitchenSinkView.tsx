import { PageContent } from "@/workbench/ui/components";
import { DefaultViewProps } from "@/workbench/ui/parts/TabbedPane/types";

import { AccentSection } from "./Sections/AccentSection";
import { ActionMenusSection } from "./Sections/ActionMenusSection";
import { ButtonsSection } from "./Sections/ButtonsSection";
import { CheckboxSection } from "./Sections/CheckboxSection";
import { CommandSection } from "./Sections/CommandSection";
import { IconsSection } from "./Sections/IconsSections";
import { InputsSection } from "./Sections/InputsSection";
import { InputTemplatingSection } from "./Sections/InputTemplatingSection";
import { NotificationsSection } from "./Sections/NotificationsSection";
import { RadioSection } from "./Sections/RadioSection";
import { SelectSection } from "./Sections/SelectSection";
import { TabsSection } from "./Sections/TabsSection";
import { ToggleSection } from "./Sections/ToggleSection";

export type KitchenSinkViewProps = DefaultViewProps;

export const KitchenSinkView = ({}: KitchenSinkViewProps) => {
  return (
    <PageContent className="mx-auto flex max-w-6xl flex-col gap-10">
      <AccentSection />

      <ButtonsSection />

      <SelectSection />

      <InputsSection />

      <RadioSection />

      <ToggleSection />

      <CheckboxSection />

      <TabsSection />

      <ActionMenusSection />

      <NotificationsSection />

      <InputTemplatingSection />

      <CommandSection />

      <IconsSection />
    </PageContent>
  );
};

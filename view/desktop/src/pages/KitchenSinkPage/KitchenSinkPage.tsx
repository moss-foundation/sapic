import { ActionMenusSection } from "./Sections/ActionMenusSection";
import { ButtonsSection } from "./Sections/ButtonsSection";
import { CommandSection } from "./Sections/CommandSection";
import { IconsSection } from "./Sections/IconsSections";
import { InputTemplatingSection } from "./Sections/InputTemplatingSection";
import { TableSection } from "./Sections/TableSection";
import { TabsSection } from "./Sections/TabsSection";

export const KitchenSink = () => {
  return (
    <div className="mx-auto flex max-w-6xl flex-col gap-10">
      <TabsSection />

      <TableSection />

      <ActionMenusSection />

      <ButtonsSection />

      <InputTemplatingSection />

      <CommandSection />

      <IconsSection />
    </div>
  );
};

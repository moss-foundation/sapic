import { PageContent } from "@/components";

import { ActionMenusSection } from "./Sections/ActionMenusSection";
import { ButtonsSection } from "./Sections/ButtonsSection";
import { CommandSection } from "./Sections/CommandSection";
import { GlobalNotificationsTestSection } from "./Sections/GlobalNotificationsTestSection";
import { IconsSection } from "./Sections/IconsSections";
import { InputTemplatingSection } from "./Sections/InputTemplatingSection";
import { NotificationsSection } from "./Sections/NotificationsSection";
import { TableSection } from "./Sections/TableSection";
import { TabsSection } from "./Sections/TabsSection";

export const KitchenSink = () => {
  return (
    <PageContent className="mx-auto flex max-w-6xl flex-col gap-10">
      <GlobalNotificationsTestSection />

      <TabsSection />

      <TableSection />

      <ActionMenusSection />

      <ButtonsSection />

      <NotificationsSection />

      <InputTemplatingSection />

      <CommandSection />

      <IconsSection />
    </PageContent>
  );
};

import { useState } from "react";

import PaddedTabs from "@/components/PaddedTabs/PaddedTabs";

import { KitchenSinkSection } from "../KitchenSinkSection";

export const TabsSection = () => {
  const [value, setValue] = useState("tab1");

  return (
    <KitchenSinkSection header="Tabs" description="A section for tabs">
      <PaddedTabs.Root value={value} onValueChange={setValue}>
        <PaddedTabs.List>
          <PaddedTabs.Trigger value="tab1">Create</PaddedTabs.Trigger>
          <PaddedTabs.Trigger value="tab2">Import</PaddedTabs.Trigger>
          <PaddedTabs.Trigger value="tab3">Tab 3</PaddedTabs.Trigger>
        </PaddedTabs.List>

        <PaddedTabs.Content value="tab1">
          <div>Create</div>
        </PaddedTabs.Content>
        <PaddedTabs.Content value="tab2">
          <div>Import</div>
        </PaddedTabs.Content>
        <PaddedTabs.Content value="tab3">
          <div>Tab 3 content</div>
        </PaddedTabs.Content>
      </PaddedTabs.Root>
    </KitchenSinkSection>
  );
};

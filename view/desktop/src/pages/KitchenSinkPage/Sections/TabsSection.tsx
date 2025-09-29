import { useState } from "react";

import { OutlinedTabs, PaddedTabs, PageContainerTabs } from "@/components";

import { KitchenSinkSection } from "../KitchenSinkSection";
import { KitchenSinkSectionSubHeader } from "../KitchenSinkSectionSubHeader";

export const TabsSection = () => {
  const [value, setValue] = useState("tab1");

  return (
    <KitchenSinkSection header="Tabs" description="A section for tabs">
      <KitchenSinkSectionSubHeader>PaddedTabs</KitchenSinkSectionSubHeader>
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

      <KitchenSinkSectionSubHeader>OutlinedTabs</KitchenSinkSectionSubHeader>
      <OutlinedTabs.Root value={value} onValueChange={setValue}>
        <OutlinedTabs.List>
          <OutlinedTabs.Trigger value="tab1">Create</OutlinedTabs.Trigger>
          <OutlinedTabs.Trigger value="tab2">Import</OutlinedTabs.Trigger>
          <OutlinedTabs.Trigger value="tab3">Tab 3</OutlinedTabs.Trigger>
        </OutlinedTabs.List>

        <OutlinedTabs.Content value="tab1">
          <div>Create</div>
        </OutlinedTabs.Content>
        <OutlinedTabs.Content value="tab2">
          <div>Import</div>
        </OutlinedTabs.Content>
        <OutlinedTabs.Content value="tab3">
          <div>Tab 3 content</div>
        </OutlinedTabs.Content>
      </OutlinedTabs.Root>

      <KitchenSinkSectionSubHeader>PageContainerTabs</KitchenSinkSectionSubHeader>
      <PageContainerTabs.Root value={value} onValueChange={setValue}>
        <PageContainerTabs.List>
          <PageContainerTabs.Trigger value="tab1">Create</PageContainerTabs.Trigger>
          <PageContainerTabs.Trigger value="tab2">Import</PageContainerTabs.Trigger>
          <PageContainerTabs.Trigger value="tab3">Tab 3</PageContainerTabs.Trigger>
        </PageContainerTabs.List>

        <PageContainerTabs.Content value="tab1">
          <div>Create</div>
        </PageContainerTabs.Content>
        <PageContainerTabs.Content value="tab2">
          <div>Import</div>
        </PageContainerTabs.Content>
        <PageContainerTabs.Content value="tab3">
          <div>Tab 3 content</div>
        </PageContainerTabs.Content>
      </PageContainerTabs.Root>
    </KitchenSinkSection>
  );
};

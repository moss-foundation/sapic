import { useState } from "react";

import { OutlinedTabs, PaddedTabs, PageContainerTabs, ProviderTabs } from "@/components";

import { KitchenSinkSection } from "../KitchenSinkSection";
import { KitchenSinkSectionSubHeader } from "../KitchenSinkSectionSubHeader";

export const TabsSection = () => {
  const [value, setValue] = useState("tab1");

  return (
    <KitchenSinkSection header="Tabs">
      <PaddedTabsSection value={value} setValue={setValue} />

      <OutlinedTabsSection value={value} setValue={setValue} />

      <PageContainerTabsSection value={value} setValue={setValue} />

      <ProviderTabsSection />
    </KitchenSinkSection>
  );
};

const PaddedTabsSection = ({ value, setValue }: { value: string; setValue: (value: string) => void }) => {
  return (
    <>
      <KitchenSinkSectionSubHeader>Padded Tabs</KitchenSinkSectionSubHeader>
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
    </>
  );
};

const OutlinedTabsSection = ({ value, setValue }: { value: string; setValue: (value: string) => void }) => {
  return (
    <>
      <KitchenSinkSectionSubHeader>Outlined Tabs</KitchenSinkSectionSubHeader>

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
    </>
  );
};

const PageContainerTabsSection = ({ value, setValue }: { value: string; setValue: (value: string) => void }) => {
  return (
    <>
      <KitchenSinkSectionSubHeader>Page Container Tabs</KitchenSinkSectionSubHeader>
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
    </>
  );
};

const ProviderTabsSection = () => {
  const [value, setValue] = useState("tab1");

  return (
    <>
      <KitchenSinkSectionSubHeader>Provider Tabs</KitchenSinkSectionSubHeader>
      <ProviderTabs.Root value={value} onValueChange={setValue}>
        <ProviderTabs.List className="flex gap-2">
          <ProviderTabs.Trigger value="tab1" icon="github" label="GitHub" />
          <ProviderTabs.Trigger value="tab2" icon="gitlab" label="GitLab" />
          <ProviderTabs.Trigger value="tab3" icon="postman" label="Postman" />
          <ProviderTabs.Trigger value="tab4" icon="insomnia" label="Insomnia" />
        </ProviderTabs.List>

        <ProviderTabs.Content value="tab1">
          <div>Create</div>
        </ProviderTabs.Content>
        <ProviderTabs.Content value="tab2">
          <div>Import</div>
        </ProviderTabs.Content>
        <ProviderTabs.Content value="tab3">
          <div>Tab 3 content</div>
        </ProviderTabs.Content>
        <ProviderTabs.Content value="tab4">
          <div>Tab 4 content</div>
        </ProviderTabs.Content>
      </ProviderTabs.Root>
    </>
  );
};

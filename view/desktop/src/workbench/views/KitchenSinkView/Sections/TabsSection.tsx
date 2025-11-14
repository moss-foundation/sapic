import { useState } from "react";

import { FolderTabs, FramedTabs, Icon, PillTabs, UnderlinedTabs } from "@/lib/ui";
import { ProviderIcon } from "@/workbench/ui/components/ProviderIcon";

import { KitchenSinkSection } from "../KitchenSinkSection";
import { KitchenSinkSectionSubHeader } from "../KitchenSinkSectionSubHeader";

export const TabsSection = () => {
  const [value, setValue] = useState("tab1");

  return (
    <KitchenSinkSection header="Tabs">
      <UnderlinedTabsSection value={value} setValue={setValue} />

      <FramedTabsSection value={value} setValue={setValue} />

      <FolderTabsSection value={value} setValue={setValue} />

      <PillTabsSection />
    </KitchenSinkSection>
  );
};

const UnderlinedTabsSection = ({ value, setValue }: { value: string; setValue: (value: string) => void }) => {
  return (
    <>
      <KitchenSinkSectionSubHeader>Underlined Tabs</KitchenSinkSectionSubHeader>
      <UnderlinedTabs.Root value={value} onValueChange={setValue}>
        <UnderlinedTabs.List>
          <UnderlinedTabs.Trigger value="tab1">Create</UnderlinedTabs.Trigger>
          <UnderlinedTabs.Trigger value="tab2">Import</UnderlinedTabs.Trigger>
          <UnderlinedTabs.Trigger value="tab3">Tab 3</UnderlinedTabs.Trigger>
        </UnderlinedTabs.List>

        <UnderlinedTabs.Content value="tab1">
          <div>Create</div>
        </UnderlinedTabs.Content>
        <UnderlinedTabs.Content value="tab2">
          <div>Import</div>
        </UnderlinedTabs.Content>
        <UnderlinedTabs.Content value="tab3">
          <div>Tab 3 content</div>
        </UnderlinedTabs.Content>
      </UnderlinedTabs.Root>
    </>
  );
};

const FramedTabsSection = ({ value, setValue }: { value: string; setValue: (value: string) => void }) => {
  return (
    <>
      <KitchenSinkSectionSubHeader>Framed Tabs</KitchenSinkSectionSubHeader>

      <FramedTabs.Root value={value} onValueChange={setValue}>
        <FramedTabs.List>
          <FramedTabs.Trigger value="tab1">Create</FramedTabs.Trigger>
          <FramedTabs.Trigger value="tab2">Import</FramedTabs.Trigger>
          <FramedTabs.Trigger value="tab3">Tab 3</FramedTabs.Trigger>
        </FramedTabs.List>

        <FramedTabs.Content value="tab1">
          <div>Create</div>
        </FramedTabs.Content>
        <FramedTabs.Content value="tab2">
          <div>Import</div>
        </FramedTabs.Content>
        <FramedTabs.Content value="tab3">
          <div>Tab 3 content</div>
        </FramedTabs.Content>
      </FramedTabs.Root>
    </>
  );
};

const FolderTabsSection = ({ value, setValue }: { value: string; setValue: (value: string) => void }) => {
  return (
    <>
      <KitchenSinkSectionSubHeader>Folder Tabs</KitchenSinkSectionSubHeader>
      <FolderTabs.Root value={value} onValueChange={setValue}>
        <FolderTabs.List>
          <FolderTabs.Trigger value="tab1">Create</FolderTabs.Trigger>
          <FolderTabs.Trigger value="tab2">Import</FolderTabs.Trigger>
          <FolderTabs.Trigger value="tab3">Tab 3</FolderTabs.Trigger>
        </FolderTabs.List>

        <FolderTabs.Content value="tab1">
          <div>Create</div>
        </FolderTabs.Content>
        <FolderTabs.Content value="tab2">
          <div>Import</div>
        </FolderTabs.Content>
        <FolderTabs.Content value="tab3">
          <div>Tab 3 content</div>
        </FolderTabs.Content>
      </FolderTabs.Root>
    </>
  );
};

const PillTabsSection = () => {
  const [value, setValue] = useState("tab1");

  return (
    <>
      <KitchenSinkSectionSubHeader>Pill Tabs</KitchenSinkSectionSubHeader>
      <PillTabs.Root value={value} onValueChange={setValue}>
        <PillTabs.List className="flex gap-2">
          <PillTabs.Trigger value="tab1" leadingContent={<ProviderIcon icon="github" />} label="GitHub" />
          <PillTabs.Trigger value="tab2" leadingContent={<ProviderIcon icon="gitlab" />} label="GitLab" />
          <PillTabs.Trigger value="tab3" leadingContent={<ProviderIcon icon="postman" />} label="Postman" />
          <PillTabs.Trigger value="tab4" leadingContent={<ProviderIcon icon="insomnia" />} label="Insomnia" />
          <PillTabs.Trigger value="tab5" trailingContent={<Icon icon="Auth" />} label="Auth" />
          <PillTabs.Trigger
            value="tab6"
            leadingContent={<Icon icon="Http" />}
            trailingContent={<Icon icon="Send" />}
            label="Send"
          />
        </PillTabs.List>

        {Array.from({ length: 6 }).map((_, index) => (
          <PillTabs.Content key={index} value={`tab${index + 1}`}>
            <div>Tab {index + 1} content</div>
          </PillTabs.Content>
        ))}
      </PillTabs.Root>
    </>
  );
};

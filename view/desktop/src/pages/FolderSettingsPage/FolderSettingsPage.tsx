import { IDockviewPanelProps } from "moss-tabs";
import { useState } from "react";

import { PageHeader, PageView } from "@/components";
import { PageWrapper } from "@/components/PageView/PageWrapper";
import { ProjectTreeNode } from "@/components/ProjectTree/types";
import { useStreamProjectEntries } from "@/hooks/project/useStreamProjectEntries";
import { useRenameEntryForm } from "@/hooks/useRenameEntryForm";
import { FolderTabs, Icon, TabItemProps } from "@/lib/ui";
import { EntryKind } from "@repo/moss-project";

import { OverviewTabContent } from "./tabs/OverviewTabContent";
import { getFolderIcon } from "./utils";

export interface FolderSettingsParams {
  projectId: string;
  node: ProjectTreeNode;
  iconType: EntryKind;
}

export const FolderSettings = ({ ...props }: IDockviewPanelProps<FolderSettingsParams>) => {
  const { data: streamedEntries } = useStreamProjectEntries(props.params?.projectId);
  const node = streamedEntries?.find((entry) => entry.id === props.params?.node?.id);

  const { isRenamingEntry, setIsRenamingEntry, handleRenamingEntrySubmit, handleRenamingEntryCancel } =
    useRenameEntryForm(props?.params?.node, props?.params?.projectId);

  const [activeTabId, setActiveTabId] = useState("overview");

  if (!props?.params?.projectId || !node) {
    return (
      <div className="flex h-full items-center justify-center text-(--moss-primary-text)">
        <div className="text-center">
          <h2 className="text-lg font-semibold">No Folder Selected</h2>
          <p className="text-sm">Please select a folder to view its settings.</p>
        </div>
      </div>
    );
  }

  const tabs: TabItemProps[] = [
    {
      id: "overview",
      label: (
        <div className="flex items-center gap-1">
          <Icon icon="Folder" className="h-4 w-4" />
          <span>Overview</span>
        </div>
      ),
      content: <OverviewTabContent {...props} />,
    },
    {
      id: "permissions",
      label: (
        <div className="flex items-center gap-1">
          <Icon icon="Auth" className="h-4 w-4" />
          <span>Permissions</span>
        </div>
      ),
      content: (
        <div className="p-4">
          <div className="rounded-lg border border-(--moss-border-color) p-4">
            <h3 className="mb-3 text-lg font-semibold text-(--moss-primary-text)">Folder Permissions</h3>
            <p className="text-(--moss-secondary-text)">Folder permissions configuration will be implemented here.</p>
            <div className="background-(--moss-secondary-background) mt-4 rounded p-3">
              <p className="text-sm text-(--moss-secondary-text)">
                This is a placeholder for folder-specific permissions and access control settings.
              </p>
            </div>
          </div>
        </div>
      ),
    },
    {
      id: "settings",
      label: (
        <div className="flex items-center gap-1">
          <Icon icon="Settings" className="h-4 w-4" />
          <span>Settings</span>
        </div>
      ),
      content: (
        <div className="p-4">
          <div className="rounded-lg border border-(--moss-border-color) p-4">
            <h3 className="mb-3 text-lg font-semibold text-(--moss-primary-text)">Folder Settings</h3>
            <p className="text-(--moss-secondary-text)">Advanced folder settings will be implemented here.</p>
            <div className="mt-4 space-y-4">
              <div className="background-(--moss-secondary-background) rounded p-3">
                <h4 className="mb-2 font-medium text-(--moss-primary-text)">Display Options</h4>
                <p className="text-sm text-(--moss-secondary-text)">
                  Settings for how this folder and its contents are displayed in the tree.
                </p>
              </div>
              <div className="background-(--moss-secondary-background) rounded p-3">
                <h4 className="mb-2 font-medium text-(--moss-primary-text)">Organization</h4>
                <p className="text-sm text-(--moss-secondary-text)">
                  Settings for organizing and sorting items within this folder.
                </p>
              </div>
            </div>
          </div>
        </div>
      ),
    },
  ];

  const isRoot = node.path.segments.length === 1;

  return (
    <PageView>
      <PageHeader
        icon={getFolderIcon()}
        title={node?.name}
        disableTitleChange={isRoot}
        isRenamingTitle={isRenamingEntry}
        setIsRenamingTitle={setIsRenamingEntry}
        handleRenamingFormCancel={handleRenamingEntryCancel}
        onTitleChange={handleRenamingEntrySubmit}
        {...props}
      />
      <PageWrapper>
        <FolderTabs.Root value={activeTabId} onValueChange={setActiveTabId}>
          <FolderTabs.List>
            {tabs.map((tab) => (
              <FolderTabs.Trigger key={tab.id} value={tab.id}>
                {tab.label}
              </FolderTabs.Trigger>
            ))}
          </FolderTabs.List>

          {tabs.map((tab) => (
            <FolderTabs.Content key={tab.id} value={tab.id}>
              {tab.content}
            </FolderTabs.Content>
          ))}
        </FolderTabs.Root>
      </PageWrapper>
    </PageView>
  );
};

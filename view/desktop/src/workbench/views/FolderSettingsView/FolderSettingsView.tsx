import { IDockviewPanelProps } from "moss-tabs";
import { useState } from "react";

import { FolderTabs, Icon, TabItemProps } from "@/lib/ui";
import { useStreamProjectResources } from "@/workbench/adapters/tanstackQuery/project/useStreamProjectResources";
import { useRenameResourceForm } from "@/workbench/hooks/useRenameResourceForm";
import { PageHeader, PageView } from "@/workbench/ui/components";
import { PageWrapper } from "@/workbench/ui/components/PageView/PageWrapper";
import { ProjectTreeNode } from "@/workbench/ui/components/ProjectTree/types";
import { ResourceKind } from "@repo/moss-project";

import { OverviewTabContent } from "./tabs/OverviewTabContent";
import { getFolderIcon } from "./utils";

export interface FolderSettingsViewParams {
  projectId: string;
  node: ProjectTreeNode;
  iconType: ResourceKind;
}

export const FolderSettingsView = ({ ...props }: IDockviewPanelProps<FolderSettingsViewParams>) => {
  const { data: streamedResources } = useStreamProjectResources(props.params?.projectId);
  const node = streamedResources?.find((resource) => resource.id === props.params?.node?.id);

  const { isRenamingResource, setIsRenamingResource, handleRenamingResourceSubmit, handleRenamingResourceCancel } =
    useRenameResourceForm(props?.params?.node, props?.params?.projectId);

  const [activeTabId, setActiveTabId] = useState("overview");

  if (!props?.params?.projectId || !node) {
    return (
      <div className="text-(--moss-primary-foreground) flex h-full items-center justify-center">
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
          <div className="border-(--moss-border) rounded-lg border p-4">
            <h3 className="text-(--moss-primary-foreground) mb-3 text-lg font-semibold">Folder Permissions</h3>
            <p className="text-(--moss-secondary-foreground)">
              Folder permissions configuration will be implemented here.
            </p>
            <div className="background-(--moss-secondary-background) mt-4 rounded p-3">
              <p className="text-(--moss-secondary-foreground) text-sm">
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
          <div className="border-(--moss-border) rounded-lg border p-4">
            <h3 className="text-(--moss-primary-foreground) mb-3 text-lg font-semibold">Folder Settings</h3>
            <p className="text-(--moss-secondary-foreground)">Advanced folder settings will be implemented here.</p>
            <div className="mt-4 space-y-4">
              <div className="background-(--moss-secondary-background) rounded p-3">
                <h4 className="text-(--moss-primary-foreground) mb-2 font-medium">Display Options</h4>
                <p className="text-(--moss-secondary-foreground) text-sm">
                  Settings for how this folder and its contents are displayed in the tree.
                </p>
              </div>
              <div className="background-(--moss-secondary-background) rounded p-3">
                <h4 className="text-(--moss-primary-foreground) mb-2 font-medium">Organization</h4>
                <p className="text-(--moss-secondary-foreground) text-sm">
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
        isRenamingTitle={isRenamingResource}
        setIsRenamingTitle={setIsRenamingResource}
        handleRenamingFormCancel={handleRenamingResourceCancel}
        onTitleChange={handleRenamingResourceSubmit}
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

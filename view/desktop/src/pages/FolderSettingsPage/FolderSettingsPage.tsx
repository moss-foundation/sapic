import { useState } from "react";

import { PageContainerWithTabs, TabItem } from "@/components/PageContainer";
import { IDockviewPanelProps } from "@/lib/moss-tabs/src";
import { Icon } from "@/lib/ui";
import { TreeCollectionNode } from "@/components/CollectionTree/types";
import { EntryKind } from "@repo/moss-collection";

import { OverviewTabContent } from "./tabs/OverviewTabContent";

export interface FolderSettingsParams {
  collectionId: string;
  node: TreeCollectionNode;
  iconType: EntryKind;
}

export const FolderSettings = ({ ...props }: IDockviewPanelProps<FolderSettingsParams>) => {
  const { collectionId, node } = props.params || {};

  const [activeTabId, setActiveTabId] = useState("overview");

  if (!collectionId || !node) {
    return (
      <div className="flex h-full items-center justify-center text-(--moss-primary-text)">
        <div className="text-center">
          <h2 className="text-lg font-semibold">No Folder Selected</h2>
          <p className="text-sm">Please select a folder to view its settings.</p>
        </div>
      </div>
    );
  }

  // Define the tabs for the folder settings
  const tabs: TabItem[] = [
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
            <div className="mt-4 rounded bg-(--moss-secondary-background) p-3">
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
              <div className="rounded bg-(--moss-secondary-background) p-3">
                <h4 className="mb-2 font-medium text-(--moss-primary-text)">Display Options</h4>
                <p className="text-sm text-(--moss-secondary-text)">
                  Settings for how this folder and its contents are displayed in the tree.
                </p>
              </div>
              <div className="rounded bg-(--moss-secondary-background) p-3">
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

  return <PageContainerWithTabs tabs={tabs} activeTabId={activeTabId} onTabChange={setActiveTabId} />;
};

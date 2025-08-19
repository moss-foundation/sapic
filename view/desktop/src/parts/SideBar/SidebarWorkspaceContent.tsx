import { useDescribeWorkspaceState } from "@/hooks/workspace/useDescribeWorkspaceState";
import { CollectionTreesView } from "@/parts/CollectionTreesView/CollectionTreesView";
import { SidebarHeader } from "@/parts/SideBar/SidebarHeader";
import {
  TREE_VIEW_GROUP_COLLECTIONS,
  TREE_VIEW_GROUP_ENVIRONMENTS,
  TREE_VIEW_GROUP_MOCK_SERVERS,
} from "@repo/moss-workspace";

import { EnvironmentsListView } from "../EnvironmentsListView/EnvironmentsListView";

interface SidebarWorkspaceContentProps {
  workspaceName?: string | null;
  // FIXME: remove from props and replace with workspaceState?.sidebar?.treeViewGroupId ?? "default";
  groupId?: string;
}

export const SidebarWorkspaceContent = ({ workspaceName, groupId = "default" }: SidebarWorkspaceContentProps) => {
  const { data: workspaceState, isLoading: isLoadingWorkspace, error: workspaceError } = useDescribeWorkspaceState();

  // Show loading state while workspace data is loading
  if (isLoadingWorkspace) {
    return <div className="flex h-full w-full items-center justify-center p-4">Loading...</div>;
  }

  if (!workspaceName) {
    return <div className="flex h-full w-full items-center justify-center p-4">No workspace selected</div>;
  }

  // Handle actual errors loading workspace data
  if (workspaceError) {
    return (
      <div className="flex h-full w-full items-center justify-center p-4">
        <div className="text-center">
          <p className="text-red-600">Error loading workspace: {workspaceName}</p>
          <p className="mt-2 text-sm text-gray-500">{workspaceError?.message || "Unknown error"}</p>
        </div>
      </div>
    );
  }

  // Handle case where workspace data is null (different from error)
  if (!workspaceState) {
    return (
      <div className="flex h-full w-full items-center justify-center p-4">
        <div className="text-center">
          <p>Workspace "{workspaceName}" not found</p>
          <p className="mt-2 text-sm text-gray-500">The workspace may have been moved or deleted</p>
        </div>
      </div>
    );
  }

  // Handle different sidebar views based on groupId
  switch (groupId) {
    case TREE_VIEW_GROUP_COLLECTIONS:
      return <CollectionTreesView />;

    case TREE_VIEW_GROUP_ENVIRONMENTS:
      return <EnvironmentsListView />;

    case TREE_VIEW_GROUP_MOCK_SERVERS:
      return (
        <div className="flex h-full flex-col">
          <SidebarHeader title="Mock Servers" />
          <div className="p-4">
            <h3 className="text-lg font-semibold">Mock Servers</h3>
            <p className="mt-2 text-sm text-gray-500">Mock server configuration</p>
          </div>
        </div>
      );

    case "4":
      return (
        <div className="flex h-full flex-col">
          <SidebarHeader title="Preferences" />
          <div className="p-4">
            <h3 className="text-lg font-semibold">Preferences</h3>
            <p className="mt-2 text-sm text-gray-500">Preferences configuration</p>
          </div>
        </div>
      );

    case "5":
      return (
        <div className="flex h-full flex-col">
          <SidebarHeader title="Commits" />
          <div className="p-4">
            <h3 className="text-lg font-semibold">Commits</h3>
            <p className="mt-2 text-sm text-gray-500">Mock server configuration</p>
          </div>
        </div>
      );

    default:
      return (
        <div className="p-4">
          <h3 className="text-lg font-semibold">No content</h3>
          <p className="mt-2 text-sm text-gray-500">No content for this group, showing default view</p>
          <div>{groupId}</div>
        </div>
      );
  }
};

export default SidebarWorkspaceContent;

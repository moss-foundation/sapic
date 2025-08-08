import { useDescribeWorkspaceState } from "@/hooks/workspace/useDescribeWorkspaceState";
import {
  TREE_VIEW_GROUP_COLLECTIONS,
  TREE_VIEW_GROUP_ENVIRONMENTS,
  TREE_VIEW_GROUP_MOCK_SERVERS,
} from "@repo/moss-workspace";

import CollectionTreeView from "./CollectionTreeView";

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
      return <CollectionTreeView />;

    case TREE_VIEW_GROUP_ENVIRONMENTS:
      return (
        <div className="p-4">
          <h3 className="text-lg font-semibold">Environments</h3>
          <p className="mt-2 text-sm text-gray-500">Environment management</p>
        </div>
      );

    case TREE_VIEW_GROUP_MOCK_SERVERS:
      return (
        <div className="p-4">
          <h3 className="text-lg font-semibold">Mock Servers</h3>
          <p className="mt-2 text-sm text-gray-500">Mock server configuration</p>
        </div>
      );

    default:
      // For default groupId, show collections tree view as the default sidebar content
      return <CollectionTreeView />;
  }
};

export default SidebarWorkspaceContent;

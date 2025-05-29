import { useDescribeWorkspaceState } from "@/hooks/workspace/useDescribeWorkspaceState";
import { useListCollections } from "@/hooks/collection/useListCollections";
import TabbedPane from "../parts/TabbedPane/TabbedPane";

interface WorkspaceProps {
  workspaceName?: string | null;
}

export const Workspace = ({ workspaceName }: WorkspaceProps) => {
  const effectiveWorkspaceName = workspaceName ?? null;

  const { data: workspaceState, isLoading: isLoadingWorkspace, error: workspaceError } = useDescribeWorkspaceState({});
  const { isLoading: isLoadingCollections, error: collectionsError } = useListCollections(effectiveWorkspaceName);

  // Show loading state while any critical data is loading
  if (isLoadingWorkspace || isLoadingCollections) {
    return <div className="flex h-full w-full items-center justify-center">Loading workspace...</div>;
  }

  // Handle case where no workspace is selected
  if (!effectiveWorkspaceName) {
    return <div className="flex h-full w-full items-center justify-center">No workspace selected</div>;
  }

  // Handle actual errors loading workspace data
  if (workspaceError || collectionsError) {
    return (
      <div className="flex h-full w-full items-center justify-center">
        <div className="text-center">
          <p className="text-red-600">Error loading workspace: {effectiveWorkspaceName}</p>
          <p className="mt-2 text-sm text-gray-500">
            {workspaceError?.message || collectionsError?.message || "Unknown error"}
          </p>
        </div>
      </div>
    );
  }

  // Handle case where workspace data is null (different from error)
  if (!workspaceState) {
    return (
      <div className="flex h-full w-full items-center justify-center">
        <div className="text-center">
          <p>Workspace "{effectiveWorkspaceName}" not found</p>
          <p className="mt-2 text-sm text-gray-500">The workspace may have been moved or deleted</p>
        </div>
      </div>
    );
  }

  // Always render TabbedPane for the main content area
  return <TabbedPane theme="dockview-theme-light" mode="empty" />;
};

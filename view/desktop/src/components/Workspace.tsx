import { useEffect } from "react";
import { useDescribeWorkspaceState } from "@/hooks/workspace/useDescribeWorkspaceState";
import { useListCollections } from "@/hooks/collection/useListCollections";
import { useGetViewGroup } from "@/hooks/viewGroups/useGetViewGroup";
import { ContentLayout } from "@/layouts/ContentLayout";
import CollectionTreeView from "./CollectionTreeView";
import TabbedPane from "../parts/TabbedPane/TabbedPane";

interface WorkspaceProps {
  groupId?: string;
  workspaceName?: string | null;
}

export const Workspace = ({ groupId = "default", workspaceName }: WorkspaceProps) => {
  // Use the provided workspaceName prop - context is no longer needed
  const effectiveWorkspaceName = workspaceName ?? null;

  const { data: workspaceState, isLoading: isLoadingWorkspace, error: workspaceError } = useDescribeWorkspaceState({});
  const {
    data: collections = [],
    isLoading: isLoadingCollections,
    error: collectionsError,
  } = useListCollections(effectiveWorkspaceName);
  const { data: viewGroup, isLoading: isLoadingViewGroup } = useGetViewGroup(groupId);

  useEffect(() => {
    if (effectiveWorkspaceName) {
      // Update page title when workspace changes
      document.title = `Sapic - ${effectiveWorkspaceName}`;
    }
  }, [effectiveWorkspaceName]);

  // Show loading state while any critical data is loading
  if (isLoadingWorkspace || isLoadingCollections || isLoadingViewGroup) {
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

  // Check if this is the main area usage (default groupId) - render TabbedPane
  if (groupId === "default") {
    return <TabbedPane theme="dockview-theme-light" mode="empty" />;
  }

  // If view group doesn't exist for this groupId, show an error
  if (groupId !== "default" && !viewGroup) {
    return <div className="p-4">No view group found for "{groupId}"</div>;
  }

  // Handle different sidebar views based on groupId
  switch (groupId) {
    case "collections.groupId":
      return <CollectionTreeView />;

    case "environments.groupId":
      return (
        <ContentLayout>
          <div className="p-6">
            <h2 className="text-2xl font-bold">Environments</h2>
            <p className="mt-4 text-gray-500">Environment management view</p>
          </div>
        </ContentLayout>
      );

    case "mock.groupId":
      return (
        <ContentLayout>
          <div className="p-6">
            <h2 className="text-2xl font-bold">Mock Server</h2>
            <p className="mt-4 text-gray-500">Mock server configuration</p>
          </div>
        </ContentLayout>
      );

    default:
      // Default sidebar view - show collections
      return (
        <ContentLayout>
          <div className="p-6">
            <h1 className="mb-4 text-2xl font-bold">{effectiveWorkspaceName}</h1>

            {collections.length === 0 ? (
              <div className="rounded-md bg-amber-50 p-4 dark:bg-amber-950">
                <p>This workspace has no collections yet.</p>
                <p>Create your first collection to get started.</p>
              </div>
            ) : (
              <div className="grid grid-cols-1 gap-4 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4">
                {collections.map((collection) => (
                  <div
                    key={collection.id}
                    className="flex flex-col rounded-lg border border-gray-200 p-4 shadow-sm dark:border-gray-800"
                  >
                    <h3 className="text-lg font-medium">{collection.displayName}</h3>
                    <p className="mt-2 flex-grow text-sm text-gray-500">{"No description"}</p>
                    <div className="mt-4 text-sm text-gray-400">{0} items</div>
                  </div>
                ))}
              </div>
            )}
          </div>
        </ContentLayout>
      );
  }
};

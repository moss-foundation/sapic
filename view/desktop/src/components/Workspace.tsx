import { useListCollections } from "@/hooks/collection/useListCollections";
import { useWorkspaceSidebarState } from "@/hooks/workspace/useWorkspaceSidebarState";
import TabbedPane from "../parts/TabbedPane/TabbedPane";

interface WorkspaceProps {
  workspaceName?: string | null;
}

export const Workspace = ({ workspaceName }: WorkspaceProps) => {
  const effectiveWorkspaceName = workspaceName ?? null;

  useWorkspaceSidebarState();

  const { isLoading: isLoadingCollections, error: collectionsError } = useListCollections(effectiveWorkspaceName);

  // Show loading state while any critical data is loading
  if (isLoadingCollections) {
    return <div className="flex h-full w-full items-center justify-center">Loading workspace...</div>;
  }

  // Handle case where no workspace is selected
  if (!effectiveWorkspaceName) {
    return <div className="flex h-full w-full items-center justify-center">No workspace selected</div>;
  }

  // Handle actual errors loading workspace data
  if (collectionsError) {
    return (
      <div className="flex h-full w-full items-center justify-center">
        <div className="text-center">
          <p className="text-red-600">Error loading workspace: {effectiveWorkspaceName}</p>
          <p className="mt-2 text-sm text-gray-500">{collectionsError?.message || "Unknown error"}</p>
        </div>
      </div>
    );
  }

  return <TabbedPane theme="dockview-theme-light" mode="auto" />;
};

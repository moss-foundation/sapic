import { useStreamProjects } from "@/hooks";
import { useWorkspaceSidebarState } from "@/hooks/workspace/derived/useWorkspaceSidebarState";

import TabbedPane from "../parts/TabbedPane/TabbedPane";

interface WorkspaceProps {
  workspaceName?: string | null;
}

export const Workspace = ({ workspaceName }: WorkspaceProps) => {
  const effectiveWorkspaceName = workspaceName ?? null;

  useWorkspaceSidebarState();

  const { isLoading: isLoadingProjects, error: projectsError } = useStreamProjects();

  // Show loading state while any critical data is loading
  if (isLoadingProjects) {
    return <div className="flex h-full w-full items-center justify-center">Loading workspace...</div>;
  }

  // Handle case where no workspace is selected
  if (!effectiveWorkspaceName) {
    return <div className="flex h-full w-full items-center justify-center">No workspace selected</div>;
  }

  // Handle actual errors loading workspace data
  if (projectsError) {
    return (
      <div className="flex h-full w-full items-center justify-center">
        <div className="text-center">
          <p className="text-red-600">Error loading workspace: {effectiveWorkspaceName}</p>
          <p className="mt-2 text-sm text-gray-500">{projectsError?.message || "Unknown error"}</p>
        </div>
      </div>
    );
  }

  return <TabbedPane theme="dockview-theme-light" mode="auto" />;
};

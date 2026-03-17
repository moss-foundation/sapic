import { useGetAllLocalProjectSummaries } from "@/db/projectSummaries/hooks/useGetAllLocalProjectSummaries";
import { useGetLayout } from "@/workbench/adapters";
import { TabbedPane } from "@/workbench/ui/parts";

interface WorkspaceProps {
  workspaceName?: string | null;
}

export const Workspace = ({ workspaceName }: WorkspaceProps) => {
  const effectiveWorkspaceName = workspaceName ?? null;

  const { isFetching: isFetchingLayout } = useGetLayout();
  const { isLoading: areProjectsLoading, isError: areProjectsError } = useGetAllLocalProjectSummaries();

  // Show loading state while any critical data is loading
  if (areProjectsLoading || isFetchingLayout) {
    return <div className="flex h-full w-full items-center justify-center">Loading workspace...</div>;
  }

  // Handle case where no workspace is selected
  if (!effectiveWorkspaceName) {
    return <div className="flex h-full w-full items-center justify-center">No workspace selected</div>;
  }

  // Handle actual errors loading workspace data
  if (areProjectsError) {
    return (
      <div className="flex h-full w-full items-center justify-center">
        <div className="text-center">
          <p className="text-red-600">Error loading workspace: {effectiveWorkspaceName}</p>
        </div>
      </div>
    );
  }

  return <TabbedPane />;
};

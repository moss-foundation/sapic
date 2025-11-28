import { useListWorkspaces } from "@/adapters/tanstackQuery/workspace";
import { useParams } from "@tanstack/react-router";

/**
 * @deprecated we now use useParams to get the current workspace on window. Technically we can't be out of workspace if we are in main window
 */
export const useActiveWorkspace = () => {
  const { workspaceId } = useParams({ strict: false });
  const { data: workspaces } = useListWorkspaces();

  const activeWorkspace = workspaces?.find((workspace) => workspace.id === workspaceId) || undefined;
  const activeWorkspaceId = activeWorkspace?.id || undefined;
  const hasActiveWorkspace = !!activeWorkspace;

  return {
    activeWorkspace,
    activeWorkspaceId,
    hasActiveWorkspace,
  };
};

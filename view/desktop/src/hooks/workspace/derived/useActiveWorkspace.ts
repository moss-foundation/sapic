import { useWorkspaceMapping } from "@/hooks";

export const useActiveWorkspace = () => {
  const { workspaces } = useWorkspaceMapping();

  const activeWorkspace = workspaces.find((workspace) => workspace.active) || null;
  const activeWorkspaceId = activeWorkspace?.id || null;
  const hasActiveWorkspace = !!activeWorkspace;

  return {
    activeWorkspace,
    activeWorkspaceId,
    hasActiveWorkspace,
  };
};

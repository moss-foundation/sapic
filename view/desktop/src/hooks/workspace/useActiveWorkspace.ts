import { useWorkspaceMapping } from "@/hooks";
import { WorkspaceInfo } from "@repo/moss-app";

export const useActiveWorkspace = (): WorkspaceInfo | null => {
  const { workspaces } = useWorkspaceMapping();

  // Find the workspace that is marked as active
  const activeWorkspace = workspaces.find((workspace) => workspace.active) || null;

  return activeWorkspace;
};

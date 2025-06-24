import { useDescribeAppState, useWorkspaceMapping } from "@/hooks";
import { WorkspaceInfo } from "@repo/moss-app";

export const useActiveWorkspace = (): WorkspaceInfo | null => {
  const { data: appState } = useDescribeAppState();
  const { getWorkspaceById } = useWorkspaceMapping();

  const activeWorkspaceId = appState?.lastWorkspace || null;
  const activeWorkspace = activeWorkspaceId ? getWorkspaceById(activeWorkspaceId) || null : null;

  return activeWorkspace;
};

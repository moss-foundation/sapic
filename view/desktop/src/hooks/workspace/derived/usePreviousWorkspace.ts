import { useDescribeAppState, useWorkspaceMapping } from "@/hooks";
import { WorkspaceInfo } from "@repo/moss-app";

export const usePreviousWorkspace = (): WorkspaceInfo | null => {
  const { data: appState } = useDescribeAppState();
  const { getWorkspaceById } = useWorkspaceMapping();

  const prevWorkspaceId = appState?.prevWorkspaceId || null;
  const prevWorkspace = prevWorkspaceId ? getWorkspaceById(prevWorkspaceId) || null : null;

  return prevWorkspace;
};

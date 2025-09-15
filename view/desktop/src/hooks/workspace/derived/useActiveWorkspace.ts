import { useDescribeApp } from "@/hooks/useDescribeApp";

export const useActiveWorkspace = () => {
  const { data: appState } = useDescribeApp();

  const activeWorkspace = appState?.workspace || null;
  const activeWorkspaceId = appState?.workspace?.id || null;
  const hasActiveWorkspace = !!activeWorkspace;

  return {
    activeWorkspace,
    activeWorkspaceId,
    hasActiveWorkspace,
  };
};

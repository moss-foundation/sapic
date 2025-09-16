import { useDescribeApp } from "@/hooks/useDescribeApp";

export const useActiveWorkspace = () => {
  const { data: appState, isLoading } = useDescribeApp();

  const activeWorkspace = appState?.workspace || null;
  const activeWorkspaceId = appState?.workspace?.id || null;
  const hasActiveWorkspace = !!activeWorkspace && !isLoading;

  return {
    activeWorkspace,
    activeWorkspaceId,
    hasActiveWorkspace,
  };
};

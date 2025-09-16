import { useDescribeApp } from "@/hooks/useDescribeApp";

export const useActiveWorkspace = () => {
  const { data: appState, isLoading, isSuccess } = useDescribeApp();

  const activeWorkspace = appState?.workspace || null;
  const activeWorkspaceId = appState?.workspace?.id || null;
  const hasActiveWorkspace = !!activeWorkspace && isSuccess;

  return {
    activeWorkspace,
    activeWorkspaceId,
    hasActiveWorkspace,
    isLoading,
    isSuccess,
  };
};

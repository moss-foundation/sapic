import { useDescribeApp } from "@/hooks/app/useDescribeApp";

export const useActiveWorkspace = () => {
  const { data: appState, isLoading } = useDescribeApp();

  const activeWorkspace = appState?.workspace || undefined;
  const activeWorkspaceId = appState?.workspace?.id || undefined;
  const hasActiveWorkspace = !!appState?.workspace && !isLoading;

  return {
    activeWorkspace,
    activeWorkspaceId,
    hasActiveWorkspace,
  };
};

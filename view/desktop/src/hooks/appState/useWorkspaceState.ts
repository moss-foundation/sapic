import { useDescribeAppState } from "./useDescribeAppState";

export type WorkspaceStateType = "empty" | "inWorkspace";

export interface WorkspaceState {
  state: WorkspaceStateType;
  lastWorkspace: string | null;
  isLoading: boolean;
}

/**
 * Hook to determine if the application is in Empty or InWorkspace state
 * based on the presence of a lastWorkspace in appState
 */
export const useWorkspaceState = (): WorkspaceState => {
  const { data: appState, isLoading } = useDescribeAppState();

  if (isLoading) {
    return {
      state: "empty",
      lastWorkspace: null,
      isLoading: true,
    };
  }

  const hasWorkspace = !!appState?.lastWorkspace;

  return {
    state: hasWorkspace ? "inWorkspace" : "empty",
    lastWorkspace: appState?.lastWorkspace || null,
    isLoading: false,
  };
};

import React, { createContext, useContext, useState, useEffect, ReactNode } from "react";
import { useOpenWorkspace } from "@/hooks/workspaces/useOpenWorkspace";
import { useWorkspaceState } from "@/hooks/appState/useWorkspaceState";

// Helper to extract workspace name from prefixed ID
export const extractWorkspaceName = (actionId: string): string => {
  return actionId.startsWith("workspace:") ? actionId.replace("workspace:", "") : actionId;
};

interface WorkspaceContextType {
  selectedWorkspace: string | null;
  setSelectedWorkspace: (workspace: string | null) => void;
  openAndSelectWorkspace: (workspace: string) => void;
  workspaceState: "empty" | "inWorkspace";
}

const WorkspaceContext = createContext<WorkspaceContextType>({
  selectedWorkspace: null,
  setSelectedWorkspace: () => {},
  openAndSelectWorkspace: () => {},
  workspaceState: "empty",
});

export const useWorkspaceContext = () => useContext(WorkspaceContext);

export const WorkspaceProvider: React.FC<{ children: ReactNode }> = ({ children }) => {
  const [selectedWorkspace, setSelectedWorkspace] = useState<string | null>(null);
  const { mutate: openWorkspace } = useOpenWorkspace();
  const { state: workspaceState, lastWorkspace } = useWorkspaceState();

  // When app loads, sync selectedWorkspace with lastWorkspace from appState
  useEffect(() => {
    if (lastWorkspace && !selectedWorkspace) {
      setSelectedWorkspace(lastWorkspace);
    }
  }, [lastWorkspace, selectedWorkspace]);

  const openAndSelectWorkspace = (workspace: string) => {
    // Extract the workspace name in case it has a prefix
    const workspaceName = extractWorkspaceName(workspace);
    openWorkspace(workspaceName);
    setSelectedWorkspace(workspaceName);
  };

  return (
    <WorkspaceContext.Provider
      value={{
        selectedWorkspace,
        setSelectedWorkspace,
        openAndSelectWorkspace,
        workspaceState,
      }}
    >
      {children}
    </WorkspaceContext.Provider>
  );
};

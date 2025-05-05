import React, { createContext, useContext, useState, ReactNode } from "react";
import { useOpenWorkspace } from "@/hooks/workspaces/useOpenWorkspace";

// Helper to extract workspace name from prefixed ID
export const extractWorkspaceName = (actionId: string): string => {
  return actionId.startsWith("workspace:") ? actionId.replace("workspace:", "") : actionId;
};

interface WorkspaceContextType {
  selectedWorkspace: string | null;
  setSelectedWorkspace: (workspace: string | null) => void;
  openAndSelectWorkspace: (workspace: string) => void;
}

const WorkspaceContext = createContext<WorkspaceContextType>({
  selectedWorkspace: null,
  setSelectedWorkspace: () => {},
  openAndSelectWorkspace: () => {},
});

export const useWorkspaceContext = () => useContext(WorkspaceContext);

export const WorkspaceProvider: React.FC<{ children: ReactNode }> = ({ children }) => {
  const [selectedWorkspace, setSelectedWorkspace] = useState<string | null>(null);
  const { mutate: openWorkspace } = useOpenWorkspace();

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
      }}
    >
      {children}
    </WorkspaceContext.Provider>
  );
};

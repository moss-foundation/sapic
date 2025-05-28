import React, { createContext, useContext, useState, ReactNode, useEffect } from "react";
import { useOpenWorkspace } from "@/hooks/workbench/useOpenWorkspace";

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

interface WorkspaceProviderProps {
  children: ReactNode;
  initialWorkspace?: string | null;
}

export const WorkspaceProvider: React.FC<WorkspaceProviderProps> = ({ children, initialWorkspace = null }) => {
  const [selectedWorkspace, setSelectedWorkspace] = useState<string | null>(initialWorkspace);
  const { mutate: openWorkspace } = useOpenWorkspace();

  // Update selectedWorkspace when initialWorkspace changes
  useEffect(() => {
    if (initialWorkspace !== selectedWorkspace) {
      setSelectedWorkspace(initialWorkspace);
    }
  }, [initialWorkspace]);

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

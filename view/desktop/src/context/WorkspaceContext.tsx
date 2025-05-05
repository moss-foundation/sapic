import React, { createContext, useContext, useState, ReactNode } from "react";
import { useOpenWorkspace } from "@/hooks/workspaces/useOpenWorkspace";

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
    openWorkspace(workspace);
    setSelectedWorkspace(workspace);
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

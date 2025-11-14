import React, { createContext, ReactNode, useContext } from "react";

import { useListWorkspaces } from "@/hooks/workbench/useListWorkspaces";
import { MenuItemProps } from "@/workbench/utils/renderActionMenuItem";

import {
  additionalSelectedWorkspaceMenuItems,
  baseSelectedWorkspaceMenuItems,
  baseWorkspaceMenuItems,
  createAllWorkspacesMenuSection,
} from "./HeadBarData";

interface WorkspaceMenuContextType {
  workspaceMenuItems: MenuItemProps[];
  selectedWorkspaceMenuItems: MenuItemProps[];
  isLoading: boolean;
}

const WorkspaceMenuContext = createContext<WorkspaceMenuContextType>({
  workspaceMenuItems: [],
  selectedWorkspaceMenuItems: [],
  isLoading: true,
});

export const useWorkspaceMenu = () => useContext(WorkspaceMenuContext);

export const WorkspaceMenuProvider: React.FC<{ children: ReactNode }> = ({ children }) => {
  const { data: workspaces, isLoading } = useListWorkspaces();

  const allWorkspacesMenuSection = createAllWorkspacesMenuSection(workspaces || []);

  // Combine base menu items with the dynamic workspaces section
  const workspaceMenuItems = [...baseWorkspaceMenuItems, allWorkspacesMenuSection];

  // Combine base selected workspace menu items with the dynamic workspaces section and additional items
  const selectedWorkspaceMenuItems = [
    ...baseSelectedWorkspaceMenuItems,
    allWorkspacesMenuSection,
    ...additionalSelectedWorkspaceMenuItems,
  ];

  return (
    <WorkspaceMenuContext.Provider
      value={{
        workspaceMenuItems,
        selectedWorkspaceMenuItems,
        isLoading,
      }}
    >
      {children}
    </WorkspaceMenuContext.Provider>
  );
};

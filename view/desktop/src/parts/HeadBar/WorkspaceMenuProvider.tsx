import React, { createContext, useContext, ReactNode } from "react";
import { MenuItemProps } from "@/components/ActionMenu/ActionMenu";
import { useGetWorkspaces } from "@/hooks/workspaces/useGetWorkspaces";
import {
  baseWorkspaceMenuItems,
  baseSelectedWorkspaceMenuItems,
  additionalSelectedWorkspaceMenuItems,
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
  const { data: workspaces, isLoading } = useGetWorkspaces();

  // Create the All Workspaces menu section with real data
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

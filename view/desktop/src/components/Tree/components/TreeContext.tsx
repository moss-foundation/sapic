import { createContext, useContext } from "react";

import { WorkspaceMode } from "@repo/moss-workspace";

export interface TreeContextProps {
  id: string;
  iconPath?: string;
  treePaddingLeft: number;
  treePaddingRight: number;
  nodeOffset: number;
  allFoldersAreExpanded: boolean;
  allFoldersAreCollapsed: boolean;
  searchInput?: string;
  displayMode: WorkspaceMode;
  showNodeOrders?: boolean;
}

export const TreeContext = createContext<TreeContextProps>({
  id: "",
  iconPath: "",
  treePaddingLeft: 8,
  treePaddingRight: 8,
  nodeOffset: 12,
  allFoldersAreExpanded: false,
  allFoldersAreCollapsed: false,
  displayMode: "REQUEST_FIRST",
  showNodeOrders: false,
});

export const useTreeContext = () => {
  return useContext(TreeContext);
};

import { createContext, useContext } from "react";

export interface TreeContextProps {
  id: string;
  iconPath?: string;
  treePaddingLeft: number;
  treePaddingRight: number;
  nodeOffset: number;
  allFoldersAreExpanded: boolean;
  allFoldersAreCollapsed: boolean;
  searchInput?: string;
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
  showNodeOrders: false,
});

export const useTreeContext = () => {
  return useContext(TreeContext);
};

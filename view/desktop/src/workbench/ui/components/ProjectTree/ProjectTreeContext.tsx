import { createContext } from "react";

import { WorkspaceMode } from "@repo/base";

export interface ProjectTreeContextProps {
  id: string;
  name: string;
  order: number;
  treePaddingLeft: number;
  treePaddingRight: number;
  nodeOffset: number;
  showOrders?: boolean;
  showRootNodeIds?: boolean;
  iconPath?: string;
  allFoldersAreExpanded: boolean;
  allFoldersAreCollapsed: boolean;
  searchInput: string;
  displayMode: WorkspaceMode;
}

export const ProjectTreeContext = createContext<ProjectTreeContextProps>({
  id: "",
  name: "",
  order: 0,
  iconPath: undefined,
  treePaddingLeft: 0,
  treePaddingRight: 0,
  nodeOffset: 0,
  allFoldersAreExpanded: false,
  allFoldersAreCollapsed: true,
  searchInput: "",
  displayMode: "LIVE",
  showOrders: false,
  showRootNodeIds: false,
});

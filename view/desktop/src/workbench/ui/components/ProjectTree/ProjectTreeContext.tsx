import { createContext } from "react";

import { WorkspaceMode } from "@repo/base";

import { BaseTreeContextProps } from "../../../../lib/ui/Tree/types";

export interface ProjectTreeContextProps extends BaseTreeContextProps {
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

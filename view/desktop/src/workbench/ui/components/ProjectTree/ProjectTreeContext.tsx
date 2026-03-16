import { createContext } from "react";

import { WorkspaceMode } from "@repo/base";

export interface ProjectTreeContextProps {
  id: string;
  name: string;
  order: number;
  showOrders?: boolean;
  showTreeRootIds?: boolean;
  iconPath?: string;
  isFullyExpanded: boolean;
  isFullyCollapsed: boolean;
  searchInput: string;
  displayMode: WorkspaceMode;
}

export const ProjectTreeContext = createContext<ProjectTreeContextProps>({
  id: "",
  name: "",
  order: 0,
  iconPath: undefined,
  isFullyExpanded: false,
  isFullyCollapsed: false,
  searchInput: "",
  displayMode: "LIVE",
  showOrders: false,
  showTreeRootIds: false,
});

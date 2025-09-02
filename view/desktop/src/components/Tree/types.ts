import { WorkspaceMode } from "@repo/moss-workspace";

export interface BaseTreeContextProps {
  id: string;
  name: string;
  order: number;
  iconPath?: string;
  treePaddingLeft: number;
  treePaddingRight: number;
  nodeOffset: number;
  allFoldersAreExpanded: boolean;
  allFoldersAreCollapsed: boolean;
  searchInput: string;
  displayMode: WorkspaceMode;
  showOrders: boolean;
}

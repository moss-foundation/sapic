import { createContext } from "react";

import { WorkspaceMode } from "@repo/moss-workspace";

import { BaseTreeContextProps } from "../Tree/types";

export interface CollectionTreeContextProps extends BaseTreeContextProps {
  displayMode: WorkspaceMode;
}

export const CollectionTreeContext = createContext<CollectionTreeContextProps>({
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
  displayMode: "REQUEST_FIRST",
  showOrders: false,
});

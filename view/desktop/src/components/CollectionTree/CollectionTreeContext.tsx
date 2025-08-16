import { createContext } from "react";

import { CollectionTreeContextProps } from "./types";

export const CollectionTreeContext = createContext<CollectionTreeContextProps>({
  id: "",
  name: "",
  repository: undefined,
  order: undefined,
  iconPath: undefined,
  expanded: false,
  treePaddingLeft: 0,
  treePaddingRight: 0,
  nodeOffset: 0,
  allFoldersAreExpanded: false,
  allFoldersAreCollapsed: true,
  searchInput: undefined,
  displayMode: "REQUEST_FIRST",
  showNodeOrders: false,
  contributors: [],
  repositoryInfo: undefined,
});

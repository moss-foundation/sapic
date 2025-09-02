import { createContext } from "react";

import { BaseTreeContextProps } from "../Tree/types";

export const CollectionTreeContext = createContext<BaseTreeContextProps>({
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

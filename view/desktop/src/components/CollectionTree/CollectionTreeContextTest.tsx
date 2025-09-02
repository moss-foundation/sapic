import { createContext } from "react";

import { BaseTreeContextProps } from "../Tree/types";

export interface CollectionTreeContextTestProps extends BaseTreeContextProps {
  something: string;
}

export const CollectionTreeContextTest = createContext<CollectionTreeContextTestProps>({
  something: "",
  id: "",
  treePaddingLeft: 0,
  treePaddingRight: 0,
  allFoldersAreExpanded: false,
  allFoldersAreCollapsed: false,
  displayMode: "REQUEST_FIRST",
  showNodeOrders: false,
});

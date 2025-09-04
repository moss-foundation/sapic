import { createContext, useContext } from "react";

import { BaseTreeContextProps } from "../types";

export const TreeContext = createContext<BaseTreeContextProps>({
  id: "",
  name: "",
  order: 0,
  treePaddingLeft: 8,
  treePaddingRight: 8,
  nodeOffset: 12,
  showOrders: false,
});

export const useTreeContext = () => {
  return useContext(TreeContext);
};

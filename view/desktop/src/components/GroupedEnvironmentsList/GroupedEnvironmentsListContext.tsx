import { createContext } from "react";

import { BaseTreeContextProps } from "@/lib/ui/Tree/types";

export const GroupedEnvironmentsListContext = createContext<BaseTreeContextProps>({
  id: "",
  name: "",
  order: 0,
  treePaddingLeft: 0,
  treePaddingRight: 0,
  nodeOffset: 0,
  showOrders: false,
});

import { useContext } from "react";

import { ActivityRouterContext } from "@/workbench/providers/ActivityRouterProvider";

export const useActivityRouter = () => {
  return useContext(ActivityRouterContext);
};

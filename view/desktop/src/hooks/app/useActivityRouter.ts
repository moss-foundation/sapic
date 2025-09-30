import { useContext } from "react";

import { ActivityRouterContext } from "@/app/ActivityRouterProvider";

export const useActivityRouter = () => {
  return useContext(ActivityRouterContext);
};

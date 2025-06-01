import { useEffect, useState } from "react";

import { useAppResizableLayoutStore } from "@/store/appResizableLayout";

import { useDescribeWorkspaceState } from "./workspace/useDescribeWorkspaceState";

export interface WindowPreparationState {
  isPreparing: boolean;
}

export const usePrepareWindow = (): WindowPreparationState => {
  const [isPreparing, setIsPreparing] = useState(true);

  const { initialize } = useAppResizableLayoutStore();
  const { isFetched, data: layout } = useDescribeWorkspaceState({});

  useEffect(() => {
    if (isFetched) setIsPreparing(false);

    if (layout) {
      initialize({
        sideBar: {
          width: layout?.sidebar?.size,
          visible: layout?.sidebar?.visible,
        },
        bottomPane: {
          height: layout?.panel?.size,
          visible: layout?.panel?.visible,
        },
      });
    }
  }, [initialize, isFetched, layout]);

  return { isPreparing };
};

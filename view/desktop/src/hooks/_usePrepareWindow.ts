import { useEffect, useState } from "react";

import { useAppResizableLayoutStore } from "@/store/appResizableLayout";

import { useDescribeWorkspaceState } from "./workspace/useDescribeWorkspaceState";

export interface WindowPreparationState {
  isPreparing: boolean;
}

export const usePrepareWindow = (): WindowPreparationState => {
  const [isPreparing, setIsPreparing] = useState(true);

  const { initialize } = useAppResizableLayoutStore();
  const { isFetched, data: layout } = useDescribeWorkspaceState();

  useEffect(() => {
    if (isFetched) setIsPreparing(false);

    if (layout) {
      initialize({
        sideBar: {
          width: layout?.sidebar?.preferredSize,
          visible: layout?.sidebar?.isVisible,
        },
        bottomPane: {
          height: layout?.panel?.preferredSize,
          visible: layout?.panel?.isVisible,
        },
      });
    }
  }, [initialize, isFetched, layout]);

  return { isPreparing };
};

import { useEffect, useRef, useState } from "react";

import { describeLayoutPartsState, openWorkspace } from "@/lib/backend/workspace";
import { SerializedDockview } from "@/lib/moss-tabs/src";
import { useAppResizableLayoutStore } from "@/store/appResizableLayout";
import { useTabbedPaneStore } from "@/store/tabbedPane";

export interface WindowPreparationState {
  isPreparing: boolean;
}

export const usePrepareWindow = (): WindowPreparationState => {
  const [isPreparing, setIsPreparing] = useState(true);

  const hasOpenedWorkspace = useRef(false);

  const { initialize: initializeResizableLayout } = useAppResizableLayoutStore();
  const { setGridState } = useTabbedPaneStore();

  useEffect(() => {
    const initializeWorkspace = async () => {
      await openWorkspace("TestWorkspace");

      const layout = await describeLayoutPartsState();

      if (layout === undefined) {
        setIsPreparing(false);
        return;
      }

      if (layout?.editor) {
        setGridState(layout.editor as unknown as SerializedDockview);
      }

      initializeResizableLayout({
        sideBar: {
          width: layout?.sidebar?.preferredSize,
          visible: layout?.sidebar?.isVisible,
        },
        bottomPane: {
          height: layout?.panel?.preferredSize,
          visible: layout?.panel?.isVisible,
        },
      });

      setIsPreparing(false);
    };

    // Running this on mount ensures that the workspace is called only once
    // open_workspace will throw an error if previous request is still pending
    // The error usually happens in strict mode
    if (!hasOpenedWorkspace.current) {
      hasOpenedWorkspace.current = true;
      initializeWorkspace();
    }
  }, []);

  return { isPreparing };
};

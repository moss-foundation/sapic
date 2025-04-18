import { useEffect, useRef, useState } from "react";

import { describeLayoutPartsState, openWorkspace, setLayoutPartsState } from "@/lib/backend/workspace";
import { SerializedDockview } from "@/lib/moss-tabs/src";
import { useAppResizableLayoutStore } from "@/store/appResizableLayout";
import { useTabbedPaneStore } from "@/store/tabbedPane";
import { listen } from "@tauri-apps/api/event";

export interface WindowPreparationState {
  isPreparing: boolean;
}

export const usePrepareWindow = (): WindowPreparationState => {
  const [isPreparing, setIsPreparing] = useState(true);
  const hasOpenedWorkspace = useRef(false);
  const { initialize: initializeResizableLayout, sideBar, bottomPane } = useAppResizableLayoutStore();
  const { initialize: initializeTabbedPane, api } = useTabbedPaneStore();

  useEffect(() => {
    const initializeWorkspace = async () => {
      await openWorkspace("TestWorkspace");

      const layout = await describeLayoutPartsState();

      if (layout.status !== "ok" || !layout.data) {
        setIsPreparing(false);
        return;
      }

      if (layout.data?.editor) {
        initializeTabbedPane(layout.data.editor as unknown as SerializedDockview);
      }

      initializeResizableLayout({
        sideBar: {
          width: layout?.data?.sidebar?.preferredSize,
          visible: layout?.data?.sidebar?.isVisible,
        },
        bottomPane: {
          height: layout?.data?.panel?.preferredSize,
          visible: layout?.data?.panel?.isVisible,
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

  useEffect(() => {
    const unlisten = listen("kernel-windowCloseRequested", () => {
      setLayoutPartsState({
        input: {
          editor: api?.toJSON(),
          sidebar: {
            preferredSize: sideBar.width,
            isVisible: sideBar.visible,
          },
          panel: {
            preferredSize: bottomPane.height,
            isVisible: bottomPane.visible,
          },
        },
        params: { isOnExit: true },
      });
    });

    return () => {
      unlisten.then((unlisten) => unlisten());
    };
  }, [api, bottomPane.height, bottomPane.visible, sideBar.visible, sideBar.width]);

  return { isPreparing };
};

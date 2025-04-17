import { useEffect, useRef, useState } from "react";

import { invokeTauriIpc } from "@/lib/backend/tauri";
import { SerializedDockview } from "@/lib/moss-tabs/src";
import { useAppResizableLayoutStore } from "@/store/appResizableLayout";
import { useTabbedPaneStore } from "@/store/tabbedPane";
import { setLayoutPartsState } from "@/utils/setLayoutPartsState";
import { DescribeLayoutPartsStateOutput, OpenWorkspaceInput, OpenWorkspaceOutput } from "@repo/moss-workspace";
import { listen } from "@tauri-apps/api/event";

export interface WindowPreparationState {
  isPreparing: boolean;
}

export const usePrepareWindow = (): WindowPreparationState => {
  const [isPreparing, setIsPreparing] = useState(true);
  const hasOpenedWorkspace = useRef(false);
  const { initialize, sideBar, bottomPane } = useAppResizableLayoutStore();
  const { initialize: initializeTabbedPane, gridState } = useTabbedPaneStore();

  useEffect(() => {
    const openWorkspace = async () => {
      await invokeTauriIpc<OpenWorkspaceInput, OpenWorkspaceOutput>("open_workspace", {
        input: { name: "TestWorkspace" },
      });

      const layout = await invokeTauriIpc<DescribeLayoutPartsStateOutput>("describe_layout_parts_state");

      if (layout.status !== "ok" || !layout.data) {
        setIsPreparing(false);
        return;
      }

      if (layout.data?.editor) {
        initializeTabbedPane(layout.data.editor as unknown as SerializedDockview);
      }

      initialize({
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
      openWorkspace();
    }
  }, []);

  useEffect(() => {
    const unlisten = listen("kernel-windowCloseRequested", () => {
      setLayoutPartsState({
        input: {
          editor: gridState,
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
  }, [bottomPane.height, bottomPane.visible, gridState, sideBar.visible, sideBar.width]);

  return { isPreparing };
};

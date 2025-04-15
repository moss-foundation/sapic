import { useEffect, useState } from "react";

import { invokeTauriIpc } from "@/lib/backend/tauri";
import { useAppResizableLayoutStore } from "@/store/appResizableLayout";
import { listen } from "@tauri-apps/api/event";

export interface WindowPreparationState {
  isPreparing: boolean;
}

export const usePrepareWindow = (): WindowPreparationState => {
  const [isPreparing, setIsPreparing] = useState(true);

  const { sideBar, bottomPane } = useAppResizableLayoutStore();

  useEffect(() => {
    const openWorkspace = async () => {
      await invokeTauriIpc("open_workspace", {
        input: { name: "TestWorkspace" },
      });

      const res = await invokeTauriIpc("describe_layout_parts_state");
      console.log(res);
      if (res.status !== "ok") return;

      // res?.editor !== null && setEditorGridState(res.editor);

      if (res?.sidebar !== null) {
        console.log(1);
        sideBar.setWidth(res.data.sidebar.preferredSize);
        sideBar.setVisible(res.data.sidebar.isVisible);
      }
      if (res?.panel !== null) {
        bottomPane.setHeight(res.data.panel.preferredSize);
        bottomPane.setVisible(res.data.panel.isVisible);
      }
    };

    openWorkspace();
  }, []);

  useEffect(() => {
    const unlisten = listen("kernel-windowCloseRequested", (event) => {
      console.log(event);
      invokeTauriIpc("set_layout_parts_state", {
        input: {
          editor: null,
          sidebar: {
            preferredSize: sideBar.width,
            isVisible: sideBar.visible,
          },
          panel: {
            preferredSize: bottomPane.height,
            isVisible: bottomPane.visible,
          },
        },
        params: {
          isOnExit: true,
        },
      });
    });

    return () => {
      unlisten.then((unlisten) => unlisten());
    };
  }, [bottomPane.height, bottomPane.visible, sideBar.visible, sideBar.width]);

  useEffect(() => {
    setIsPreparing(false);
  }, []);

  return { isPreparing };
};

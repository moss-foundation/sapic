import { useEffect, useState } from "react";

import { invokeTauriIpc } from "@/lib/backend/tauri";
import { listen } from "@tauri-apps/api/event";

export interface WindowPreparationState {
  isPreparing: boolean;
}

export const usePrepareWindow = (): WindowPreparationState => {
  const [isPreparing, setIsPreparing] = useState(true);

  useEffect(() => {
    const openWorkspace = async () => {
      await invokeTauriIpc("open_workspace", {
        input: { name: "TestWorkspace" },
      });
      const res = await invokeTauriIpc("describe_layout_parts_state");
      console.log(res);
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
            preferredSize: 600,
            isVisible: false,
          },
          panel: {
            preferredSize: 600,
            isVisible: false,
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
  }, []);

  useEffect(() => {
    setIsPreparing(false);
  }, []);

  return { isPreparing };
};

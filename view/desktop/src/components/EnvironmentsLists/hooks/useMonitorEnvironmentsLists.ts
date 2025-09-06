import { useEffect } from "react";

import { monitorForElements } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";

import { isSourceEnvironmentItem } from "../utils";

export const useMonitorEnvironmentsLists = () => {
  useEffect(() => {
    return monitorForElements({
      canMonitor({ source }) {
        return isSourceEnvironmentItem(source);
      },
      onDrop({ source, location }) {
        if (!isSourceEnvironmentItem(source)) return;

        console.log("onDrop", source);
      },
    });
  }, []);
};

import { useContext, useEffect } from "react";

import { monitorForElements } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";

import { TreeContext } from "../Tree";
import { getLocationTreeNodeData, getSourceTreeNodeData, isSourceTreeNode } from "../utils2";

export const useMonitorForNodeDragAndDrop = () => {
  const { id } = useContext(TreeContext);

  useEffect(() => {
    return monitorForElements({
      canMonitor({ source, initial }) {
        // console.log({ source, initial });
        return isSourceTreeNode(source);
      },
      onDrop({ location, source }) {
        // console.log({ location, source });
        const sourceTreeNodeData = getSourceTreeNodeData(source);
        const locationTreeNodeData = getLocationTreeNodeData(location);
        console.log({ sourceTreeNodeData, locationTreeNodeData });
        // const instruction = getInstruction(self);
      },
    });
  }, []);
};

import { RefObject, useContext, useEffect, useState } from "react";

import { extractInstruction } from "@atlaskit/pragmatic-drag-and-drop-hitbox/list-item";
import { monitorForElements } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";

import { ProjectTreeContext } from "../../../ProjectTreeContext.tsx";
import { ResourcesTreeRoot } from "../../../TreeRoot/types";
import { DragResourceNode } from "../types.dnd.ts";
import { isSourceResourceNode } from "../validation/isSourceResourceTreeNode.ts";

interface UseMonitorResourcesListForBlockedChildOperationProps {
  listRef: RefObject<HTMLElement | null>;
  tree: ResourcesTreeRoot;
}

export const useMonitorResourcesListForBlockedChildOperation = ({
  listRef,
  tree,
}: UseMonitorResourcesListForBlockedChildOperationProps) => {
  const { id } = useContext(ProjectTreeContext);
  const rootResourcesNodes = tree.childNodes;
  const [childNodeHasBlockedOperation, setChildNodeHasBlockedOperation] = useState(false);

  useEffect(() => {
    const element = listRef.current;
    if (!element) return;
    return monitorForElements({
      canMonitor: ({ source }) => isSourceResourceNode(source),
      onDrag: ({ location }) => {
        const innermost = location.current.dropTargets[0];
        if (!innermost) {
          setChildNodeHasBlockedOperation(false);
          return;
        }

        const innermostData = innermost.data as DragResourceNode;
        const isDirectChild = innermostData.data?.parentNode?.id === tree.id;

        if (!isDirectChild) {
          setChildNodeHasBlockedOperation(false);
          return;
        }

        const extractedInstruction = extractInstruction(innermost.data);
        if (extractedInstruction?.operation !== "combine") {
          setChildNodeHasBlockedOperation(!!extractedInstruction?.blocked);
        } else {
          setChildNodeHasBlockedOperation(false);
        }
      },
      onDrop: () => setChildNodeHasBlockedOperation(false),
    });
  }, [id, listRef, rootResourcesNodes, tree]);

  return { childNodeHasBlockedOperation };
};

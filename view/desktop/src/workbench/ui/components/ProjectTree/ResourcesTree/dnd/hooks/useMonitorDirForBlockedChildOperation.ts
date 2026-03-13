import { useContext, useEffect, useState } from "react";

import { extractInstruction } from "@atlaskit/pragmatic-drag-and-drop-hitbox/list-item";
import { dropTargetForElements } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";

import { ProjectDragType } from "../../../constants";
import { ProjectTreeContext } from "../../../ProjectTreeContext";
import { ResourcesTreeRoot } from "../../../TreeRoot/types";
import { isSourceProjectTreeNode } from "../../../utils/DragAndDrop";
import { ResourceNode } from "../../types";
import { DragResourceNode } from "../types.dnd";

interface UseMonitorDirNodeBlockedChildOperationProps {
  nodeRef: React.RefObject<HTMLLIElement | null>;
  node: ResourceNode;
  parentNode: ResourceNode | ResourcesTreeRoot;
}
export const useMonitorDirForBlockedChildOperation = ({
  nodeRef,
  node,
  parentNode,
}: UseMonitorDirNodeBlockedChildOperationProps) => {
  const { id } = useContext(ProjectTreeContext);
  const [childNodeHasBlockedOperation, setChildNodeHasBlockedOperation] = useState<boolean>(false);

  useEffect(() => {
    const element = nodeRef.current;
    if (!element) return;

    return dropTargetForElements({
      element,
      canDrop: ({ source }) => isSourceProjectTreeNode(source),
      getData: () => {
        const locationData: DragResourceNode = {
          type: ProjectDragType.NODE,
          data: { projectId: id, node, parentNode },
        };
        return locationData;
      },
      onDrag: ({ location, self }) => {
        const innermost = location.current.dropTargets[0];
        // If this node IS the innermost target, no child is being hovered
        if (innermost.element === self.element) {
          setChildNodeHasBlockedOperation(false);
          return;
        }
        const childInstruction = extractInstruction(innermost.data);
        // Only highlight if the hovered node is a direct child of this node
        const innermostData = innermost.data as DragResourceNode;
        const isDirectChild = innermostData.data?.parentNode?.id === node.id;
        setChildNodeHasBlockedOperation(isDirectChild && !!childInstruction?.blocked);
      },
      onDragLeave: () => {
        setChildNodeHasBlockedOperation(false);
      },
      onDrop: () => {
        setChildNodeHasBlockedOperation(false);
      },
    });
  }, [id, node, nodeRef, parentNode]);

  return { childNodeHasBlockedOperation };
};

import { RefObject, useEffect } from "react";

import { dropTargetForElements } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";

import { TreeNodeProps } from "../types";
import { canDropNode, getActualDropSourceTarget, getActualDropTarget } from "../utils";

export const useDropTargetRootNode = (
  node: TreeNodeProps,
  treeId: string | number,
  dropTargetListRef: RefObject<HTMLDivElement>
) => {
  useEffect(() => {
    const element = dropTargetListRef.current;
    if (!element) return;

    return dropTargetForElements({
      element,
      getData: () => {
        return { type: "TreeRootNode", data: { treeId, node } };
      },
      canDrop({ source }) {
        return source.data.type === "TreeNode";
      },

      onDropTargetChange({ location, source }) {
        element.classList.remove("background-(--moss-success-background)", "background-(--moss-error-background)");

        if (
          location.current?.dropTargets.length === 0 ||
          location.current.dropTargets[0].data.type !== "TreeRootNode"
        ) {
          element.classList.remove("background-(--moss-success-background)", "background-(--moss-error-background)");
          return;
        }

        const sourceTarget = getActualDropSourceTarget(source);
        const dropTarget = getActualDropTarget(location);

        if (!dropTarget || !sourceTarget || dropTarget?.node.uniqueId !== node.uniqueId) {
          return;
        }
        if (canDropNode(sourceTarget, dropTarget, node)) {
          element.classList.add("background-(--moss-success-background)");
        } else {
          element.classList.add("background-(--moss-error-background)");
        }
      },
      onDrop({ location, source }) {
        element.classList.remove("background-(--moss-success-background)", "background-(--moss-error-background)");

        if (
          location.current?.dropTargets.length === 0 ||
          location.current.dropTargets[0].data.type !== "TreeRootNode"
        ) {
          return;
        }

        const sourceTarget = getActualDropSourceTarget(source);
        const dropTarget = getActualDropTarget(location);

        if (dropTarget?.node.uniqueId !== node.uniqueId) {
          return;
        }

        if (canDropNode(sourceTarget, dropTarget, node)) {
          window.dispatchEvent(
            new CustomEvent("moveTreeNode", {
              detail: {
                source: sourceTarget,
                target: dropTarget,
              },
            })
          );
        }

        element.classList.remove("background-(--moss-success-background)", "background-(--moss-error-background)");
      },
    });
  }, [dropTargetListRef, node, treeId]);
};

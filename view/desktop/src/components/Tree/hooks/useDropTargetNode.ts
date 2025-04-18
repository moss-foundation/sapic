import { RefObject, useEffect } from "react";

import { dropTargetForElements } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";

import { TreeNodeProps } from "../types";
import { canDropNode, getActualDropSourceTarget, getActualDropTarget } from "../utils";

export const useDropTargetNode = (
  node: TreeNodeProps,
  treeId: string | number,
  dropTargetListRef: RefObject<HTMLLIElement>,
  dropTargetFolderRef: RefObject<HTMLDivElement>
) => {
  useEffect(() => {
    const element = dropTargetListRef.current || dropTargetFolderRef.current;
    if (!element) return;

    return dropTargetForElements({
      element,
      getData: () => ({
        type: "TreeNode",
        data: { treeId, node },
      }),
      canDrop({ source }) {
        return source.data.type === "TreeNode";
      },

      onDragLeave() {
        element.classList.remove("background-(--moss-success-background)", "background-(--moss-error-background)");
      },
      onDropTargetChange({ location, source }) {
        if (location.current?.dropTargets.length === 0 || location.current.dropTargets[0].data.type !== "TreeNode") {
          element.classList.remove("background-(--moss-success-background)", "background-(--moss-error-background)");
          return;
        }

        const sourceTarget = getActualDropSourceTarget(source);
        const dropTarget = getActualDropTarget(location);

        if (!dropTarget || !sourceTarget || dropTarget?.node.uniqueId !== node.uniqueId) {
          element.classList.remove("background-(--moss-success-background)", "background-(--moss-error-background)");
          return;
        }
        if (canDropNode(sourceTarget, dropTarget, node)) {
          element.classList.add("background-(--moss-success-background)");
        } else {
          element.classList.add("background-(--moss-error-background)");
        }
      },
      onDrop({ location, source }) {
        console.log("onDrop");
        if (location.current?.dropTargets.length === 0 || location.current.dropTargets[0].data.type !== "TreeNode") {
          return;
        }

        const sourceTarget = getActualDropSourceTarget(source);
        const dropTarget = getActualDropTarget(location);

        if (dropTarget?.node.uniqueId !== node.uniqueId) {
          element.classList.remove("background-(--moss-success-background)", "background-(--moss-error-background)");
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
  }, [dropTargetFolderRef, dropTargetListRef, node, treeId]);
};

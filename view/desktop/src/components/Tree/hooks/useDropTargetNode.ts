import { RefObject, useEffect } from "react";
import { TreeNodeProps } from "../types";
import { dropTargetForElements } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";
import { getActualDropSourceTarget, getActualDropTarget, canDropNode } from "../utils";

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
        data: {
          treeId,
          node,
        },
      }),
      canDrop({ source }) {
        return source.data.type === "TreeNode";
      },
      onDragLeave() {
        element.classList.remove("background-(--moss-background-success)", "background-(--moss-background-error)");
      },
      onDrag({ location, source }) {
        if (location.current.dropTargets[0].data.type !== "TreeNode" || location.current?.dropTargets.length === 0) {
          return;
        }

        const sourceTarget = getActualDropSourceTarget(source);
        const dropTarget = getActualDropTarget(location);

        if (!dropTarget || !sourceTarget || dropTarget?.node.uniqueId !== node.uniqueId) {
          element.classList.remove("background-(--moss-background-success)", "background-(--moss-background-error)");
          return;
        }
        if (canDropNode(sourceTarget, dropTarget, node)) {
          element.classList.add("background-(--moss-background-success)");
        } else {
          element.classList.add("background-(--moss-background-error)");
        }
      },
      onDrop({ location, source }) {
        if (location.current?.dropTargets.length === 0 || location.current.dropTargets[0].data.type !== "TreeNode") {
          return;
        }

        const sourceTarget = getActualDropSourceTarget(source);
        const dropTarget = getActualDropTarget(location);

        if (dropTarget?.node.uniqueId !== node.uniqueId) {
          element.classList.remove("background-(--moss-background-success)", "background-(--moss-background-error)");
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

        element.classList.remove("background-(--moss-background-success)", "background-(--moss-background-error)");
      },
    });
  }, [dropTargetFolderRef, dropTargetListRef, node, treeId]);
};

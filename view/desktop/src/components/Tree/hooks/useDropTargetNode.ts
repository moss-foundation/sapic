import { RefObject, useEffect, useState } from "react";

import { attachInstruction, extractInstruction, Instruction } from "@atlaskit/pragmatic-drag-and-drop-hitbox/tree-item";
import { dropTargetForElements } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";

import { TreeNodeProps } from "../types";
import { canDropNode, getActualDropSourceTarget, getActualDropTarget } from "../utils";

export const useDropTargetNode = (
  node: TreeNodeProps,
  treeId: string | number,
  dropTargetListRef: RefObject<HTMLButtonElement>,
  depth: number
) => {
  const [instruction, setInstruction] = useState<Instruction | null>(null);

  useEffect(() => {
    const element = dropTargetListRef.current;
    if (!element) return;

    return dropTargetForElements({
      element,
      getData: ({ input, element }) => {
        const data = { type: "TreeNode", data: { treeId, node } };

        return attachInstruction(data, {
          input,
          element,
          indentPerLevel: 1,
          currentLevel: depth,
          mode: "expanded",
          block: [],
        });
      },
      canDrop({ source }) {
        return source.data.type === "TreeNode";
      },
      onDrag({ location }) {
        setInstruction(extractInstruction(location.current.dropTargets[0].data));
      },
      onDragLeave() {
        element.classList.remove("background-(--moss-success-background)", "background-(--moss-error-background)");
        setInstruction(null);
      },
      onDropTargetChange({ location, source }) {
        setInstruction(null);

        if (location.current?.dropTargets.length === 0 || location.current.dropTargets[0].data.type !== "TreeNode") {
          element.classList.remove("background-(--moss-success-background)", "background-(--moss-error-background)");
          return;
        }

        const sourceTarget = getActualDropSourceTarget(source);
        const dropTarget = getActualDropTarget(location);

        console.log({ sourceTarget, dropTarget });

        // console.log("onDropTargetChange", { location, source });

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
        setInstruction(null);
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
  }, [dropTargetListRef, node, treeId]);

  return { instruction };
};

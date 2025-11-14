import { RefObject, useContext, useEffect, useState } from "react";

import {
  attachInstruction,
  Availability,
  extractInstruction,
  Instruction,
  Operation,
} from "@atlaskit/pragmatic-drag-and-drop-hitbox/list-item";
import { combine } from "@atlaskit/pragmatic-drag-and-drop/combine";
import { draggable, dropTargetForElements } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";

import { ProjectDragType } from "../../constants";
import { ProjectTreeContext } from "../../ProjectTreeContext";
import { DragNode, ProjectTreeRootNode } from "../../types";
import {
  getAllNestedLocationProjectTreeData,
  getLocationProjectTreeData,
  getLocationProjectTreeRootNodeData,
  getSourceProjectTreeNodeData,
  hasDirectSimilarDescendant,
  isSourceProjectTreeNode,
  isSourceTreeRootNode,
} from "../../utils";
import { getTreeRootNodeSourceData } from "../../utils/TreeRoot";

interface UseDraggableRootNodeProps {
  dirRef: RefObject<HTMLUListElement | null>;
  triggerRef: RefObject<HTMLLIElement | null>;
  node: ProjectTreeRootNode;
  isRenamingNode: boolean;
}

export const useDraggableRootNode = ({ dirRef, triggerRef, node, isRenamingNode }: UseDraggableRootNodeProps) => {
  const { id, displayMode } = useContext(ProjectTreeContext);

  const [isDragging, setIsDragging] = useState<boolean>(false);
  const [instruction, setInstruction] = useState<Instruction | null>(null);
  const [dirInstruction, setDirInstruction] = useState<Instruction | null>(null);

  useEffect(() => {
    const triggerElement = triggerRef.current;
    const dirElement = dirRef.current;

    if (!triggerElement || !dirElement || isRenamingNode) return;

    return combine(
      draggable({
        element: triggerElement,
        getInitialData: () => ({
          type: ProjectDragType.ROOT_NODE,
          data: {
            node,
            projectId: id,
          },
        }),
        onDragStart: () => setIsDragging(true),
        onDrop: () => setIsDragging(false),
      }),
      dropTargetForElements({
        element: dirElement,
        canDrop: ({ source }) => isSourceTreeRootNode(source) || isSourceProjectTreeNode(source),
        getIsSticky: ({ source }) => isSourceTreeRootNode(source),
        getData: ({ input, source }) => {
          const dropTarget = {
            type: ProjectDragType.ROOT_NODE,
            node,
            projectId: id,
          };

          if (isSourceTreeRootNode(source)) {
            return attachInstruction(dropTarget, {
              element: dirElement,
              input,
              operations: {
                "reorder-before": "available",
                "reorder-after": "available",
                combine: "not-available",
              },
            });
          }

          if (isSourceProjectTreeNode(source)) {
            const sourceTarget = getSourceProjectTreeNodeData(source);
            if (sourceTarget) {
              return attachInstruction(dropTarget, {
                element: dirElement,
                input,
                operations: evaluateTreeNodeOperations(node, sourceTarget),
              });
            }
          }

          return attachInstruction(dropTarget, {
            element: dirElement,
            input,
            operations: {
              "reorder-before": "not-available",
              "reorder-after": "not-available",
              combine: "not-available",
            },
          });
        },
        onDrag: ({ source, location, self }) => {
          const sourceTarget = getTreeRootNodeSourceData(source);
          const dropTarget = getLocationProjectTreeData(location);
          const nestedDropTargets = getAllNestedLocationProjectTreeData(location);
          const rootDropTarget = getLocationProjectTreeRootNodeData(location);
          const instruction = extractInstruction(self.data);

          if (!sourceTarget) {
            setInstruction(null);
            setDirInstruction(null);
            return;
          }

          if (!rootDropTarget && !dropTarget) {
            if (
              instruction?.blocked &&
              node.childNodes.some((child) => child.id === nestedDropTargets[0].node.id) &&
              nestedDropTargets[0].instruction?.blocked &&
              nestedDropTargets[0].instruction?.operation !== "combine"
            ) {
              setInstruction(null);
              setDirInstruction(instruction);
              return;
            }

            setDirInstruction(null);
            setInstruction(null);
            return;
          }

          if (rootDropTarget) {
            setInstruction(null);
            setDirInstruction(instruction);
          }

          if (dropTarget) {
            setInstruction(instruction);
            setDirInstruction(null);
          }
        },
        onDragLeave: () => {
          setInstruction(null);
          setDirInstruction(null);
        },
        onDrop: () => {
          setInstruction(null);
          setDirInstruction(null);
        },
      })
    );
  }, [dirRef, displayMode, id, isRenamingNode, node, triggerRef]);

  return { isDragging, instruction, dirInstruction };
};

export const evaluateTreeNodeOperations = (
  dropTarget: ProjectTreeRootNode,
  sourceNode: DragNode
): {
  [TKey in Operation]?: Availability;
} => {
  const hasSimilarDescendant = hasDirectSimilarDescendant(dropTarget, sourceNode.node);

  return {
    "reorder-before": "not-available",
    "reorder-after": "not-available",
    combine: hasSimilarDescendant ? "blocked" : "available",
  };
};

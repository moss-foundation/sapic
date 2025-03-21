import { useEffect, useState } from "react";
import { NodeProps } from "../types";
import { draggable } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";
import { setCustomNativeDragPreview } from "@atlaskit/pragmatic-drag-and-drop/element/set-custom-native-drag-preview";

export const useDraggableNode = (
  draggableNodeRef: React.RefObject<HTMLButtonElement>,
  node: NodeProps,
  treeId: string | number,
  isRenamingNode: boolean,
  setPreview: React.Dispatch<React.SetStateAction<HTMLElement | null>>
) => {
  const [isDragging, setIsDragging] = useState<boolean>(false);
  useEffect(() => {
    const element = draggableNodeRef.current;
    if (!element || isRenamingNode) return;

    return draggable({
      element,
      getInitialData: () => ({
        type: "TreeNode",
        data: {
          node,
          treeId,
        },
      }),
      onDragStart() {
        setIsDragging(true);
      },
      onDrop: () => {
        setPreview(null);
        setIsDragging(false);
      },
      onGenerateDragPreview({ nativeSetDragImage }) {
        setCustomNativeDragPreview({
          nativeSetDragImage,
          render({ container }) {
            setPreview((prev) => (prev === container ? prev : container));
          },
        });
      },
    });
  }, [treeId, node, isRenamingNode, draggableNodeRef, setPreview]);

  return {
    isDragging,
  };
};

import { useEffect, useState } from "react";

import { draggable } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";
import { setCustomNativeDragPreview } from "@atlaskit/pragmatic-drag-and-drop/element/set-custom-native-drag-preview";

import { NodeProps } from "../types";

export const useDraggableNode = (
  draggableNodeRef: React.RefObject<HTMLButtonElement>,
  node: NodeProps,
  treeId: string | number,
  setPreview: React.Dispatch<React.SetStateAction<HTMLElement | null>>
) => {
  const [isDragging, setIsDragging] = useState<boolean>(false);

  useEffect(() => {
    const element = draggableNodeRef.current;
    if (!element) return;

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
  }, [treeId, node, draggableNodeRef, setPreview]);

  return {
    isDragging,
  };
};

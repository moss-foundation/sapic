import { useEffect } from "react";
import { NodeProps } from "../types";
import { draggable } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";
import { setCustomNativeDragPreview } from "@atlaskit/pragmatic-drag-and-drop/element/set-custom-native-drag-preview";

export const useDraggableNode = (
  draggableRef: React.RefObject<HTMLButtonElement>,
  node: NodeProps,
  treeId: string,
  isRenamingNode: boolean,
  setPreview: React.Dispatch<React.SetStateAction<HTMLElement | null>>
) => {
  useEffect(() => {
    const element = draggableRef.current;
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
      onDrop: () => {
        setPreview(null);
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
  }, [treeId, node, isRenamingNode, draggableRef, setPreview]);
};

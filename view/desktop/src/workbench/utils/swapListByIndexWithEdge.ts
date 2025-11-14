import { Edge } from "@atlaskit/pragmatic-drag-and-drop-hitbox/dist/types/types";

export const swapListByIndexWithEdge = <T>(fromIndex: number, toIndex: number, list: T[], edge?: Edge | null): T[] => {
  if (fromIndex === toIndex || fromIndex === -1 || toIndex === -1) return list;

  const updatedItems = [...list];

  if (!edge) {
    [updatedItems[fromIndex], updatedItems[toIndex]] = [updatedItems[toIndex], updatedItems[fromIndex]];
  } else {
    const [itemToMove] = updatedItems.splice(fromIndex, 1);

    const newToIndex = fromIndex < toIndex ? toIndex - 1 : toIndex;

    if (edge === "top" || edge === "left") {
      updatedItems.splice(newToIndex, 0, itemToMove);
    }

    if (edge === "bottom" || edge === "right") {
      updatedItems.splice(newToIndex + 1, 0, itemToMove);
    }
  }

  return updatedItems;
};

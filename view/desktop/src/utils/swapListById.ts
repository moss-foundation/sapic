import { Edge } from "@atlaskit/pragmatic-drag-and-drop-hitbox/dist/types/types";

interface Identifiable {
  id: string | number;
  order: number | null;
}

export const swapListById = <T extends Identifiable>(
  fromId: number | string,
  toId: number | string,
  list: T[],
  edge?: Edge | null
): T[] | null => {
  if (fromId === toId) return [...list];

  const fromIndex = list.findIndex((item) => item.id === fromId);
  const toIndex = list.findIndex((item) => item.id === toId);

  if (fromIndex === -1 || toIndex === -1) {
    return null;
  }

  const updatedItems: T[] = [...list];

  if (!edge) {
    [updatedItems[fromIndex], updatedItems[toIndex]] = [updatedItems[toIndex], updatedItems[fromIndex]];
  } else {
    const [itemToMove] = updatedItems.splice(fromIndex, 1);

    const newToIndex = fromIndex < toIndex ? toIndex - 1 : toIndex;

    if (edge === "top" || edge === "left") {
      updatedItems.splice(newToIndex, 0, itemToMove);
    } else {
      updatedItems.splice(newToIndex + 1, 0, itemToMove);
    }
  }

  updatedItems.forEach((item, index) => {
    item.order = index;
  });

  return updatedItems;
};

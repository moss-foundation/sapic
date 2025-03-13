import { Edge } from "@atlaskit/pragmatic-drag-and-drop-hitbox/dist/types/types";

interface Identifiable {
  id: number | string;
  order: number;
}

export const swapListById = <T extends Identifiable>(
  fromId: number | string,
  toId: number | string,
  list: T[],
  edge?: Edge
): T[] | null => {
  // If dragging an item onto itself, return the original list
  if (fromId === toId) {
    return list;
  }

  // Find the indices of the dragged and target items
  const fromIndex = list.findIndex((item) => item.id === fromId);
  const toIndex = list.findIndex((item) => item.id === toId);

  // If either item is not found, return null
  if (fromIndex === -1 || toIndex === -1) {
    return null;
  }

  // Create a copy of the list to avoid mutating the original
  const updatedItems = [...list];

  if (!edge) {
    // Case 1: No edge provided, swap the items
    [updatedItems[fromIndex], updatedItems[toIndex]] = [
      updatedItems[toIndex],
      updatedItems[fromIndex],
    ];
  } else {
    // Case 2: Edge provided, reorder based on drop position
    // Remove the dragged item
    const [itemToMove] = updatedItems.splice(fromIndex, 1);

    // Adjust target index after removal
    const newToIndex = fromIndex < toIndex ? toIndex - 1 : toIndex;

    // Insert based on edge: "top" or "left" before, "bottom" or "right" after
    if (edge === "top" || edge === "left") {
      updatedItems.splice(newToIndex, 0, itemToMove);
    } else {
      updatedItems.splice(newToIndex + 1, 0, itemToMove);
    }
  }

  // Update the order property of each item to reflect its new position
  updatedItems.forEach((item, index) => {
    item.order = index;
  });

  return updatedItems;
};
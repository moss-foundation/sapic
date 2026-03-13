import { ResourceNode } from "../types";

export const countNumberOfAllNestedItems = (node: ResourceNode): number => {
  if (!node.childNodes) return 0;
  return node.childNodes.reduce((acc, child) => {
    const childCount = child.kind === "Item" ? 1 : 0;
    return acc + childCount + countNumberOfAllNestedItems(child);
  }, 0);
};

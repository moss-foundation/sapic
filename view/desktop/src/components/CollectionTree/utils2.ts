import { ElementDragPayload } from "@atlaskit/pragmatic-drag-and-drop/dist/types/internal-types";
import { EntryInfo } from "@repo/moss-collection";

import { DragNode, TreeCollectionNode } from "./types";

//TODO order should always be set, it's a temporary solution until backend updates it's type
export const sortByOrder = <T extends { order?: number }>(entries: T[]) => {
  return [...entries].sort((a, b) => (a.order ?? 0) - (b.order ?? 0));
};

export const isSourceTreeNode = (source: ElementDragPayload): boolean => {
  return source.data.type === "TreeNode";
};

export const getSourceTreeNodeData = (source: ElementDragPayload): DragNode | null => {
  if (source.data.type === "TreeNode") {
    return source.data.data as DragNode;
  }

  return null;
};

export const getAllNestedEntries = (node: TreeCollectionNode): EntryInfo[] => {
  const result: EntryInfo[] = [];

  const { childNodes, ...entryInfo } = node;
  result.push(entryInfo);

  for (const child of childNodes) {
    result.push(...getAllNestedEntries(child));
  }

  return result;
};

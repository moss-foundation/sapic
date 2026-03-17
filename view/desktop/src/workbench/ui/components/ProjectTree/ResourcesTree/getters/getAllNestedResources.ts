import { ListProjectResourceItem } from "@repo/ipc";

import { ResourceNode } from "../types";

export const getAllNestedResources = (node: ResourceNode): ListProjectResourceItem[] => {
  const result: ListProjectResourceItem[] = [];

  result.push({
    id: node.id,
    name: node.name,
    kind: node.kind,
    class: node.class,
    path: node.path,
    protocol: node.protocol,
  });

  for (const child of node.childNodes) {
    result.push(...getAllNestedResources(child));
  }

  const sortedResult = result.sort((a, b) => a.path.segments.length - b.path.segments.length);

  return sortedResult;
};

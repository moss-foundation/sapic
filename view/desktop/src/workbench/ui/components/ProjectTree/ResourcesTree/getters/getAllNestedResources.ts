import { ResourceNode } from "../types";

export const getAllNestedResources = (node: ResourceNode): ResourceNode[] => {
  const result: ResourceNode[] = [];

  result.push({
    id: node.id,
    name: node.name,
    kind: node.kind,
    class: node.class,
    path: node.path,
    protocol: node.protocol,
    order: node.order,
    expanded: node.expanded,
    childNodes: [],
  });

  for (const child of node.childNodes) {
    result.push(...getAllNestedResources(child));
  }

  const sortedResult = result.sort((a, b) => a.path.segments.length - b.path.segments.length);

  return sortedResult;
};

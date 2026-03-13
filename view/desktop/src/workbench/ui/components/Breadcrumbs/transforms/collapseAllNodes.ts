import { ResourceNode } from "../../ProjectTree/ResourcesTree/types";

export const collapseAllNodes = <T extends ResourceNode>(node: T): T => {
  return {
    ...node,
    expanded: node.kind === "Dir" ? false : node.expanded,
    childNodes: node.childNodes.map((child) => collapseAllNodes(child)),
  };
};

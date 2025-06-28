import { TreeCollectionNode, TreeCollectionRootNode } from "../types";

export const prepareCollectionForTree = (tree: TreeCollectionRootNode) => {
  return {
    ...tree,
    requests: assignOrderToNode(tree.requests),
    endpoints: assignOrderToNode(tree.endpoints),
    components: assignOrderToNode(tree.components),
    schemas: assignOrderToNode(tree.schemas),
  };
};

const assignOrderToNode = (node: TreeCollectionNode) => {
  if (node.kind === "Item") {
    return node;
  }

  return {
    ...node,
    order: node.order,
    childNodes: assignOrderToNodes(node.childNodes),
  };
};

const assignOrderToNodes = (nodes: TreeCollectionNode[]) => {
  return nodes.map((node, index) => ({
    ...node,
    order: node.order ?? index + 1,
    childNodes: assignOrderToNodes(node.childNodes),
  }));
};

import { TreeCollectionNode, TreeCollectionRootNode } from "../types";

export const updateNodeInTree = (
  tree: TreeCollectionRootNode,
  updatedNode: TreeCollectionNode
): TreeCollectionRootNode => {
  switch (updatedNode.class) {
    case "Request":
      return {
        ...tree,
        requests: updateNode(tree.requests, updatedNode),
      };
    case "Endpoint":
      return {
        ...tree,
        endpoints: updateNode(tree.endpoints, updatedNode),
      };
    case "Component":
      return {
        ...tree,
        components: updateNode(tree.components, updatedNode),
      };
    case "Schema":
      return {
        ...tree,
        schemas: updateNode(tree.schemas, updatedNode),
      };
    default:
      return tree;
  }
};

const updateNode = (node: TreeCollectionNode, updatedNode: TreeCollectionNode): TreeCollectionNode => {
  if (node.id === updatedNode.id) return updatedNode;

  if (node.childNodes.length > 0) {
    return {
      ...node,
      childNodes: updateNodeInArray(node.childNodes, updatedNode),
    };
  }

  return node;
};

const updateNodeInArray = (array: TreeCollectionNode[], updatedNode: TreeCollectionNode): TreeCollectionNode[] => {
  return array.map((node) => {
    if (node.id === updatedNode.id) return updatedNode;

    if (node.childNodes.length > 0) {
      node.childNodes = updateNodeInArray(node.childNodes, updatedNode);
    }

    return node;
  });
};

//check if all folders are expanded
export const checkIfAllFoldersAreExpanded = (tree: TreeCollectionRootNode): boolean => {
  const checkIfAllNodesAreExpanded = (node: TreeCollectionNode): boolean => {
    if (!node || node.childNodes.length === 0) return true;

    if (node.kind === "Item") {
      return true;
    }

    return node.childNodes.every(checkIfAllNodesAreExpanded);
  };

  return [tree.requests, tree.endpoints, tree.components, tree.schemas].every(checkIfAllNodesAreExpanded);
};

export const checkIfAllFoldersAreCollapsed = (tree: TreeCollectionRootNode): boolean => {
  const checkIfAllNodesAreCollapsed = (node: TreeCollectionNode): boolean => {
    if (node.kind === "Item") {
      return true;
    }

    if (!node || node.childNodes.length === 0) return true;

    return node.childNodes.every(checkIfAllNodesAreCollapsed);
  };
  return [tree.requests, tree.endpoints, tree.components, tree.schemas].every(checkIfAllNodesAreCollapsed);
};

//expand all nodes
export const expandAllNodes = (tree: TreeCollectionRootNode): TreeCollectionRootNode => {
  const expandNodes = (node: TreeCollectionNode): TreeCollectionNode => {
    if (node.kind === "Item") {
      return node;
    }

    return {
      ...node,
      expanded: true,
      childNodes: node.childNodes.map(expandNodes),
    };
  };

  return {
    ...tree,
    requests: expandNodes(tree.requests),
    endpoints: expandNodes(tree.endpoints),
    components: expandNodes(tree.components),
    schemas: expandNodes(tree.schemas),
  };
};

//collapse all nodes
export const collapseAllNodes = (tree: TreeCollectionRootNode): TreeCollectionRootNode => {
  const collapseNodes = (node: TreeCollectionNode): TreeCollectionNode => {
    if (node.kind === "Item") {
      return node;
    }

    return {
      ...node,
      expanded: false,
      childNodes: node.childNodes.map(collapseNodes),
    };
  };

  return {
    ...tree,
    requests: collapseNodes(tree.requests),
    endpoints: collapseNodes(tree.endpoints),
    components: collapseNodes(tree.components),
    schemas: collapseNodes(tree.schemas),
  };
};

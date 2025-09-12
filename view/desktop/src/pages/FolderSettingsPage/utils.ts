import { TreeCollectionNode, TreeCollectionRootNode } from "@/components/CollectionTree/types";
import { Icons } from "@/lib/ui";

export const getFolderIcon = (node: TreeCollectionNode): Icons => {
  const isRoot = node.path.segments.length === 1;

  if (isRoot) {
    switch (node.class) {
      case "Schema":
        return "SchemasFolder";
      case "Endpoint":
        return "EndpointsFolder";
      case "Component":
        return "ComponentsFolder";
      default:
        return "RequestsFolder";
    }
  }

  return "Folder";
};

export const findNodeInCollection = (collection: TreeCollectionRootNode, searchId: string) => {
  // Search in all categories

  for (const category of collection.childNodes) {
    if (category.id === searchId) return category;

    // Recursively search child nodes
    const findInChildren = (node: TreeCollectionNode): TreeCollectionNode | undefined => {
      if (node.id === searchId) return node;
      for (const child of node.childNodes) {
        const found = findInChildren(child);
        if (found) return found;
      }
      return undefined;
    };

    const found = findInChildren(category);
    if (found) return found;
  }
  return undefined;
};

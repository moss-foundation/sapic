import { useMemo } from "react";

import { TreeCollectionNode, TreeCollectionRootNode } from "@/components/CollectionTree/types";

import { useStreamedCollectionsWithEntries } from "./useStreamedCollectionsWithEntries";

export interface useCollectionsTreesProps {
  collectionsTrees: TreeCollectionRootNode[];
  isLoading: boolean;
}

export const useCollectionsTrees = (): useCollectionsTreesProps => {
  const {
    data: collectionsWithEntries,
    isEntriesLoading,
    isLoading: isCollectionsLoading,
  } = useStreamedCollectionsWithEntries();

  const isLoading = isEntriesLoading || isCollectionsLoading;

  const validCollections = useMemo(() => {
    return collectionsWithEntries.filter((collection) => collection.entries.length >= 4);
  }, [collectionsWithEntries]);

  const collectionsTrees = useMemo(() => {
    return validCollections.map((collection) => {
      const { entries, isEntriesLoading, entriesError, ...rest } = collection;

      // Create category nodes with proper structure
      const createCategoryNode = (
        categoryName: string,
        categoryClass: "Request" | "Endpoint" | "Component" | "Schema"
      ): TreeCollectionNode => ({
        id: `${collection.id}-${categoryName}`,
        name: categoryName,
        path: {
          raw: categoryName,
          segments: [categoryName],
        },
        class: categoryClass,
        kind: "Dir" as const,
        expanded: false,
        childNodes: [],
      });

      const endpoints = createCategoryNode("endpoints", "Endpoint");
      const schemas = createCategoryNode("schemas", "Schema");
      const components = createCategoryNode("components", "Component");
      const requests = createCategoryNode("requests", "Request");

      // Map entry class to collection category nodes
      const categoryMap: { [key: string]: TreeCollectionNode } = {
        "Request": requests,
        "Endpoint": endpoints,
        "Component": components,
        "Schema": schemas,
      };

      // Distribute entries based on their class and path structure
      entries.forEach((entry) => {
        const rootNode = categoryMap[entry.class];
        if (!rootNode) {
          console.error(`Unknown class ${entry.class}`);
          return;
        }

        // If path has one component (root node)
        if (entry.path.segments.length === 1) {
          // Update root node properties, preserving childNodes
          const existingChildNodes = rootNode.childNodes;
          Object.assign(rootNode, entry, { childNodes: existingChildNodes });
          return;
        }

        // Handle nested paths - similar to Zustand store logic
        let currentNode = rootNode;
        const relativePath = entry.path.segments.slice(1); // Skip the category name

        // Navigate or create intermediate directories
        for (let i = 0; i < relativePath.length - 1; i++) {
          const component = relativePath[i];
          const pathSoFar = entry.path.segments.slice(0, i + 2); // Include category + path up to current component

          let child = currentNode.childNodes.find((node) => node.name === component && node.kind === "Dir");

          if (!child) {
            child = {
              id: `${collection.id}-${pathSoFar.join("-")}`, // Generate unique ID for intermediate directory
              name: component,
              path: {
                raw: pathSoFar.join("/"),
                segments: pathSoFar,
              },
              class: entry.class,
              kind: "Dir",
              protocol: undefined,
              order: undefined,
              expanded: false,
              childNodes: [],
            };
            currentNode.childNodes.push(child);
          }
          currentNode = child;
        }

        // Handle the final component
        const lastComponent = relativePath[relativePath.length - 1];
        const existingNode = currentNode.childNodes.find((node) => node.name === lastComponent);

        if (existingNode) {
          // Update existing node, preserving childNodes
          const existingChildNodes = existingNode.childNodes;
          Object.assign(existingNode, entry, { childNodes: existingChildNodes });
        } else {
          // Create new node
          const newNode: TreeCollectionNode = {
            ...entry,
            childNodes: [],
          };
          currentNode.childNodes.push(newNode);
        }
      });

      return {
        ...rest,
        expanded: true,
        endpoints,
        schemas,
        components,
        requests,
      };
    });
  }, [validCollections]);

  return { collectionsTrees, isLoading };
};

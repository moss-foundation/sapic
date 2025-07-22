import { useMemo } from "react";

import { TreeCollectionNode, TreeCollectionRootNode } from "@/components/CollectionTree/types";

import { useStreamedCollectionsWithEntries } from "./useStreamedCollectionsWithEntries";

export interface UseCollectionsTreesProps {
  collectionsTrees: TreeCollectionRootNode[];
  isLoading: boolean;
}

export const useCollectionsTrees = (): UseCollectionsTreesProps => {
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

      const categoryMap: { [key: string]: TreeCollectionNode } = {
        "Request": requests,
        "Endpoint": endpoints,
        "Component": components,
        "Schema": schemas,
      };

      entries.forEach((entry) => {
        const rootNode = categoryMap[entry.class];
        if (!rootNode) {
          console.error(`Unknown class ${entry.class}`);
          return;
        }

        if (entry.path.segments.length === 1) {
          const existingChildNodes = rootNode.childNodes;
          Object.assign(rootNode, entry, { childNodes: existingChildNodes });
          return;
        }

        let currentNode = rootNode;
        const relativePath = entry.path.segments.slice(1);

        for (let i = 0; i < relativePath.length - 1; i++) {
          const component = relativePath[i];
          const pathSoFar = entry.path.segments.slice(0, i + 2);

          let child = currentNode.childNodes.find((node) => node.name === component && node.kind === "Dir");

          if (!child) {
            child = {
              id: `${collection.id}-${pathSoFar.join("-")}`,
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

        const lastComponent = relativePath[relativePath.length - 1];
        const existingNode = currentNode.childNodes.find((node) => node.name === lastComponent);

        if (existingNode) {
          const existingChildNodes = existingNode.childNodes;
          Object.assign(existingNode, entry, { childNodes: existingChildNodes });
        } else {
          const newNode: TreeCollectionNode = {
            ...entry,
            childNodes: [],
          };
          currentNode.childNodes.push(newNode);
        }
      });

      return {
        ...rest,
        expanded: rest.expanded ?? true, // TODO expanded should be set
        endpoints,
        schemas,
        components,
        requests,
      };
    });
  }, [validCollections]);

  return { collectionsTrees, isLoading };
};

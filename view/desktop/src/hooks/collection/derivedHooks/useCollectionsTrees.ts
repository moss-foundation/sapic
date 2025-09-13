import { useMemo } from "react";

import { ProjectTreeNode, ProjectTreeRootNode } from "@/components/ProjectTree/types";
import { sortObjectsByOrder } from "@/utils/sortObjectsByOrder";

import { useStreamedCollectionsWithEntries } from "./useStreamedCollectionsWithEntries";

export interface UseCollectionsTreesProps {
  collectionsTrees: ProjectTreeRootNode[];
  collectionsTreesSortedByOrder: ProjectTreeRootNode[];
  isLoading: boolean;
}

export const useCollectionsTrees = (): UseCollectionsTreesProps => {
  const {
    data: collectionsWithEntries,
    isEntriesLoading,
    isLoading: isCollectionsLoading,
  } = useStreamedCollectionsWithEntries();

  const isLoading = isEntriesLoading || isCollectionsLoading;

  const collectionsTrees: ProjectTreeRootNode[] = useMemo(() => {
    return collectionsWithEntries.map((collection): ProjectTreeRootNode => {
      const {
        entries,
        isEntriesLoading: _isEntriesLoading,
        entriesError: _entriesError,
        ...collectionTree
      } = collection;

      const childNodes: ProjectTreeNode[] = [];

      entries.forEach((entry) => {
        if (entry.path.segments.length === 1) {
          // Root level entry - add directly to childNodes
          const existingNode = childNodes.find((node) => node.id === entry.id);
          if (existingNode) {
            const existingChildNodes = existingNode.childNodes;
            Object.assign(existingNode, entry, { childNodes: existingChildNodes });
          } else {
            const newNode: ProjectTreeNode = {
              ...entry,
              childNodes: [],
            };
            childNodes.push(newNode);
          }
          return;
        }

        // Nested entry - build the tree structure
        let currentNode: ProjectTreeNode | undefined;
        const pathSegments = entry.path.segments;

        for (let i = 0; i < pathSegments.length - 1; i++) {
          const component = pathSegments[i];
          const pathSoFar = pathSegments.slice(0, i + 1);

          const targetArray = i === 0 ? childNodes : currentNode?.childNodes || [];
          let child = targetArray.find((node) => node.name === component && node.kind === "Dir");

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
            targetArray.push(child);
          }
          currentNode = child;
        }

        // Add the final entry
        const lastComponent = pathSegments[pathSegments.length - 1];
        const targetArray = currentNode?.childNodes || childNodes;
        const existingNode = targetArray.find((node) => node.name === lastComponent);

        if (existingNode) {
          const existingChildNodes = existingNode.childNodes;
          Object.assign(existingNode, entry, { childNodes: existingChildNodes });
        } else {
          const newNode: ProjectTreeNode = {
            ...entry,
            childNodes: [],
          };
          targetArray.push(newNode);
        }
      });

      return {
        ...collectionTree,
        childNodes,
      };
    });
  }, [collectionsWithEntries]);

  const collectionsTreesSortedByOrder = useMemo(() => {
    return sortObjectsByOrder(collectionsTrees);
  }, [collectionsTrees]);

  // console.log({ collectionsWithEntries, collectionsTrees });

  return { collectionsTrees, collectionsTreesSortedByOrder, isLoading };
};

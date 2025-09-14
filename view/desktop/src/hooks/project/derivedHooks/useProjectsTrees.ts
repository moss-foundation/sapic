import { useMemo } from "react";

import { ProjectTreeNode, ProjectTreeRootNode } from "@/components/ProjectTree/types";
import { sortObjectsByOrder } from "@/utils/sortObjectsByOrder";

import { useStreamedProjectsWithEntries } from "./useStreamedProjectsWithEntries";

export interface UseProjectsTreesProps {
  projectsTrees: ProjectTreeRootNode[];
  projectsTreesSortedByOrder: ProjectTreeRootNode[];
  isLoading: boolean;
}

export const useProjectsTrees = (): UseProjectsTreesProps => {
  const {
    data: projectsWithEntries,
    isEntriesLoading,
    isLoading: areProjectsLoading,
  } = useStreamedProjectsWithEntries();

  const isLoading = isEntriesLoading || areProjectsLoading;

  const projectsTrees: ProjectTreeRootNode[] = useMemo(() => {
    return projectsWithEntries.map((project): ProjectTreeRootNode => {
      const { entries, isEntriesLoading: _isEntriesLoading, entriesError: _entriesError, ...projectTree } = project;

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
              id: `${project.id}-${pathSoFar.join("-")}`,
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
        ...projectTree,
        childNodes,
      };
    });
  }, [projectsWithEntries]);

  const projectsTreesSortedByOrder = useMemo(() => {
    return sortObjectsByOrder(projectsTrees);
  }, [projectsTrees]);

  return {
    projectsTrees,
    projectsTreesSortedByOrder,
    isLoading,
  };
};

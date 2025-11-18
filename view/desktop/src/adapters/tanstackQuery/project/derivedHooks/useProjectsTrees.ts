import { useMemo } from "react";

import { sortObjectsByOrder } from "@/utils/sortObjectsByOrder";
import { ProjectTreeNode, ProjectTreeRootNode } from "@/workbench/ui/components/ProjectTree/types";

import { useStreamedProjectsWithResources } from "./useStreamedProjectsWithResources";

export interface UseProjectsTreesProps {
  projectsTrees: ProjectTreeRootNode[];
  projectsTreesSortedByOrder: ProjectTreeRootNode[];
  isLoading: boolean;
}

export const useProjectsTrees = (): UseProjectsTreesProps => {
  const {
    data: projectsWithResources,
    areResourcesLoading,
    isLoading: areProjectsLoading,
  } = useStreamedProjectsWithResources();

  const isLoading = areResourcesLoading || areProjectsLoading;

  const projectsTrees: ProjectTreeRootNode[] = useMemo(() => {
    return projectsWithResources.map((project): ProjectTreeRootNode => {
      const { resources, ...projectTree } = project;

      const childNodes: ProjectTreeNode[] = [];

      resources.forEach((resource) => {
        if (resource.path.segments.length === 1) {
          // Root level resource - add directly to childNodes
          const existingNode = childNodes.find((node) => node.id === resource.id);
          if (existingNode) {
            const existingChildNodes = existingNode.childNodes;
            Object.assign(existingNode, resource, { childNodes: existingChildNodes });
          } else {
            const newNode: ProjectTreeNode = {
              ...resource,
              childNodes: [],
            };
            childNodes.push(newNode);
          }
          return;
        }

        // Nested resource - build the tree structure
        let currentNode: ProjectTreeNode | undefined;
        const pathSegments = resource.path.segments;

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
              class: resource.class,
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

        // Add the final resource
        const lastComponent = pathSegments[pathSegments.length - 1];
        const targetArray = currentNode?.childNodes || childNodes;
        const existingNode = targetArray.find((node) => node.name === lastComponent);

        if (existingNode) {
          const existingChildNodes = existingNode.childNodes;
          Object.assign(existingNode, resource, { childNodes: existingChildNodes });
        } else {
          const newNode: ProjectTreeNode = {
            ...resource,
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
  }, [projectsWithResources]);

  const projectsTreesSortedByOrder = useMemo(() => {
    return sortObjectsByOrder(projectsTrees);
  }, [projectsTrees]);

  return {
    projectsTrees,
    projectsTreesSortedByOrder,
    isLoading,
  };
};

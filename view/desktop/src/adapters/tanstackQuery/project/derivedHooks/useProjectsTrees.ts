import { useEffect, useMemo, useState } from "react";

import { useGetAllLocalProjectSummaries } from "@/db/projectSummaries/hooks/useGetAllLocalProjectSummaries";
import { useGetAllLocalResourceSummaries } from "@/db/resourceSummaries/hooks/useGetAllLocalResourceSummaries";
import { useCurrentWorkspace } from "@/hooks";
import { sortObjectsByOrder } from "@/utils/sortObjectsByOrder";
import { ProjectTreeNode, ProjectTreeRootNode } from "@/workbench/ui/components/ProjectTree/types";
import { useSyncProjectSummaries } from "@/workbench/ui/parts/ProjectTreesView/hooks/useSyncProjectSummaries";
import { useSyncResourceSummaries } from "@/workbench/ui/parts/ProjectTreesView/hooks/useSyncResourceSummaries";

export interface UseProjectsTreesProps {
  projectsTrees: ProjectTreeRootNode[];
  projectsTreesSortedByOrder: ProjectTreeRootNode[];
  isLoading: boolean;
}

export const useProjectsTrees = (): UseProjectsTreesProps => {
  const { currentWorkspaceId } = useCurrentWorkspace();

  const { isLoading: areProjectsLoading } = useSyncProjectSummaries();
  const { isLoading: areResourcesLoading } = useSyncResourceSummaries();

  const localProjectSummaries = useGetAllLocalProjectSummaries();
  const localResourceSummaries = useGetAllLocalResourceSummaries();

  const [projectsTrees, setProjectsTrees] = useState<ProjectTreeRootNode[]>([]);

  const isLoading = areResourcesLoading || areProjectsLoading;

  useEffect(() => {
    if (isLoading) return;

    const buildProjectsTrees = async () => {
      const trees = await Promise.all(
        localProjectSummaries.map(async (projectSummary): Promise<ProjectTreeRootNode> => {
          const resources = localResourceSummaries
            .filter((resource) => resource.projectId === projectSummary.id)
            .sort((a, b) => a.path.segments.length - b.path.segments.length);

          const childNodes: ProjectTreeNode[] = [];

          for (const resource of resources) {
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
              continue;
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
                  id: `${projectSummary.id}-${pathSoFar.join("-")}`,
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
          }

          return {
            ...projectSummary,
            id: projectSummary.id,
            name: projectSummary.name,
            expanded: projectSummary.expanded,
            archived: projectSummary.archived,
            branch: projectSummary.branch ?? undefined,
            iconPath: projectSummary.iconPath ?? undefined,
            childNodes,
          };
        })
      );

      setProjectsTrees(trees);
    };

    if (localProjectSummaries.length > 0) {
      buildProjectsTrees();
    } else {
      setProjectsTrees([]);
    }
  }, [currentWorkspaceId, isLoading, localProjectSummaries, localResourceSummaries]);

  const projectsTreesSortedByOrder = useMemo(() => {
    return sortObjectsByOrder(projectsTrees);
  }, [projectsTrees]);

  return {
    projectsTrees,
    projectsTreesSortedByOrder,
    isLoading,
  };
};

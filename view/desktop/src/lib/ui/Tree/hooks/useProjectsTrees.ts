import { useMemo } from "react";

import { useGetAllLocalProjectSummaries } from "@/db/projectSummaries/hooks/useGetAllLocalProjectSummaries";
import { useSyncProjectSummaries } from "@/db/projectSummaries/hooks/useSyncProjectSummaries";
import { useGetAllLocalResourceSummaries } from "@/db/resourceSummaries/hooks/useGetAllLocalResourceSummaries";
import { useSyncResourceSummaries } from "@/db/resourceSummaries/hooks/useSyncResourceSummaries";
import { LocalResourceSummary } from "@/db/resourceSummaries/types";
import { sortObjectsByOrder } from "@/utils/sortObjectsByOrder";
import { ProjectTreeNode, ProjectTreeRootNode } from "@/workbench/ui/components/ProjectTree/types";

export interface UseProjectsTreesProps {
  projectsTrees: ProjectTreeRootNode[];
  projectsTreesSortedByOrder: ProjectTreeRootNode[];
  isLoading: boolean;
}

export const useProjectsTrees = (): UseProjectsTreesProps => {
  const { isPending: areProjectsPending } = useSyncProjectSummaries();
  const { isPending: areResourcesPending } = useSyncResourceSummaries();

  const { data: localProjectSummaries = [] } = useGetAllLocalProjectSummaries();
  const { data: localResourceSummaries = [] } = useGetAllLocalResourceSummaries();

  const isLoading = areResourcesPending || areProjectsPending;

  const projectsTrees = useMemo(() => {
    if (isLoading || localProjectSummaries.length === 0) return [];

    return localProjectSummaries.map(
      (projectSummary): ProjectTreeRootNode => ({
        ...projectSummary,
        id: projectSummary.id,
        name: projectSummary.name,
        expanded: projectSummary.expanded,
        archived: projectSummary.archived,
        branch: projectSummary.branch ?? undefined,
        iconPath: projectSummary.iconPath ?? undefined,
        childNodes: buildProjectTreeNodes(projectSummary.id, localResourceSummaries),
      })
    );
  }, [isLoading, localProjectSummaries, localResourceSummaries]);

  const projectsTreesSortedByOrder = useMemo(() => {
    return sortObjectsByOrder(projectsTrees);
  }, [projectsTrees]);

  return {
    projectsTrees,
    projectsTreesSortedByOrder,
    isLoading,
  };
};

const buildProjectTreeNodes = (projectId: string, allResources: LocalResourceSummary[]): ProjectTreeNode[] => {
  const projectResources = allResources
    .filter((resource) => resource.projectId === projectId)
    .sort((a, b) => a.path.segments.length - b.path.segments.length);

  const childNodes: ProjectTreeNode[] = [];

  for (const resource of projectResources) {
    if (resource.path.segments.length === 1) {
      // Root level resource
      const existingNode = childNodes.find((node) => node.id === resource.id);
      if (existingNode) {
        Object.assign(existingNode, resource, { childNodes: existingNode.childNodes });
      } else {
        childNodes.push(resourceToTreeNode(resource));
      }
      continue;
    }

    // Nested resource
    let currentNode: ProjectTreeNode | undefined;
    const pathSegments = resource.path.segments;

    for (let i = 0; i < pathSegments.length - 1; i++) {
      const component = pathSegments[i];
      const pathSoFar = pathSegments.slice(0, i + 1);
      const targetArray = i === 0 ? childNodes : currentNode?.childNodes || [];

      currentNode = getOrCreateDirNode(targetArray, component, projectId, pathSoFar, resource);
    }

    // Add the final resource
    const lastComponent = pathSegments[pathSegments.length - 1];
    const targetArray = currentNode?.childNodes || childNodes;
    const existingNode = targetArray.find((node) => node.name === lastComponent);

    if (existingNode) {
      Object.assign(existingNode, resource, { childNodes: existingNode.childNodes });
    } else {
      targetArray.push(resourceToTreeNode(resource));
    }
  }

  return childNodes;
};

const resourceToTreeNode = (resource: LocalResourceSummary): ProjectTreeNode => ({
  ...resource,
  expanded: resource.expanded ?? false,
  order: resource.order,
  childNodes: [],
});

const getOrCreateDirNode = (
  parentChildren: ProjectTreeNode[],
  segment: string,
  projectId: string,
  pathSoFar: string[],
  template: LocalResourceSummary
): ProjectTreeNode => {
  let dir = parentChildren.find((n) => n.name === segment && n.kind === "Dir");
  if (!dir) {
    dir = {
      id: `${projectId}-${pathSoFar.join("-")}`,
      name: segment,
      path: { raw: pathSoFar.join("/"), segments: pathSoFar },
      class: template.class,
      kind: "Dir",
      protocol: undefined,
      order: template.order,
      expanded: template.expanded ?? false,
      childNodes: [],
    } satisfies ProjectTreeNode;
    parentChildren.push(dir);
  }
  return dir;
};

import { useMemo } from "react";

import { useGetResourcesSummariesByProjectId } from "@/db/resourceSummaries/hooks/useGetResourcesSummariesByProjectId";
import { LocalResourceSummary } from "@/db/resourceSummaries/types";
import { sortObjectsByOrder } from "@/utils/sortObjectsByOrder";
import { ResourceNode } from "@/workbench/ui/components/ProjectTree/types";

export const useResourcesNodes = (projectId: string) => {
  const { data: localResourceSummaries } = useGetResourcesSummariesByProjectId(projectId);

  const rootResourcesNodes = useMemo(() => {
    return sortObjectsByOrder(buildResourceTreeNodes(projectId, localResourceSummaries));
  }, [projectId, localResourceSummaries]);

  return {
    rootResourcesNodes,
  };
};

const resourceToTreeNode = (resource: LocalResourceSummary): ResourceNode => ({
  ...resource,
  expanded: resource.expanded ?? false,
  order: resource.order,
  childNodes: [],
});

const getOrCreateDirNode = (
  parentChildren: ResourceNode[],
  segment: string,
  projectId: string,
  pathSoFar: string[],
  template: LocalResourceSummary
): ResourceNode => {
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
    } satisfies ResourceNode;
    parentChildren.push(dir);
  }
  return dir;
};

const buildResourceTreeNodes = (projectId: string, resources: LocalResourceSummary[]): ResourceNode[] => {
  const sorted = [...resources].sort((a, b) => a.path.segments.length - b.path.segments.length);

  const childNodes: ResourceNode[] = [];

  for (const resource of sorted) {
    if (resource.path.segments.length === 1) {
      const existingNode = childNodes.find((node) => node.id === resource.id);
      if (existingNode) {
        Object.assign(existingNode, resource, { childNodes: existingNode.childNodes });
      } else {
        childNodes.push(resourceToTreeNode(resource));
      }
      continue;
    }

    let currentNode: ResourceNode | undefined;
    const pathSegments = resource.path.segments;

    for (let i = 0; i < pathSegments.length - 1; i++) {
      const component = pathSegments[i];
      const pathSoFar = pathSegments.slice(0, i + 1);
      const targetArray = i === 0 ? childNodes : currentNode?.childNodes || [];

      currentNode = getOrCreateDirNode(targetArray, component, projectId, pathSoFar, resource);
    }

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

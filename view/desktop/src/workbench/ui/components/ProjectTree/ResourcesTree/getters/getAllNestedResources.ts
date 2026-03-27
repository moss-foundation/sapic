import { resourceService } from "@/domains/resource/resourceService";

import { ResourceNodeWithDetails } from "../dnd/types.dnd";
import { ResourceNode } from "../types";

interface GetAllNestedResourcesProps {
  node: ResourceNode;
  projectId: string;
}

export const getAllNestedResources = async ({
  node,
  projectId,
}: GetAllNestedResourcesProps): Promise<ResourceNodeWithDetails[]> => {
  const result: ResourceNodeWithDetails[] = [];

  const nodeDescription = await resourceService.describe(projectId, node.id);
  result.push({
    id: node.id,
    name: node.name,
    kind: node.kind,
    class: node.class,
    path: node.path,
    protocol: node.protocol,
    order: node.order,
    expanded: node.expanded,
    details: nodeDescription,
    childNodes: [],
  });

  await Promise.all(
    node.childNodes.map(async (child) => {
      const childDescription = await resourceService.describe(projectId, child.id);
      result.push({
        ...child,
        details: childDescription,
      });
    })
  );

  const sortedResult = result.sort((a, b) => a.path.segments.length - b.path.segments.length);

  return sortedResult;
};

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

  const collectResources = async (currentNode: ResourceNode) => {
    const description = await resourceService.describe(projectId, currentNode.id);
    result.push({
      ...currentNode,
      details: description,
    });
    await Promise.all(currentNode.childNodes.map((child) => collectResources(child)));
  };

  await collectResources(node);

  return result.sort((a, b) => a.path.segments.length - b.path.segments.length);
};

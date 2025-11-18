import { sortObjectsByOrder } from "@/utils/sortObjectsByOrder";
import {
  USE_STREAM_PROJECT_RESOURCES_QUERY_KEY,
  useDeleteProjectResource,
} from "@/workbench/adapters/tanstackQuery/project";
import { useBatchUpdateProjectResource } from "@/workbench/adapters/tanstackQuery/project/useBatchUpdateProjectResource";
import { StreamResourcesEvent } from "@repo/moss-project";
import { useQueryClient } from "@tanstack/react-query";

import { ProjectTreeNode, ProjectTreeRootNode } from "../types";
import { siblingsAfterRemovalPayload } from "../utils";

export const useDeleteAndUpdatePeers = (
  projectId: string,
  node: ProjectTreeNode,
  parentNode: ProjectTreeNode | ProjectTreeRootNode
) => {
  const queryClient = useQueryClient();

  const { mutateAsync: deleteProjectResource } = useDeleteProjectResource();
  const { mutateAsync: batchUpdateProjectResource } = useBatchUpdateProjectResource();

  const deleteAndUpdatePeers = async () => {
    await deleteProjectResource({
      projectId,
      input: {
        id: node.id,
      },
    });

    const sortedChildren = sortObjectsByOrder(parentNode.childNodes);
    const index = sortedChildren.findIndex((e) => e.id === node.id) + 1;
    const updatedParentNodeChildren = sortedChildren.slice(index).map((e) => ({
      ...e,
      order: e.order! - 1,
    }));

    const result = await batchUpdateProjectResource({
      projectId,
      resources: {
        resources: siblingsAfterRemovalPayload({
          nodes: parentNode.childNodes,
          removedNode: node,
        }),
      },
    });

    if (result.status === "ok") {
      queryClient.setQueryData(
        [USE_STREAM_PROJECT_RESOURCES_QUERY_KEY, projectId],
        (cacheData: StreamResourcesEvent[]) => {
          return cacheData.map((cacheResource) => {
            if (updatedParentNodeChildren.some((resource) => resource.id === cacheResource.id)) {
              const updatedResource = updatedParentNodeChildren.find((resource) => resource.id === cacheResource.id);
              return { ...cacheResource, ...updatedResource };
            }

            return cacheResource;
          });
        }
      );
    }
  };

  return {
    deleteAndUpdatePeers,
  };
};

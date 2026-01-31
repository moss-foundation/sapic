import { useDeleteProjectResource } from "@/adapters";
import { useBatchUpdateProjectResource } from "@/adapters/tanstackQuery/resource/useBatchUpdateProjectResource";
import { USE_STREAM_PROJECT_RESOURCES_QUERY_KEY } from "@/adapters/tanstackQuery/resource/useStreamProjectResources";
import { useCurrentWorkspace } from "@/hooks";
import { sortObjectsByOrder } from "@/utils/sortObjectsByOrder";
import { useBatchRemoveTreeItemState } from "@/workbench/adapters/tanstackQuery/treeItemState/useBatchRemoveTreeItemState";
import { useBatchUpdateTreeItemState } from "@/workbench/adapters/tanstackQuery/treeItemState/useBatchUpdateTreeItemState";
import { useRemoveTreeItemState } from "@/workbench/adapters/tanstackQuery/treeItemState/useRemoveTreeItemState";
import { StreamResourcesEvent } from "@repo/moss-project";
import { useQueryClient } from "@tanstack/react-query";

import { ProjectTreeNode, ProjectTreeRootNode } from "../types";
import { getAllNestedResources, siblingsAfterRemovalPayload } from "../utils";

export const useDeleteAndUpdatePeers = (
  projectId: string,
  node: ProjectTreeNode,
  parentNode: ProjectTreeNode | ProjectTreeRootNode
) => {
  const queryClient = useQueryClient();

  const { currentWorkspaceId } = useCurrentWorkspace();

  const { mutateAsync: deleteProjectResource } = useDeleteProjectResource();
  const { mutateAsync: batchUpdateProjectResource } = useBatchUpdateProjectResource();

  const { mutateAsync: removeTreeItemState } = useRemoveTreeItemState();
  const { mutateAsync: batchUpdateTreeItemState } = useBatchUpdateTreeItemState();
  const { mutateAsync: batchRemoveTreeItemState } = useBatchRemoveTreeItemState();

  const deleteAndUpdatePeers = async () => {
    await deleteProjectResource({
      projectId,
      input: {
        id: node.id,
      },
    });

    const allNestedChildren = getAllNestedResources(node);

    await batchRemoveTreeItemState({
      ids: allNestedChildren.map((child) => child.id),
      workspaceId: currentWorkspaceId,
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
      //TODO: Remove this once we have a proper way to update the project resources(in the tanstack adapter)
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

      await batchUpdateTreeItemState({
        treeItemStates: updatedParentNodeChildren.map((child) => ({
          id: child.id,
          order: child.order! - 1,
          expanded: child.expanded,
        })),
        workspaceId: currentWorkspaceId,
      });

      await removeTreeItemState({
        id: node.id,
        workspaceId: currentWorkspaceId,
      });
    }
  };

  return {
    deleteAndUpdatePeers,
  };
};

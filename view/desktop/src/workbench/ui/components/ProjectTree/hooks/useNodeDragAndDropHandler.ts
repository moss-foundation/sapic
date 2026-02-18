import { useCallback, useEffect } from "react";

import {
  useBatchCreateProjectResource,
  useBatchUpdateProjectResource,
  useCreateProjectResource,
  useDeleteProjectResource,
  useFetchResourcesForPath,
  useUpdateProjectResource,
} from "@/adapters";
import { resourceDetailsCollection } from "@/db/resourceDetails/resourceDetailsCollection";
import { resourceSummariesCollection } from "@/db/resourceSummaries/resourceSummariesCollection";
import { useCurrentWorkspace } from "@/hooks";
import { treeItemStateService } from "@/workbench/services/treeItemStateService";
import { Operation } from "@atlaskit/pragmatic-drag-and-drop-hitbox/dist/types/list-item";
import { monitorForElements } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";
import { join } from "@tauri-apps/api/path";

import { DragNode, DropNode, DropRootNode } from "../types";
import {
  canDropNode,
  createResourceKind,
  getAllNestedResources,
  getInstructionFromLocation,
  getLocationProjectTreeNodeData,
  getLocationProjectTreeRootNodeData,
  getSourceProjectTreeNodeData,
  hasDirectDescendantWithSimilarName,
  isSourceProjectTreeNode,
  makeDirUpdatePayload,
  makeItemUpdatePayload,
  prepareNestedDirResourcesForDrop,
  prepareResourcesForCreation,
  reorderedNodesForDifferentDirPayload,
  reorderedNodesForSameDirPayload,
  resolveParentPath,
  siblingsAfterRemovalPayload,
} from "../utils";

export const useNodeDragAndDropHandler = () => {
  const { currentWorkspaceId } = useCurrentWorkspace();

  const { mutateAsync: createProjectResource } = useCreateProjectResource();
  const { mutateAsync: updateProjectResource } = useUpdateProjectResource();
  const { mutateAsync: deleteProjectResource } = useDeleteProjectResource();

  const { mutateAsync: batchCreateProjectResource } = useBatchCreateProjectResource();
  const { mutateAsync: batchUpdateProjectResource } = useBatchUpdateProjectResource();

  const { fetchResourcesForPath } = useFetchResourcesForPath();

  //Within Project
  const handleNodeOnFolderWithinProject = useCallback(
    async (sourceTreeNodeData: DragNode, locationTreeNodeData: DropNode) => {
      const newOrder = locationTreeNodeData.node.childNodes.length + 1;

      const sourceNodeUpdate =
        sourceTreeNodeData.node.kind === "Dir"
          ? makeDirUpdatePayload({
              id: sourceTreeNodeData.node.id,
              path: locationTreeNodeData.node.path.raw,
              order: newOrder,
            })
          : makeItemUpdatePayload({
              id: sourceTreeNodeData.node.id,
              path: locationTreeNodeData.node.path.raw,
              order: newOrder,
            });

      const sourceParentNodes = sourceTreeNodeData.parentNode.childNodes;
      const nodesToUpdate = siblingsAfterRemovalPayload({
        nodes: sourceParentNodes,
        removedNode: sourceTreeNodeData.node,
      });

      const allUpdates = [sourceNodeUpdate, ...nodesToUpdate];
      await batchUpdateProjectResource({
        projectId: sourceTreeNodeData.projectId,
        resources: {
          resources: allUpdates,
        },
      });

      await treeItemStateService.putOrder(sourceTreeNodeData.node.id, newOrder, currentWorkspaceId);
      await treeItemStateService.putExpanded(
        sourceTreeNodeData.node.id,
        sourceTreeNodeData.node.expanded,
        currentWorkspaceId
      );

      await fetchResourcesForPath(locationTreeNodeData.projectId, resolveParentPath(locationTreeNodeData.parentNode));

      await fetchResourcesForPath(sourceTreeNodeData.projectId, resolveParentPath(sourceTreeNodeData.parentNode));

      return;
    },
    [batchUpdateProjectResource, currentWorkspaceId, fetchResourcesForPath]
  );

  const handleNodeOnNodeWithinProject = useCallback(
    async (sourceTreeNodeData: DragNode, locationTreeNodeData: DropNode, operation: Operation) => {
      const dropIndex =
        operation === "reorder-before"
          ? locationTreeNodeData.node.order! - 0.5
          : locationTreeNodeData.node.order! + 0.5;

      const inSameDir = sourceTreeNodeData.parentNode.id === locationTreeNodeData.parentNode.id;
      if (inSameDir) {
        const updatedSourceNodesPayload = reorderedNodesForSameDirPayload({
          nodes: sourceTreeNodeData.parentNode.childNodes,
          movedId: sourceTreeNodeData.node.id,
          moveToIndex: dropIndex,
        });

        await batchUpdateProjectResource({
          projectId: sourceTreeNodeData.projectId,
          resources: {
            resources: updatedSourceNodesPayload,
          },
        });

        const orderItems: Record<string, number> = {};
        const expandedItems: Record<string, boolean> = {};

        for (const resource of updatedSourceNodesPayload) {
          if ("ITEM" in resource) {
            expandedItems[resource.ITEM.id] = sourceTreeNodeData.node.expanded;
            if ("order" in resource.ITEM) {
              orderItems[resource.ITEM.id] = resource.ITEM.order as number;
            }
          } else if ("DIR" in resource) {
            orderItems[resource.DIR.id] = resource.DIR.order!;
            expandedItems[resource.DIR.id] = sourceTreeNodeData.node.expanded;
          }
        }

        await Promise.all([
          Object.keys(orderItems).length > 0
            ? treeItemStateService.batchPutOrder(orderItems, currentWorkspaceId)
            : Promise.resolve(),
          Object.keys(expandedItems).length > 0
            ? treeItemStateService.batchPutExpanded(expandedItems, currentWorkspaceId)
            : Promise.resolve(),
        ]);

        await fetchResourcesForPath(sourceTreeNodeData.projectId, resolveParentPath(sourceTreeNodeData.parentNode));

        return;
      }

      const targetResourcesToUpdate = reorderedNodesForDifferentDirPayload({
        node: locationTreeNodeData.parentNode,
        newNode: sourceTreeNodeData.node,
        moveToIndex: dropIndex,
      });

      const sourceResourcesToUpdate = siblingsAfterRemovalPayload({
        nodes: sourceTreeNodeData.parentNode.childNodes,
        removedNode: sourceTreeNodeData.node,
      });

      const allResourcesToUpdate = [...targetResourcesToUpdate, ...sourceResourcesToUpdate];

      await batchUpdateProjectResource({
        projectId: sourceTreeNodeData.projectId,
        resources: {
          resources: allResourcesToUpdate,
        },
      });

      const orderItems: Record<string, number> = {};
      const expandedItems: Record<string, boolean> = {};

      for (const resource of allResourcesToUpdate) {
        if ("ITEM" in resource) {
          expandedItems[resource.ITEM.id] = sourceTreeNodeData.node.expanded;
          if ("order" in resource.ITEM) {
            orderItems[resource.ITEM.id] = resource.ITEM.order as number;
          }
        } else if ("DIR" in resource) {
          orderItems[resource.DIR.id] = resource.DIR.order!;
          expandedItems[resource.DIR.id] = sourceTreeNodeData.node.expanded;
        }
      }

      await Promise.all([
        Object.keys(orderItems).length > 0
          ? treeItemStateService.batchPutOrder(orderItems, currentWorkspaceId)
          : Promise.resolve(),
        Object.keys(expandedItems).length > 0
          ? treeItemStateService.batchPutExpanded(expandedItems, currentWorkspaceId)
          : Promise.resolve(),
      ]);

      await fetchResourcesForPath(locationTreeNodeData.projectId, resolveParentPath(locationTreeNodeData.parentNode));
      await fetchResourcesForPath(sourceTreeNodeData.projectId, resolveParentPath(sourceTreeNodeData.parentNode));

      return;
    },
    [batchUpdateProjectResource, currentWorkspaceId, fetchResourcesForPath]
  );

  //To Another Project
  const handleNodeOnNodeToAnotherProject = useCallback(
    async (sourceTreeNodeData: DragNode, locationTreeNodeData: DropNode, operation: Operation) => {
      const dropIndex =
        operation === "reorder-before"
          ? locationTreeNodeData.node.order! - 0.5
          : locationTreeNodeData.node.order! + 0.5;

      const targetResourcesToUpdate = reorderedNodesForDifferentDirPayload({
        node: locationTreeNodeData.parentNode,
        newNode: sourceTreeNodeData.node,
        moveToIndex: dropIndex,
      }).filter((resource) => {
        if ("ITEM" in resource) {
          return resource.ITEM.id !== sourceTreeNodeData.node.id;
        } else {
          return resource.DIR.id !== sourceTreeNodeData.node.id;
        }
      });

      const updatedSourceResourcesPayload = siblingsAfterRemovalPayload({
        nodes: sourceTreeNodeData.parentNode.childNodes,
        removedNode: sourceTreeNodeData.node,
      });

      //Update items in source project
      await batchUpdateProjectResource({
        projectId: sourceTreeNodeData.projectId,
        resources: {
          resources: updatedSourceResourcesPayload,
        },
      });

      for (const resource of updatedSourceResourcesPayload) {
        if ("ITEM" in resource) {
          await treeItemStateService.putExpanded(
            resource.ITEM.id,
            sourceTreeNodeData.node.expanded,
            currentWorkspaceId
          );
        } else if ("DIR" in resource) {
          await treeItemStateService.putOrder(resource.DIR.id, resource.DIR.order!, currentWorkspaceId);
          await treeItemStateService.putExpanded(resource.DIR.id, sourceTreeNodeData.node.expanded, currentWorkspaceId);
        }
      }

      //Update items in target project
      await batchUpdateProjectResource({
        projectId: locationTreeNodeData.projectId,
        resources: {
          resources: targetResourcesToUpdate,
        },
      });

      for (const resource of targetResourcesToUpdate) {
        if ("ITEM" in resource) {
          await treeItemStateService.putExpanded(
            resource.ITEM.id,
            sourceTreeNodeData.node.expanded,
            currentWorkspaceId
          );
        } else if ("DIR" in resource) {
          await treeItemStateService.putOrder(resource.DIR.id, resource.DIR.order!, currentWorkspaceId);
          await treeItemStateService.putExpanded(resource.DIR.id, sourceTreeNodeData.node.expanded, currentWorkspaceId);
        }
      }
      //Delete item in source project
      await deleteProjectResource({
        projectId: sourceTreeNodeData.projectId,
        input: { id: sourceTreeNodeData.node.id },
      });

      await treeItemStateService.removeOrder(sourceTreeNodeData.node.id, currentWorkspaceId);
      await treeItemStateService.removeExpanded(sourceTreeNodeData.node.id, currentWorkspaceId);

      const newDropOrder =
        operation === "reorder-before" ? locationTreeNodeData.node.order! : locationTreeNodeData.node.order! + 1;
      const allResources = getAllNestedResources(sourceTreeNodeData.node);
      const nestedResourcesWithoutName = await prepareNestedDirResourcesForDrop(allResources);
      const batchCreateResourceInput = await Promise.all(
        nestedResourcesWithoutName.map(async (resource, index) => {
          if (index === 0) {
            return createResourceKind({
              name: resource.name,
              path: "path" in locationTreeNodeData.parentNode ? locationTreeNodeData.parentNode.path.raw : "",
              isAddingFolder: resource.kind === "Dir",
              order: newDropOrder,
              protocol: resource.protocol,
              class: "endpoint",
            });
          } else {
            const newResourcePath = await join(
              "path" in locationTreeNodeData.parentNode ? locationTreeNodeData.parentNode.path.raw : "",
              resource.path.raw
            );
            return createResourceKind({
              name: resource.name,
              path: newResourcePath,
              isAddingFolder: resource.kind === "Dir",
              order: -1,
              protocol: resource.protocol,
              class: "endpoint",
            });
          }
        })
      );

      const batchCreateResourceOutput = await batchCreateProjectResource({
        projectId: locationTreeNodeData.projectId,
        input: {
          resources: batchCreateResourceInput,
        },
      });

      for (const resource of batchCreateResourceOutput.resources) {
        const resourceInput = batchCreateResourceInput.find((input) => {
          if ("ITEM" in input) {
            return input.ITEM.path === resource.path.raw && input.ITEM.name === resource.name;
          } else {
            return input.DIR.path === resource.path.raw && input.DIR.name === resource.name;
          }
        });

        if (resourceInput) {
          if ("ITEM" in resourceInput) {
            await treeItemStateService.putOrder(resource.id, resourceInput.ITEM.order, currentWorkspaceId);
            await treeItemStateService.putExpanded(resource.id, sourceTreeNodeData.node.expanded, currentWorkspaceId);
          } else {
            await treeItemStateService.putOrder(resource.id, resourceInput.DIR.order, currentWorkspaceId);
            await treeItemStateService.putExpanded(resource.id, sourceTreeNodeData.node.expanded, currentWorkspaceId);
          }
        }
      }

      await fetchResourcesForPath(
        locationTreeNodeData.projectId,
        "path" in locationTreeNodeData.parentNode ? locationTreeNodeData.parentNode.path.raw : ""
      );
      await fetchResourcesForPath(
        sourceTreeNodeData.projectId,
        "path" in sourceTreeNodeData.parentNode ? sourceTreeNodeData.parentNode.path.raw : ""
      );
    },
    [
      batchUpdateProjectResource,
      deleteProjectResource,
      currentWorkspaceId,
      batchCreateProjectResource,
      fetchResourcesForPath,
    ]
  );

  const handleNodeOnFolderToAnotherProject = useCallback(
    async (sourceTreeNodeData: DragNode, locationTreeNodeData: DropNode) => {
      const allResources = getAllNestedResources(sourceTreeNodeData.node);
      const resourcesPreparedForCreation = await prepareResourcesForCreation(allResources);

      const newOrder = locationTreeNodeData.node.childNodes.length + 1;

      await deleteProjectResource({
        projectId: sourceTreeNodeData.projectId,
        input: { id: sourceTreeNodeData.node.id },
      });

      await treeItemStateService.removeOrder(sourceTreeNodeData.node.id, currentWorkspaceId);
      await treeItemStateService.removeExpanded(sourceTreeNodeData.node.id, currentWorkspaceId);

      const updatedSourceResourcesPayload = siblingsAfterRemovalPayload({
        nodes: sourceTreeNodeData.parentNode.childNodes,
        removedNode: sourceTreeNodeData.node,
      });

      await batchUpdateProjectResource({
        projectId: sourceTreeNodeData.projectId,
        resources: {
          resources: updatedSourceResourcesPayload,
        },
      });

      for (const resource of updatedSourceResourcesPayload) {
        if ("ITEM" in resource) {
          await treeItemStateService.putExpanded(
            resource.ITEM.id,
            sourceTreeNodeData.node.expanded,
            currentWorkspaceId
          );
        } else if ("DIR" in resource) {
          await treeItemStateService.putOrder(resource.DIR.id, resource.DIR.order!, currentWorkspaceId);
          await treeItemStateService.putExpanded(resource.DIR.id, sourceTreeNodeData.node.expanded, currentWorkspaceId);
        }
      }

      const batchCreateResourceInput = await Promise.all(
        resourcesPreparedForCreation.map(async (resource, index) => {
          if (index === 0) {
            return createResourceKind({
              name: resource.name,
              path: locationTreeNodeData.node.path.raw,
              isAddingFolder: resource.kind === "Dir",
              order: newOrder,
              protocol: resource.protocol,
              class: "endpoint",
            });
          } else {
            const newResourcePath = await join(locationTreeNodeData.node.path.raw, resource.path.raw);
            return createResourceKind({
              name: resource.name,
              path: newResourcePath,
              isAddingFolder: resource.kind === "Dir",
              order: -1,
              protocol: resource.protocol,
              class: "endpoint",
            });
          }
        })
      );

      await batchCreateProjectResource({
        projectId: locationTreeNodeData.projectId,
        input: { resources: batchCreateResourceInput },
      });
      //TODO Create orders for created resources
      await fetchResourcesForPath(locationTreeNodeData.projectId, resolveParentPath(locationTreeNodeData.parentNode));
      await fetchResourcesForPath(sourceTreeNodeData.projectId, resolveParentPath(sourceTreeNodeData.parentNode));

      return;
    },
    [
      batchCreateProjectResource,
      batchUpdateProjectResource,
      currentWorkspaceId,
      deleteProjectResource,
      fetchResourcesForPath,
    ]
  );

  //To Another Project's Root
  const handleNodeOnAnotherProjectRoot = useCallback(
    async (sourceTreeNodeData: DragNode, locationTreeRootNodeData: DropRootNode) => {
      const allResources = getAllNestedResources(sourceTreeNodeData.node);
      const resourcesWithoutName = await prepareNestedDirResourcesForDrop(allResources);

      const newOrder = locationTreeRootNodeData.node.childNodes.length + 1;

      await deleteProjectResource({
        projectId: sourceTreeNodeData.projectId,
        input: { id: sourceTreeNodeData.node.id },
      });

      for (const resource of allResources) {
        if (resourceSummariesCollection.has(resource.id)) {
          resourceSummariesCollection.delete(resource.id);
        }
        if (resourceDetailsCollection.has(resource.id)) {
          resourceDetailsCollection.delete(resource.id);
        }
      }

      const updatedSourceResourcesPayload = siblingsAfterRemovalPayload({
        nodes: sourceTreeNodeData.parentNode.childNodes,
        removedNode: sourceTreeNodeData.node,
      });

      const batchCreateResourceInput = await Promise.all(
        resourcesWithoutName.map(async (resource, index) => {
          const newResourcePath = await join(resource.path.raw);

          if (index === 0) {
            return createResourceKind({
              name: resource.name,
              path: "",
              class: "endpoint",
              isAddingFolder: resource.kind === "Dir",
              order: newOrder,
              protocol: resource.protocol,
            });
          } else {
            return createResourceKind({
              name: resource.name,
              path: newResourcePath,
              class: "endpoint",
              isAddingFolder: resource.kind === "Dir",
              order: -1,
              protocol: resource.protocol,
            });
          }
        })
      );

      await batchUpdateProjectResource({
        projectId: sourceTreeNodeData.projectId,
        resources: {
          resources: updatedSourceResourcesPayload,
        },
      });

      const batchCreateResourceOutput = await batchCreateProjectResource({
        projectId: locationTreeRootNodeData.projectId,
        input: {
          resources: batchCreateResourceInput,
        },
      });

      const orderItems: Record<string, number> = {};
      const expandedItems: Record<string, boolean> = {};

      for (const resource of batchCreateResourceOutput.resources) {
        const resourceInput = batchCreateResourceInput.find((input) => {
          if ("ITEM" in input) {
            return input.ITEM.path === resource.path.raw && input.ITEM.name === resource.name;
          } else {
            return input.DIR.path === resource.path.raw && input.DIR.name === resource.name;
          }
        });

        orderItems[resource.id] = resourceInput
          ? "ITEM" in resourceInput
            ? resourceInput.ITEM.order
            : resourceInput.DIR.order
          : -1;
        expandedItems[resource.id] = sourceTreeNodeData.node.expanded;
      }

      await treeItemStateService.batchPutOrder(orderItems, currentWorkspaceId);
      await treeItemStateService.batchPutExpanded(expandedItems, currentWorkspaceId);

      await fetchResourcesForPath(locationTreeRootNodeData.projectId, "");
      await fetchResourcesForPath(sourceTreeNodeData.projectId, resolveParentPath(sourceTreeNodeData.parentNode));
    },
    [
      deleteProjectResource,
      batchUpdateProjectResource,
      batchCreateProjectResource,
      fetchResourcesForPath,
      currentWorkspaceId,
    ]
  );

  useEffect(() => {
    return monitorForElements({
      canMonitor({ source }) {
        return isSourceProjectTreeNode(source);
      },
      onDrop: async ({ location, source }) => {
        const sourceTreeNodeData = getSourceProjectTreeNodeData(source);
        const locationTreeNodeData = getLocationProjectTreeNodeData(location);
        const locationTreeRootNodeData = getLocationProjectTreeRootNodeData(location);

        const instruction = getInstructionFromLocation(location);
        const operation = instruction?.operation;

        if (!sourceTreeNodeData) {
          console.warn("can't drop: no source");
          return;
        }

        if (instruction?.blocked || !operation) {
          console.warn("can't drop: blocked or no operation", { instruction, operation });
          return;
        }

        if (locationTreeRootNodeData && operation === "combine") {
          if (hasDirectDescendantWithSimilarName(locationTreeRootNodeData.node, sourceTreeNodeData.node)) {
            console.warn("can't drop: has direct similar descendant");
            return;
          }

          await handleNodeOnAnotherProjectRoot(sourceTreeNodeData, locationTreeRootNodeData);
          return;
        }

        if (!locationTreeNodeData) {
          console.warn("can't drop: no location");
          return;
        }

        if (!canDropNode(sourceTreeNodeData, locationTreeNodeData, operation)) {
          console.warn("can't drop: invalid operation");
          return;
        }

        const isSameProject = sourceTreeNodeData.projectId === locationTreeNodeData.projectId;
        if (isSameProject) {
          if (operation === "combine") {
            await handleNodeOnFolderWithinProject(sourceTreeNodeData, locationTreeNodeData);
          } else {
            await handleNodeOnNodeWithinProject(sourceTreeNodeData, locationTreeNodeData, operation);
          }
        } else {
          if (operation === "combine") {
            await handleNodeOnFolderToAnotherProject(sourceTreeNodeData, locationTreeNodeData);
          } else {
            await handleNodeOnNodeToAnotherProject(sourceTreeNodeData, locationTreeNodeData, operation);
          }
        }
      },
    });
  }, [
    batchCreateProjectResource,
    batchUpdateProjectResource,
    createProjectResource,
    deleteProjectResource,
    fetchResourcesForPath,
    handleNodeOnFolderWithinProject,
    handleNodeOnNodeToAnotherProject,
    handleNodeOnNodeWithinProject,
    handleNodeOnAnotherProjectRoot,
    updateProjectResource,
    handleNodeOnFolderToAnotherProject,
  ]);
};

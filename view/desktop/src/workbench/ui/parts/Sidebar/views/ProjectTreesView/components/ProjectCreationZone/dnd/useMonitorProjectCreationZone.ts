import { useEffect } from "react";

import { useGetAllLocalProjectSummaries } from "@/db/projectSummaries/hooks/useGetAllLocalProjectSummaries";
import { resourceSummariesCollection } from "@/db/resourceSummaries/resourceSummariesCollection";
import { projectService } from "@/domains/project/projectService";
import { resourceService } from "@/domains/resource/resourceService";
import { useCurrentWorkspace } from "@/hooks";
import { sortObjectsByOrder } from "@/utils/sortObjectsByOrder";
import { treeItemStateService } from "@/workbench/services/treeItemStateService";
import { getInstructionFromFirstLocation } from "@/workbench/ui/components/ProjectTree/ResourcesTree/dnd/getters/getInstructionFromFirstLocation";
import { getSourceProjectTreeNodeData } from "@/workbench/ui/components/ProjectTree/ResourcesTree/dnd/getters/getSourceProjectTreeNodeData";
import { siblingsAfterRemovalPayload } from "@/workbench/ui/components/ProjectTree/ResourcesTree/dnd/handlerOperations/path";
import { isSourceResourceNode } from "@/workbench/ui/components/ProjectTree/ResourcesTree/dnd/validation/isSourceResourceTreeNode";
import { getAllNestedResources } from "@/workbench/ui/components/ProjectTree/ResourcesTree/getters/getAllNestedResources";
import { ResourceNode } from "@/workbench/ui/components/ProjectTree/ResourcesTree/types";
import { monitorForElements } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";
import { BatchUpdateResourceEvent } from "@repo/moss-project";
import { Channel } from "@tauri-apps/api/core";
import { join } from "@tauri-apps/api/path";

import { convertResourceToCreateInput } from "./utils/convertResourceToCreateInput";
import { isLocationProjectCreationZone } from "./validation/isLocationProjectCreationZone";

export const useMonitorProjectCreationZone = () => {
  const { currentWorkspaceId } = useCurrentWorkspace();

  const { data: projectSummaries } = useGetAllLocalProjectSummaries();

  useEffect(() => {
    return monitorForElements({
      canMonitor: ({ source }) => {
        return isSourceResourceNode(source);
      },
      onDrop: async ({ source, location }) => {
        const sourceData = getSourceProjectTreeNodeData(source);
        const instruction = getInstructionFromFirstLocation(location);

        if (!sourceData || instruction?.blocked) {
          console.warn("Invalid source data or instruction for project creation zone", {
            sourceData,
            instruction,
          });
          return;
        }

        if (!isLocationProjectCreationZone(location)) {
          return;
        }

        const resources = await getAllNestedResources({ node: sourceData.node, projectId: sourceData.projectId });

        const rootResource = resources[0];
        const nestedResources = resources.slice(1);

        const newProjectSummary = await projectService.create({
          name: rootResource.name,
        });

        const newProjectOrder = projectSummaries?.length ? projectSummaries.length + 1 : 1;
        await treeItemStateService.putOrder(newProjectSummary.id, newProjectOrder, currentWorkspaceId);
        await treeItemStateService.putExpanded(newProjectSummary.id, true, currentWorkspaceId);

        try {
          await resourceService.delete(sourceData.projectId, {
            id: rootResource.id,
          });

          nestedResources.forEach((resource) => {
            if (resourceSummariesCollection.has(resource.id)) {
              resourceSummariesCollection.delete(resource.id);
            }
          });

          await treeItemStateService.batchRemoveOrder(
            resources.map((r) => r.id),
            currentWorkspaceId
          );

          const updatedSourceSiblings = siblingsAfterRemovalPayload({
            nodes: sourceData.parentNode.childNodes,
            removedNode: sourceData.node,
          });

          if (updatedSourceSiblings.length > 0) {
            const channelEvent = new Channel<BatchUpdateResourceEvent>();
            await resourceService.batchUpdate(
              sourceData.projectId,
              {
                resources: updatedSourceSiblings,
              },
              channelEvent
            );

            const sortedSiblings = sortObjectsByOrder(sourceData.parentNode.childNodes);
            const removedIndex = sortedSiblings.findIndex((e) => e.id === sourceData.node.id);
            const siblingsAfterRemoved = sortedSiblings.slice(removedIndex + 1);

            await treeItemStateService.batchPutOrder(
              Object.fromEntries(siblingsAfterRemoved.map((child) => [child.id, child.order! - 1])),
              currentWorkspaceId
            );
          }
        } catch (error) {
          console.error("Error during project creation:", error);
        }

        try {
          const buildOrderMap = (node: ResourceNode): Map<string, number> => {
            const map = new Map<string, number>();
            const traverse = (n: ResourceNode) => {
              if (n.order !== undefined) map.set(n.id, n.order);
              for (const child of n.childNodes) traverse(child);
            };
            traverse(node);
            return map;
          };
          const orderMap = buildOrderMap(sourceData.node);

          const rootChildIds = new Set(sourceData.node.childNodes.map((c) => c.id));
          const sortedRootChildren = sortObjectsByOrder(sourceData.node.childNodes);
          const rootChildNewOrders = new Map<string, number>(
            sortedRootChildren.map((child, idx) => [child.id, idx + 1])
          );

          const resourcesToCreate = await Promise.all(
            nestedResources.map(async (resource, index) => {
              const rootResourceName = rootResource.name;
              let adjustedSegments = resource.path.segments;

              const rootNameIndex = adjustedSegments.findIndex((segment) => segment === rootResourceName);
              if (rootNameIndex !== -1) {
                adjustedSegments = [
                  ...adjustedSegments.slice(0, rootNameIndex),
                  ...adjustedSegments.slice(rootNameIndex + 1),
                ];
              }

              const parentSegments = adjustedSegments.slice(0, -1);
              const parentPath = parentSegments.length > 0 ? await join(...parentSegments) : "";

              const createInput = convertResourceToCreateInput(resource, parentPath);

              const isRootChild = rootChildIds.has(resource.id);
              const order = isRootChild
                ? rootChildNewOrders.get(resource.id)!
                : (orderMap.get(resource.id) ?? index + 1);

              createInput[resource.kind === "Dir" ? "DIR" : "ITEM"].order = order;

              return createInput;
            })
          );

          const batchCreateOutput = await resourceService.batchCreate(newProjectSummary.id, {
            resources: resourcesToCreate,
          });

          const orderItems: Record<string, number> = {};
          for (const created of batchCreateOutput.resources) {
            const matchingInput = resourcesToCreate.find((input) => {
              if ("ITEM" in input) {
                return input.ITEM.path === created.path.raw && input.ITEM.name === created.name;
              } else {
                return input.DIR.path === created.path.raw && input.DIR.name === created.name;
              }
            });

            if (matchingInput) {
              orderItems[created.id] = "ITEM" in matchingInput ? matchingInput.ITEM.order : matchingInput.DIR.order;
            }
          }

          await treeItemStateService.batchPutOrder(orderItems, currentWorkspaceId);
        } catch (error) {
          console.error("Error during project creation:", error);
        }
      },
    });
  }, [currentWorkspaceId, projectSummaries.length]);
};

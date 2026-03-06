import { useEffect } from "react";

import {
  useBatchCreateProjectResource,
  useBatchUpdateProjectResource,
  useCreateProject,
  useCreateProjectResource,
  useDeleteProjectResource,
  useListProjects,
} from "@/adapters";
import { useCurrentWorkspace } from "@/hooks";
import { sortObjectsByOrder } from "@/utils/sortObjectsByOrder";
import { treeItemStateService } from "@/workbench/services/treeItemStateService";
import {
  convertResourceInfoToCreateInput,
  getAllNestedResources,
  getInstructionFromLocation,
  getSourceProjectTreeNodeData,
  isSourceProjectTreeNode,
  siblingsAfterRemovalPayload,
} from "@/workbench/ui/components/ProjectTree/utils";
import { monitorForElements } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";
import { join } from "@tauri-apps/api/path";

import { isLocationProjectCreationZone } from "./validation/isLocationProjectCreationZone";

export const useMonitorProjectCreationZone = () => {
  const { currentWorkspaceId } = useCurrentWorkspace();
  const { data: projects } = useListProjects();
  const { mutateAsync: createProject } = useCreateProject();
  const { mutateAsync: createProjectResource } = useCreateProjectResource();
  const { mutateAsync: batchCreateProjectResource } = useBatchCreateProjectResource();
  const { mutateAsync: batchUpdateProjectResource } = useBatchUpdateProjectResource();
  const { mutateAsync: deleteProjectResource } = useDeleteProjectResource();

  useEffect(() => {
    return monitorForElements({
      canMonitor: ({ source }) => {
        return isSourceProjectTreeNode(source);
      },
      onDrop: async ({ source, location }) => {
        const sourceData = getSourceProjectTreeNodeData(source);
        const instruction = getInstructionFromLocation(location);

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

        const resources = getAllNestedResources(sourceData.node);

        const rootResource = resources[0];
        const nestedResources = resources.slice(1);

        const newProject = await createProject({
          name: rootResource.name,
        });

        try {
          await deleteProjectResource({
            projectId: sourceData.projectId,
            input: { id: rootResource.id },
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
            await batchUpdateProjectResource({
              projectId: sourceData.projectId,
              resources: { resources: updatedSourceSiblings },
            });

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

              const createInput = convertResourceInfoToCreateInput(resource, parentPath);

              createInput[resource.kind === "Dir" ? "DIR" : "ITEM"].order = index + 1;

              return createInput;
            })
          );

          const batchCreateOutput = await batchCreateProjectResource({
            projectId: newProject.id,
            input: {
              resources: resourcesToCreate,
            },
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
  }, [
    batchCreateProjectResource,
    batchUpdateProjectResource,
    createProject,
    createProjectResource,
    currentWorkspaceId,
    deleteProjectResource,
    projects?.items?.length,
  ]);
};

import { useEffect } from "react";

import {
  useBatchCreateProjectResource,
  useBatchUpdateProjectResource,
  useCreateProjectResource,
  useDeleteProjectResource,
  useFetchResourcesForPath,
  useUpdateProjectResource,
} from "@/adapters";
import { useCurrentWorkspace } from "@/hooks";
import { monitorForElements } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";

import {
  getInstructionFromLocation,
  getLocationProjectTreeNodeData,
  getSourceProjectTreeNodeData,
  isSourceProjectTreeNode,
} from "../../../utils";
import { NodeDropOperation } from "../constants";
import { handleNodeOnFolderToAnotherProject } from "../handlers/handleNodeOnFolderToAnotherProject";
import { handleNodeOnFolderWithinProject } from "../handlers/handleNodeOnFolderWithinProject";
import { handleNodeOnNodeToAnotherProject } from "../handlers/handleNodeOnNodeToAnotherProject";
import { handleNodeOnNodeWithinProject } from "../handlers/handleNodeOnNodeWithinProject";
import { calculateNodeDropOperation } from "../validation/calculateNodeDropOperation";

export const useMonitorResourcesNodes = () => {
  const { currentWorkspaceId } = useCurrentWorkspace();

  const { mutateAsync: createProjectResource } = useCreateProjectResource();
  const { mutateAsync: updateProjectResource } = useUpdateProjectResource();
  const { mutateAsync: deleteProjectResource } = useDeleteProjectResource();
  const { mutateAsync: batchCreateProjectResource } = useBatchCreateProjectResource();
  const { mutateAsync: batchUpdateProjectResource } = useBatchUpdateProjectResource();

  const { fetchResourcesForPath } = useFetchResourcesForPath();

  useEffect(() => {
    return monitorForElements({
      canMonitor({ source }) {
        return isSourceProjectTreeNode(source);
      },
      onDrop: async ({ location, source }) => {
        console.log({
          location,
          source,
        });
        const sourceTreeNodeData = getSourceProjectTreeNodeData(source);
        const locationTreeNodeData = getLocationProjectTreeNodeData(location);
        const instruction = getInstructionFromLocation(location);

        if (!sourceTreeNodeData || !locationTreeNodeData || !instruction) {
          if (!sourceTreeNodeData) console.warn("Invalid source tree node data", { sourceTreeNodeData });
          if (!locationTreeNodeData) console.warn("Invalid location tree node data", { locationTreeNodeData });
          if (!instruction) console.warn("Invalid instruction", { instruction });
          return;
        }

        const nodeDropOperation = calculateNodeDropOperation({
          sourceTreeNodeData,
          locationTreeNodeData,
          instruction,
        });

        switch (nodeDropOperation) {
          case NodeDropOperation.NODE_ON_FOLDER_WITHIN_PROJECT:
            handleNodeOnFolderWithinProject({
              currentWorkspaceId,
              sourceTreeNodeData,
              locationTreeNodeData,
              fetchResourcesForPath,
              batchUpdateProjectResource,
            });
            break;
          case NodeDropOperation.NODE_ON_NODE_WITHIN_PROJECT:
            handleNodeOnNodeWithinProject({
              currentWorkspaceId,
              sourceTreeNodeData,
              locationTreeNodeData,
              operation: instruction.operation,
              fetchResourcesForPath,
              batchUpdateProjectResource,
            });
            break;
          case NodeDropOperation.NODE_ON_FOLDER_TO_ANOTHER_PROJECT:
            handleNodeOnFolderToAnotherProject({
              currentWorkspaceId,
              sourceTreeNodeData,
              locationTreeNodeData,
              batchCreateProjectResource,
              batchUpdateProjectResource,
              deleteProjectResource,
              fetchResourcesForPath,
            });
            break;
          case NodeDropOperation.NODE_ON_NODE_TO_ANOTHER_PROJECT:
            handleNodeOnNodeToAnotherProject({
              currentWorkspaceId,
              sourceTreeNodeData,
              locationTreeNodeData,
              operation: instruction.operation,
              batchUpdateProjectResource,
              deleteProjectResource,
              fetchResourcesForPath,
              batchCreateProjectResource,
            });
            break;
          default:
            break;
        }
      },
    });
  }, [
    batchCreateProjectResource,
    batchUpdateProjectResource,
    createProjectResource,
    deleteProjectResource,
    fetchResourcesForPath,
    updateProjectResource,
    currentWorkspaceId,
  ]);
};

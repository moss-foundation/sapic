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

export const useMonitorTreeNodes = () => {
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
        const sourceTreeNodeData = getSourceProjectTreeNodeData(source);
        const locationTreeNodeData = getLocationProjectTreeNodeData(location);
        const instruction = getInstructionFromLocation(location);

        const nodeDropOperation = calculateNodeDropOperation({
          sourceTreeNodeData,
          locationTreeNodeData,
          instruction,
        });

        switch (nodeDropOperation) {
          case NodeDropOperation.NODE_ON_FOLDER_WITHIN_PROJECT:
            handleNodeOnFolderWithinProject({
              currentWorkspaceId,
              fetchResourcesForPath,
              batchUpdateProjectResource,
            });
            break;
          case NodeDropOperation.NODE_ON_NODE_WITHIN_PROJECT:
            handleNodeOnNodeWithinProject({
              currentWorkspaceId,
              fetchResourcesForPath,
              batchUpdateProjectResource,
            });
            break;
          case NodeDropOperation.NODE_ON_FOLDER_TO_ANOTHER_PROJECT:
            handleNodeOnFolderToAnotherProject({
              batchCreateProjectResource,
              batchUpdateProjectResource,
              currentWorkspaceId,
              deleteProjectResource,
              fetchResourcesForPath,
            });
            break;
          case NodeDropOperation.NODE_ON_NODE_TO_ANOTHER_PROJECT:
            handleNodeOnNodeToAnotherProject({
              batchUpdateProjectResource,
              currentWorkspaceId,
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

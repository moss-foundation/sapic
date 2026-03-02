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
  canDropNode,
  getInstructionFromLocation,
  getLocationProjectTreeNodeData,
  getLocationProjectTreeRootNodeData,
  getSourceProjectTreeNodeData,
  hasDirectDescendantWithSimilarName,
  isSourceProjectTreeNode,
} from "../../../utils";
import { handleNodeOnAnotherProjectRoot } from "../handlers/handleNodeOnAnotherProjectRoot";
import { handleNodeOnFolderToAnotherProject } from "../handlers/handleNodeOnFolderToAnotherProject";
import { handleNodeOnFolderWithinProject } from "../handlers/handleNodeOnFolderWithinProject";
import { handleNodeOnNodeToAnotherProject } from "../handlers/handleNodeOnNodeToAnotherProject";
import { handleNodeOnNodeWithinProject } from "../handlers/handleNodeOnNodeWithinProject";

export const useNodeDragAndDropHandler = () => {
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

          handleNodeOnAnotherProjectRoot({
            deleteProjectResource,
            batchUpdateProjectResource,
            batchCreateProjectResource,
            currentWorkspaceId,
            fetchResourcesForPath,
          });
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
            handleNodeOnFolderWithinProject({
              currentWorkspaceId,
              fetchResourcesForPath,
              batchUpdateProjectResource,
            });
          } else {
            handleNodeOnNodeWithinProject({
              currentWorkspaceId,
              fetchResourcesForPath,
              batchUpdateProjectResource,
            });
          }
        } else {
          if (operation === "combine") {
            handleNodeOnFolderToAnotherProject({
              batchCreateProjectResource,
              batchUpdateProjectResource,
              currentWorkspaceId,
              deleteProjectResource,
              fetchResourcesForPath,
            });
          } else {
            handleNodeOnNodeToAnotherProject({
              batchUpdateProjectResource,
              currentWorkspaceId,
              deleteProjectResource,
              fetchResourcesForPath,
              batchCreateProjectResource,
            });
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
    updateProjectResource,
    currentWorkspaceId,
  ]);
};

import { useEffect } from "react";

import { useCurrentWorkspace } from "@/hooks";
import { monitorForElements } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";

import { ProjectDragType } from "../../../constants";
import { NodeDropOperation } from "../constants";
import { getFirstDropTargetType } from "../getters/getFirstDropTargetType";
import { getInstructionFromFirstLocation } from "../getters/getInstructionFromFirstLocation";
import { getLocationProjectTreeNodeData } from "../getters/getLocationProjectTreeNodeData";
import { getLocationResourcesListData } from "../getters/getLocationResourcesListData";
import { getSourceProjectTreeNodeData } from "../getters/getSourceProjectTreeNodeData";
import { handleNodeOnFolderToAnotherProject } from "../handlers/handleNodeOnFolderToAnotherProject";
import { handleNodeOnFolderWithinProject } from "../handlers/handleNodeOnFolderWithinProject";
import { handleNodeOnListRootToAnotherProject } from "../handlers/handleNodeOnListRootToAnotherProject";
import { handleNodeOnListRootWithinProject } from "../handlers/handleNodeOnListRootWithinProject";
import { handleNodeOnNodeToAnotherProject } from "../handlers/handleNodeOnNodeToAnotherProject";
import { handleNodeOnNodeWithinProject } from "../handlers/handleNodeOnNodeWithinProject";
import { calculateNodeDropOperation } from "../validation/calculateNodeDropOperation";
import { isSourceResourceNode } from "../validation/isSourceResourceTreeNode";

export const useMonitorResourcesNodes = () => {
  const { currentWorkspaceId } = useCurrentWorkspace();

  useEffect(() => {
    return monitorForElements({
      canMonitor({ source }) {
        return isSourceResourceNode(source);
      },
      onDrop: async ({ location, source }) => {
        const sourceTreeNodeData = getSourceProjectTreeNodeData(source);
        if (!sourceTreeNodeData) return;

        const dropTargetType = getFirstDropTargetType(location);

        if (dropTargetType === ProjectDragType.RESOURCES_LIST) {
          const listRootData = getLocationResourcesListData(location);
          const instruction = getInstructionFromFirstLocation(location);
          if (!listRootData || !instruction) return;

          const isSameProject = sourceTreeNodeData.projectId === listRootData.data.projectId;

          if (isSameProject) {
            handleNodeOnListRootWithinProject({
              currentWorkspaceId,
              sourceTreeNodeData,
              locationResourcesListData: listRootData,
            });
          } else {
            handleNodeOnListRootToAnotherProject({
              currentWorkspaceId,
              sourceTreeNodeData,
              locationResourcesListData: listRootData,
            });
          }
          return;
        }

        const locationTreeNodeData = getLocationProjectTreeNodeData(location);
        const instruction = getInstructionFromFirstLocation(location);

        if (!locationTreeNodeData || !instruction) return;

        const nodeDropOperation = calculateNodeDropOperation({
          sourceTreeNodeData,
          locationTreeNodeData,
          instruction,
        });

        switch (nodeDropOperation) {
          case NodeDropOperation.NODE_ON_NODE_WITHIN_PROJECT:
            handleNodeOnNodeWithinProject({
              currentWorkspaceId,
              sourceTreeNodeData,
              locationTreeNodeData,
              operation: instruction.operation,
            });
            break;

          case NodeDropOperation.NODE_ON_FOLDER_WITHIN_PROJECT:
            handleNodeOnFolderWithinProject({
              currentWorkspaceId,
              sourceTreeNodeData,
              locationTreeNodeData,
            });
            break;

          case NodeDropOperation.NODE_ON_NODE_TO_ANOTHER_PROJECT:
            handleNodeOnNodeToAnotherProject({
              currentWorkspaceId,
              sourceTreeNodeData,
              locationTreeNodeData,
              operation: instruction.operation,
            });
            break;

          case NodeDropOperation.NODE_ON_FOLDER_TO_ANOTHER_PROJECT:
            handleNodeOnFolderToAnotherProject({
              currentWorkspaceId,
              sourceTreeNodeData,
              locationTreeNodeData,
            });
            break;

          default:
            break;
        }
      },
    });
  }, [currentWorkspaceId]);
};

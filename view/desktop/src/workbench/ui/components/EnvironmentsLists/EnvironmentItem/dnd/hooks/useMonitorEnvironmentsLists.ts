import { useEffect } from "react";

import { useCreateEnvironment, useDeleteEnvironment } from "@/adapters";
import { useGetAllProjectEnvironments } from "@/db/environmentsSummaries/hooks/useGetAllProjectEnvironments";
import { useGetWorkspaceEnvironments } from "@/db/environmentsSummaries/hooks/useGetWorkspaceEnvironments";
import { useCurrentWorkspace } from "@/hooks";
import { useBatchPutEnvironmentItemState } from "@/workbench/adapters/tanstackQuery/environmentItemState/useBatchPutEnvironmentItemState";
import { useRemoveEnvironmentItemState } from "@/workbench/adapters/tanstackQuery/environmentItemState/useRemoveEnvironmentItemState";
import { extractInstruction } from "@atlaskit/pragmatic-drag-and-drop-hitbox/list-item";
import { monitorForElements } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";

import { EnvironmentsDropOperations } from "../../../constants";
import { getLocationEnvironmentItemData, getSourceEnvironmentItemData } from "../getters";
import { handleCombineWorkspaceEnvToProjectList } from "../handlers/handleCombineWorkspaceEnvToProjectList";
import { handleMoveProjectEnvToProjectEnv } from "../handlers/handleMoveProjectEnvToProjectEnv";
import { handleMoveProjectEnvToWorkspaceEnvs } from "../handlers/handleMoveProjectEnvToWorkspaceEnvs";
import { handleMoveWorkspaceEnvToProjectEnvs } from "../handlers/handleMoveWorkspaceEnvToProjectEnvs";
import { handleReorderProjectEnvs } from "../handlers/handleReorderProjectEnvs";
import { handleReorderWorkspaceEnvs } from "../handlers/handleReorderWorkspaceEnvs";
import { isSourceEnvironmentItem } from "../validation";
import { calculateDropType } from "../validation/calculateDropType";

export const useMonitorEnvironmentsLists = () => {
  const { currentWorkspaceId } = useCurrentWorkspace();

  const { workspaceEnvironments } = useGetWorkspaceEnvironments();
  const { projectEnvironments: allProjectEnvironments } = useGetAllProjectEnvironments();

  const { mutateAsync: deleteEnvironment } = useDeleteEnvironment();
  const { mutateAsync: createEnvironment } = useCreateEnvironment();

  const { mutateAsync: batchPutEnvironmentItemState } = useBatchPutEnvironmentItemState();
  const { mutateAsync: removeEnvironmentItemState } = useRemoveEnvironmentItemState();

  useEffect(() => {
    return monitorForElements({
      canMonitor({ source }) {
        return isSourceEnvironmentItem(source);
      },
      onDrop({ source, location }) {
        const sourceData = getSourceEnvironmentItemData(source);
        const locationData = getLocationEnvironmentItemData(location);
        const instruction = extractInstruction(locationData ?? {});

        if (!sourceData || !locationData || !instruction) {
          if (!locationData) console.warn("Invalid location data for environments lists", { locationData });
          if (!instruction) console.warn("Invalid instruction for environments lists", { instruction });
          if (!sourceData) console.warn("Invalid source data for environments lists", { sourceData });
          return;
        }

        const dropType = calculateDropType(sourceData, locationData);
        switch (dropType) {
          case EnvironmentsDropOperations.ReorderWorkspaceEnvs:
            handleReorderWorkspaceEnvs({
              sourceData,
              locationData,
              workspaceEnvironments,
              instruction,
              currentWorkspaceId,
              batchPutEnvironmentItemState,
            });
            break;
          case EnvironmentsDropOperations.ReorderProjectEnvs:
            handleReorderProjectEnvs({
              sourceData,
              locationData,
              projectEnvironments: allProjectEnvironments ?? [],
              instruction,
              currentWorkspaceId,
              batchPutEnvironmentItemState,
            });
            break;
          case EnvironmentsDropOperations.MoveWorkspaceEnvToProjectEnvs:
            handleMoveWorkspaceEnvToProjectEnvs({
              sourceData,
              locationData,
              workspaceEnvironments,
              projectEnvironments: allProjectEnvironments ?? [],
              instruction,
              currentWorkspaceId,
              batchPutEnvironmentItemState,
              removeEnvironmentItemState,
              deleteEnvironment,
              createEnvironment,
            });
            break;
          case EnvironmentsDropOperations.MoveProjectEnvToWorkspaceEnvs:
            handleMoveProjectEnvToWorkspaceEnvs({
              sourceData,
              locationData,
              projectEnvironments: allProjectEnvironments ?? [],
              workspaceEnvironments,
              instruction,
              currentWorkspaceId,
              batchPutEnvironmentItemState,
              removeEnvironmentItemState,
              deleteEnvironment,
              createEnvironment,
            });
            break;
          case EnvironmentsDropOperations.MoveProjectEnvToProjectEnv:
            handleMoveProjectEnvToProjectEnv({
              sourceData,
              locationData,
              projectEnvironments: allProjectEnvironments ?? [],
              instruction,
              currentWorkspaceId,
              batchPutEnvironmentItemState,
              removeEnvironmentItemState,
              deleteEnvironment,
              createEnvironment,
            });
            break;
          case EnvironmentsDropOperations.CombineWorkspaceEnvToProjectList:
            handleCombineWorkspaceEnvToProjectList({
              sourceData,
              locationData,
              workspaceEnvironments,
              projectEnvironments: allProjectEnvironments ?? [],
              currentWorkspaceId,
              batchPutEnvironmentItemState,
              removeEnvironmentItemState,
              deleteEnvironment,
              createEnvironment,
            });
            break;
          case EnvironmentsDropOperations.CombineProjectEnvToProjectList:
            break;
          default:
            break;
        }
      },
    });
  }, [
    batchPutEnvironmentItemState,
    currentWorkspaceId,
    allProjectEnvironments,
    workspaceEnvironments,
    removeEnvironmentItemState,
    deleteEnvironment,
    createEnvironment,
  ]);
};

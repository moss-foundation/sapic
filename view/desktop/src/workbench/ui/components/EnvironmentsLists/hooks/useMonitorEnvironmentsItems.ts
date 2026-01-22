import { useCallback, useEffect } from "react";

import {
  useCreateEnvironment,
  useDeleteEnvironment,
  useStreamEnvironments,
} from "@/adapters/tanstackQuery/environment";
import { useAllStreamedProjectEnvironments } from "@/adapters/tanstackQuery/environment/derived/useAllStreamedProjectEnvironments";
import { useBatchUpdateEnvironment } from "@/adapters/tanstackQuery/environment/useBatchUpdateEnvironment";
import { DragLocationHistory } from "@atlaskit/pragmatic-drag-and-drop/dist/types/internal-types";
import { ElementDragPayload, monitorForElements } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";

import {
  getDropOperation,
  getLocationGlobalEnvironmentItemData,
  getLocationGroupedEnvironmentItemData,
  getLocationGroupedEnvironmentListData,
  getSourceGlobalEnvironmentItemData,
  getSourceGroupedEnvironmentItemData,
  isSourceEnvironmentItem,
  isSourceGlobalEnvironmentItem,
  isSourceGroupedEnvironmentItem,
} from "../utils";

export const useMonitorEnvironmentsItems = () => {
  const { data: workspaceEnvironments } = useStreamEnvironments();
  const { allProjectEnvironments } = useAllStreamedProjectEnvironments();

  const { mutateAsync: batchUpdateEnvironment } = useBatchUpdateEnvironment();
  const { mutateAsync: deleteEnvironment } = useDeleteEnvironment();
  const { mutateAsync: createEnvironment } = useCreateEnvironment();

  const handleReorderGlobals = useCallback(
    async (source: ElementDragPayload, location: DragLocationHistory) => {
      const sourceData = getSourceGlobalEnvironmentItemData(source);
      const locationData = getLocationGlobalEnvironmentItemData(location);

      if (!sourceData || !locationData) return;

      const sourceIndex = workspaceEnvironments?.findIndex((env) => env.id === sourceData.data.environment.id);
      const targetIndex = workspaceEnvironments?.findIndex((env) => env.id === locationData.data.environment.id);
      const instruction = locationData.instruction;

      if (sourceIndex === -1 || targetIndex === -1 || !instruction) {
        console.error("Source, target or instruction not found", { sourceIndex, targetIndex, instruction });
        return;
      }

      const dropOrder = instruction.operation === "reorder-before" ? targetIndex : (targetIndex ?? 0) + 1;

      const inserted = [
        ...(workspaceEnvironments?.slice(0, dropOrder).filter((env) => env.id !== sourceData.data.environment.id) ??
          []),
        sourceData.data.environment,
        ...(workspaceEnvironments?.slice(dropOrder).filter((env) => env.id !== sourceData.data.environment.id) ?? []),
      ];

      const reordered = inserted.map((env, index) => ({
        id: env.id,
        order: index + 1,
      }));

      const environmentsToUpdateReordered = reordered.filter((env) => {
        const environmentUnderQuestion = workspaceEnvironments?.find((sortedEnv) => sortedEnv.id === env.id);
        return environmentUnderQuestion!.order !== env.order;
      });

      await batchUpdateEnvironment({
        items: environmentsToUpdateReordered.map((env) => ({
          id: env.id,
          order: env.order,
          varsToAdd: [],
          varsToUpdate: [],
          varsToDelete: [],
        })),
      });
    },
    [workspaceEnvironments, batchUpdateEnvironment]
  );

  const handleMoveToGrouped = useCallback(
    async (source: ElementDragPayload, location: DragLocationHistory) => {
      if (isSourceGlobalEnvironmentItem(source)) {
        const sourceData = getSourceGlobalEnvironmentItemData(source);
        const locationData = getLocationGroupedEnvironmentItemData(location);

        if (!sourceData || !locationData) return;

        const instruction = locationData.instruction;

        const groupedEnvironment = allProjectEnvironments?.find(
          (group) => group.projectId === locationData.data.environment.projectId
        );

        if (!groupedEnvironment) {
          console.error("Grouped environment not found", { locationData });
          return;
        }

        const targetOrder = groupedEnvironment?.order;

        if (targetOrder === undefined || targetOrder === undefined || !instruction) {
          console.error("Source, target or instruction not found", { targetOrder, instruction });
          return;
        }
        const dropOrder = instruction.operation === "reorder-before" ? targetOrder : targetOrder + 1;

        //delete global environment
        await deleteEnvironment({ id: sourceData.data.environment.id });

        //get reordered global environments after the deleted one
        const workspaceEnvironmentsToUpdate =
          workspaceEnvironments
            ?.filter((env) => env.order! > sourceData.data.environment.order!)
            .map((env) => ({
              id: env.id,
              order: env.order! - 1,
            })) ?? [];

        //add new grouped environment
        const newGroupedEnvironment = await createEnvironment({
          projectId: locationData.data.environment.projectId,
          name: sourceData.data.environment.name,
          order: dropOrder,
          variables: [],
        });

        //get reordered grouped environments after new grouped environment
        const projectEnvironmentsToUpdate =
          allProjectEnvironments
            ?.filter((env) => env.projectId === locationData.data.environment.projectId)
            .filter((env) => env.order! >= dropOrder && env.id !== newGroupedEnvironment.id)
            .map((env) => ({
              id: env.id,
              order: env.order! + 1,
            })) ?? [];

        //update global and grouped environments
        const envsToUpdate = [...workspaceEnvironmentsToUpdate, ...projectEnvironmentsToUpdate];
        await batchUpdateEnvironment({
          items: envsToUpdate.map((env) => ({
            id: env.id,
            order: env.order!,
            varsToAdd: [],
            varsToUpdate: [],
            varsToDelete: [],
          })),
        });
      }
      if (isSourceGroupedEnvironmentItem(source)) {
        const sourceData = getSourceGroupedEnvironmentItemData(source);
        const locationData = getLocationGroupedEnvironmentItemData(location);

        if (!sourceData || !locationData) {
          console.error("Source or location data not found", { sourceData, locationData });
          return;
        }

        const instruction = locationData.instruction;

        if (!instruction) {
          console.error("Instruction not found", { instruction });
          return;
        }

        const sourceGroup = allProjectEnvironments?.find(
          (group) => group.projectId === sourceData.data.environment.projectId
        );
        const locationGroup = allProjectEnvironments?.find(
          (group) => group.projectId === locationData.data.environment.projectId
        );

        if (!sourceGroup || !locationGroup) {
          console.error("Source group not found", { sourceGroup, locationGroup });
          return;
        }

        const targetOrder = locationGroup?.order;

        if (targetOrder === undefined || targetOrder === undefined || !instruction) {
          console.error("Source, target or instruction not found", { targetOrder, instruction });
          return;
        }

        const targetDropOrder = instruction.operation === "reorder-before" ? targetOrder : targetOrder + 1;

        //delete grouped environment
        await deleteEnvironment({ id: sourceData.data.environment.id });

        //get reordered grouped environments after the deleted one
        const projectEnvironmentsToUpdateSource =
          allProjectEnvironments
            ?.filter((env) => env.projectId === sourceData.data.environment.projectId)
            ?.filter((env) => env.order! > sourceData.data.environment.order!)
            .map((env) => ({
              id: env.id,
              order: env.order! - 1,
            })) ?? [];

        //add new grouped environment
        const newGroupedEnvironment = await createEnvironment({
          projectId: locationData.data.environment.projectId,
          name: sourceData.data.environment.name,
          order: targetDropOrder,
          variables: [],
        });

        //get reordered grouped environments after new grouped environment
        const projectEnvironmentsToUpdateLocation =
          allProjectEnvironments
            ?.filter((env) => env.projectId === locationData.data.environment.projectId)
            .filter((env) => env.order! >= targetDropOrder && env.id !== newGroupedEnvironment.id)
            .map((env) => ({
              id: env.id,
              order: env.order! + 1,
            })) ?? [];

        //update grouped environments
        const envsToUpdate = [...projectEnvironmentsToUpdateSource, ...projectEnvironmentsToUpdateLocation];
        await batchUpdateEnvironment({
          items: envsToUpdate.map((env) => ({
            id: env.id,
            order: env.order!,
            varsToAdd: [],
            varsToUpdate: [],
            varsToDelete: [],
          })),
        });
      }
    },
    [allProjectEnvironments, batchUpdateEnvironment, createEnvironment, deleteEnvironment, workspaceEnvironments]
  );

  const handleCombineToGrouped = useCallback(
    async (source: ElementDragPayload, location: DragLocationHistory) => {
      const locationData = getLocationGroupedEnvironmentListData(location);

      if (!locationData) return;

      const locationGroupedEnvironments = locationData.data.groupWithEnvironments.environments;

      if (isSourceGlobalEnvironmentItem(source)) {
        const sourceData = getSourceGlobalEnvironmentItemData(source);

        if (!sourceData) return;

        //delete global environment
        await deleteEnvironment({ id: sourceData.data.environment.id });

        //get reordered global environments after the deleted one
        const workspaceEnvironmentsToUpdate =
          workspaceEnvironments
            ?.filter(
              (globalEnv) =>
                (globalEnv.order ?? 0) > (sourceData.data.environment.order ?? 0) &&
                globalEnv.id !== sourceData.data.environment.id
            )
            .map((globalEnv) => ({
              id: globalEnv.id,
              order: (globalEnv.order ?? 0) - 1,
            })) ?? [];

        //update global and grouped environments
        await batchUpdateEnvironment({
          items: workspaceEnvironmentsToUpdate.map((env) => ({
            id: env.id,
            order: env.order!,
            varsToAdd: [],
            varsToUpdate: [],
            varsToDelete: [],
          })),
        });

        //add new grouped environment
        await createEnvironment({
          projectId: locationData.data.groupWithEnvironments.projectId,
          name: sourceData.data.environment.name,
          order: locationGroupedEnvironments.length + 1,
          variables: [],
        });
      } else {
        const sourceData = getSourceGroupedEnvironmentItemData(source);

        if (!sourceData) return;

        const sourceGroup = allProjectEnvironments?.find(
          (group) => group.projectId === sourceData.data.environment.projectId
        );

        if (!sourceGroup) return;

        //delete grouped environment
        await deleteEnvironment({ id: sourceData.data.environment.id });

        //get reordered grouped environments after the deleted one
        const projectEnvironmentsToUpdate =
          allProjectEnvironments
            ?.filter((env) => env.projectId === sourceData.data.environment.projectId)
            .filter((env) => env.order! > sourceData.data.environment.order!)
            .filter((env) => env.id !== sourceData.data.environment.id)
            .map((env) => ({
              id: env.id,
              order: env.order! - 1,
            })) ?? [];

        await createEnvironment({
          projectId: locationData.data.groupWithEnvironments.projectId,
          name: sourceData.data.environment.name,
          order: locationGroupedEnvironments.length + 1,
          variables: [],
        });

        //update grouped environments
        await batchUpdateEnvironment({
          items: projectEnvironmentsToUpdate.map((env) => ({
            id: env.id,
            order: env.order!,
            varsToAdd: [],
            varsToUpdate: [],
            varsToDelete: [],
          })),
        });
      }
    },
    [batchUpdateEnvironment, createEnvironment, deleteEnvironment, allProjectEnvironments, workspaceEnvironments]
  );

  const handleReorderGrouped = useCallback(
    async (source: ElementDragPayload, location: DragLocationHistory) => {
      const sourceData = getSourceGroupedEnvironmentItemData(source);
      const locationData = getLocationGroupedEnvironmentItemData(location);

      if (!sourceData || !locationData) return;

      if (sourceData.data.environment.projectId !== locationData.data.environment.projectId) return;

      const groupEnvs = allProjectEnvironments?.find(
        (group) => group.projectId === sourceData.data.environment.projectId
      );

      if (!groupEnvs) return;

      const sourceIndex = allProjectEnvironments?.findIndex((env) => env.id === sourceData.data.environment.id);
      const targetIndex = allProjectEnvironments?.findIndex((env) => env.id === locationData.data.environment.id);
      const instruction = locationData.instruction;

      if (sourceIndex === -1 || targetIndex === -1 || !instruction) {
        console.error("Source, target or instruction not found", { sourceIndex, targetIndex, instruction });
        return;
      }

      const dropOrder = instruction.operation === "reorder-before" ? targetIndex : (targetIndex ?? 0) + 1;

      const inserted = [
        ...(allProjectEnvironments?.slice(0, dropOrder).filter((env) => env.id !== sourceData.data.environment.id) ??
          []),
        sourceData.data.environment,
        ...(allProjectEnvironments?.slice(dropOrder).filter((env) => env.id !== sourceData.data.environment.id) ?? []),
      ];

      const reordered = inserted.map((env, index) => ({
        id: env.id,
        name: env.name,
        order: index + 1,
      }));

      const environmentsToUpdateReordered = reordered.filter((env) => {
        const environmentUnderQuestion = allProjectEnvironments?.find((sortedEnv) => sortedEnv.id === env.id);
        return environmentUnderQuestion!.order !== env.order;
      });

      await batchUpdateEnvironment({
        items: environmentsToUpdateReordered.map((env) => ({
          id: env.id,
          order: env.order,
          varsToAdd: [],
          varsToUpdate: [],
          varsToDelete: [],
        })),
      });
    },
    [batchUpdateEnvironment, allProjectEnvironments]
  );

  const handleMoveToGlobal = useCallback(
    async (source: ElementDragPayload, location: DragLocationHistory) => {
      const sourceData = getSourceGroupedEnvironmentItemData(source);
      const locationData = getLocationGlobalEnvironmentItemData(location);

      if (!sourceData || !locationData) {
        console.error("Source or location data not found", { sourceData, locationData });
        return;
      }

      const instruction = locationData.instruction;

      const group = allProjectEnvironments?.find((group) => group.projectId === sourceData.data.environment.projectId);

      if (!group) {
        console.error("Group environment not found", { locationData });
        return;
      }

      const targetOrder = workspaceEnvironments?.find((env) => env.id === locationData.data.environment.id)?.order;

      if (targetOrder === undefined || targetOrder === undefined || !instruction) {
        console.error("Source, target or instruction not found", { targetOrder, instruction });
        return;
      }
      const dropOrder = instruction.operation === "reorder-before" ? targetOrder : targetOrder + 1;

      //delete grouped environment
      await deleteEnvironment({ id: sourceData.data.environment.id });

      //get reordered grouped environments after the deleted one
      const projectEnvsToUpdate =
        allProjectEnvironments
          ?.filter((env) => env.order! > sourceData.data.environment.order!)
          .map((env) => ({
            id: env.id,
            order: env.order! - 1,
          })) ?? [];

      //add new global environment
      const newGlobalEnv = await createEnvironment({
        name: sourceData.data.environment.name,
        order: dropOrder,
        variables: [],
      });

      //get reordered global environments after new global environment
      const workspaceEnvironmentsToUpdate =
        workspaceEnvironments
          ?.filter((env) => env.order! >= dropOrder && env.id !== newGlobalEnv.id)
          .map((env) => ({
            id: env.id,
            order: env.order! + 1,
          })) ?? [];

      //update global and grouped environments
      const envsToUpdate = [...projectEnvsToUpdate, ...workspaceEnvironmentsToUpdate];
      await batchUpdateEnvironment({
        items: envsToUpdate.map((env) => ({
          id: env.id,
          order: env.order!,
          varsToAdd: [],
          varsToUpdate: [],
          varsToDelete: [],
        })),
      });
    },
    [allProjectEnvironments, batchUpdateEnvironment, createEnvironment, deleteEnvironment, workspaceEnvironments]
  );

  useEffect(() => {
    return monitorForElements({
      canMonitor({ source }) {
        return isSourceEnvironmentItem(source);
      },
      onDrop({ source, location }) {
        if (!isSourceEnvironmentItem(source)) {
          console.warn("can't drop: no source");
          return;
        }

        const dropOperation = getDropOperation(source, location);
        switch (dropOperation) {
          case "ReorderGlobals":
            handleReorderGlobals(source, location);
            break;
          case "ReorderGrouped":
            handleReorderGrouped(source, location);
            break;
          case "MoveToGlobal":
            handleMoveToGlobal(source, location);
            break;
          case "MoveToGrouped":
            handleMoveToGrouped(source, location);
            break;
          case "CombineToGrouped":
            handleCombineToGrouped(source, location);
            break;

          default:
            break;
        }
      },
    });
  }, [handleCombineToGrouped, handleReorderGlobals, handleMoveToGrouped, handleReorderGrouped, handleMoveToGlobal]);
};

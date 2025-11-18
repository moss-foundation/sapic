import { useCallback, useEffect } from "react";

import { useCreateEnvironment, useDeleteEnvironment, useStreamEnvironments } from "@/workbench/adapters";
import { useBatchUpdateEnvironment } from "@/workbench/adapters/tanstackQuery/environment/useBatchUpdateEnvironment";
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
import { useGroupedEnvironments } from "./useGroupedEnvironments";

export const useMonitorEnvironmentsItems = () => {
  const { globalEnvironments } = useStreamEnvironments();
  const { groupedEnvironments } = useGroupedEnvironments();
  const { mutateAsync: batchUpdateEnvironment } = useBatchUpdateEnvironment();
  const { mutateAsync: deleteEnvironment } = useDeleteEnvironment();
  const { mutateAsync: createEnvironment } = useCreateEnvironment();

  const handleReorderGlobals = useCallback(
    async (source: ElementDragPayload, location: DragLocationHistory) => {
      const sourceData = getSourceGlobalEnvironmentItemData(source);
      const locationData = getLocationGlobalEnvironmentItemData(location);

      if (!sourceData || !locationData) return;

      const sourceIndex = globalEnvironments.findIndex((env) => env.id === sourceData.data.environment.id);
      const targetIndex = globalEnvironments.findIndex((env) => env.id === locationData.data.environment.id);
      const instruction = locationData.instruction;

      if (sourceIndex === -1 || targetIndex === -1 || !instruction) {
        console.error("Source, target or instruction not found", { sourceIndex, targetIndex, instruction });
        return;
      }

      const dropOrder = instruction.operation === "reorder-before" ? targetIndex : targetIndex + 1;

      const inserted = [
        ...globalEnvironments.slice(0, dropOrder).filter((env) => env.id !== sourceData.data.environment.id),
        sourceData.data.environment,
        ...globalEnvironments.slice(dropOrder).filter((env) => env.id !== sourceData.data.environment.id),
      ];

      const reordered = inserted.map((env, index) => ({
        id: env.id,
        order: index + 1,
      }));

      const environmentsToUpdateReordered = reordered.filter((env) => {
        const environmentUnderQuestion = globalEnvironments.find((sortedEnv) => sortedEnv.id === env.id);
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
    [globalEnvironments, batchUpdateEnvironment]
  );

  const handleMoveToGrouped = useCallback(
    async (source: ElementDragPayload, location: DragLocationHistory) => {
      if (isSourceGlobalEnvironmentItem(source)) {
        const sourceData = getSourceGlobalEnvironmentItemData(source);
        const locationData = getLocationGroupedEnvironmentItemData(location);

        if (!sourceData || !locationData) return;

        const instruction = locationData.instruction;

        const groupedEnvironment = groupedEnvironments.find(
          (group) => group.projectId === locationData.data.environment.projectId
        );

        if (!groupedEnvironment) {
          console.error("Grouped environment not found", { locationData });
          return;
        }

        const targetOrder = groupedEnvironment?.environments.find(
          (env) => env.id === locationData.data.environment.id
        )?.order;

        if (targetOrder === undefined || targetOrder === undefined || !instruction) {
          console.error("Source, target or instruction not found", { targetOrder, instruction });
          return;
        }
        const dropOrder = instruction.operation === "reorder-before" ? targetOrder : targetOrder + 1;

        //delete global environment
        await deleteEnvironment({ id: sourceData.data.environment.id });

        //get reordered global environments after the deleted one
        const globalEnvironmentsToUpdate = globalEnvironments
          .filter((env) => env.order! > sourceData.data.environment.order!)
          .map((env) => ({
            id: env.id,
            order: env.order! - 1,
          }));

        //add new grouped environment
        const newGroupedEnvironment = await createEnvironment({
          projectId: locationData.data.environment.projectId,
          name: sourceData.data.environment.name,
          order: dropOrder,
          variables: [],
        });

        //get reordered grouped environments after new grouped environment
        const groupedEnvironmentsToUpdate =
          groupedEnvironment?.environments
            .filter((env) => env.order! >= dropOrder && env.id !== newGroupedEnvironment.id)
            .map((env) => ({
              id: env.id,
              order: env.order! + 1,
            })) ?? [];

        //update global and grouped environments
        const envsToUpdate = [...globalEnvironmentsToUpdate, ...groupedEnvironmentsToUpdate];
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

        const sourceGroup = groupedEnvironments.find(
          (group) => group.projectId === sourceData.data.environment.projectId
        );
        const locationGroup = groupedEnvironments.find(
          (group) => group.projectId === locationData.data.environment.projectId
        );

        if (!sourceGroup || !locationGroup) {
          console.error("Source group not found", { sourceGroup, locationGroup });
          return;
        }

        const targetOrder = locationGroup.environments.find(
          (env) => env.id === locationData.data.environment.id
        )?.order;

        if (targetOrder === undefined || targetOrder === undefined || !instruction) {
          console.error("Source, target or instruction not found", { targetOrder, instruction });
          return;
        }

        const targetDropOrder = instruction.operation === "reorder-before" ? targetOrder : targetOrder + 1;

        //delete grouped environment
        await deleteEnvironment({ id: sourceData.data.environment.id });

        //get reordered grouped environments after the deleted one
        const groupedEnvironmentsToUpdateSource = sourceGroup.environments
          .filter((env) => env.order! > sourceData.data.environment.order!)
          .map((env) => ({
            id: env.id,
            order: env.order! - 1,
          }));

        //add new grouped environment
        const newGroupedEnvironment = await createEnvironment({
          projectId: locationData.data.environment.projectId,
          name: sourceData.data.environment.name,
          order: targetDropOrder,
          variables: [],
        });

        //get reordered grouped environments after new grouped environment
        const groupedEnvironmentsToUpdateLocation = locationGroup.environments
          .filter((env) => env.order! >= targetDropOrder && env.id !== newGroupedEnvironment.id)
          .map((env) => ({
            id: env.id,
            order: env.order! + 1,
          }));

        //update grouped environments
        const envsToUpdate = [...groupedEnvironmentsToUpdateSource, ...groupedEnvironmentsToUpdateLocation];
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
    [batchUpdateEnvironment, createEnvironment, deleteEnvironment, globalEnvironments, groupedEnvironments]
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
        const globalEnvironmentsToUpdate = globalEnvironments
          .filter(
            (globalEnv) =>
              (globalEnv.order ?? 0) > (sourceData.data.environment.order ?? 0) &&
              globalEnv.id !== sourceData.data.environment.id
          )
          .map((globalEnv) => ({
            id: globalEnv.id,
            order: (globalEnv.order ?? 0) - 1,
          }));

        //update global and grouped environments
        await batchUpdateEnvironment({
          items: globalEnvironmentsToUpdate.map((env) => ({
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

        const sourceGroup = groupedEnvironments.find(
          (group) => group.projectId === sourceData.data.environment.projectId
        );

        if (!sourceGroup) return;

        //delete grouped environment
        await deleteEnvironment({ id: sourceData.data.environment.id });

        //get reordered grouped environments after the deleted one
        const groupedEnvironmentsToUpdate = sourceGroup.environments
          .filter((env) => env.order! > sourceData.data.environment.order!)
          .filter((env) => env.id !== sourceData.data.environment.id)
          .map((env) => ({
            id: env.id,
            order: env.order! - 1,
          }));

        await createEnvironment({
          projectId: locationData.data.groupWithEnvironments.projectId,
          name: sourceData.data.environment.name,
          order: locationGroupedEnvironments.length + 1,
          variables: [],
        });

        //update grouped environments
        await batchUpdateEnvironment({
          items: groupedEnvironmentsToUpdate.map((env) => ({
            id: env.id,
            order: env.order!,
            varsToAdd: [],
            varsToUpdate: [],
            varsToDelete: [],
          })),
        });
      }
    },
    [batchUpdateEnvironment, createEnvironment, deleteEnvironment, globalEnvironments, groupedEnvironments]
  );

  const handleReorderGrouped = useCallback(
    async (source: ElementDragPayload, location: DragLocationHistory) => {
      const sourceData = getSourceGroupedEnvironmentItemData(source);
      const locationData = getLocationGroupedEnvironmentItemData(location);

      if (!sourceData || !locationData) return;

      if (sourceData.data.environment.projectId !== locationData.data.environment.projectId) return;

      const groupEnvs = groupedEnvironments.find((env) => env.projectId === sourceData.data.environment.projectId);

      if (!groupEnvs) return;

      const sourceIndex = groupEnvs?.environments.findIndex((env) => env.id === sourceData.data.environment.id);
      const targetIndex = groupEnvs?.environments.findIndex((env) => env.id === locationData.data.environment.id);
      const instruction = locationData.instruction;

      if (sourceIndex === -1 || targetIndex === -1 || !instruction) {
        console.error("Source, target or instruction not found", { sourceIndex, targetIndex, instruction });
        return;
      }

      const dropOrder = instruction.operation === "reorder-before" ? targetIndex : (targetIndex ?? 0) + 1;

      const inserted = [
        ...groupEnvs.environments.slice(0, dropOrder).filter((env) => env.id !== sourceData.data.environment.id),
        sourceData.data.environment,
        ...groupEnvs.environments.slice(dropOrder).filter((env) => env.id !== sourceData.data.environment.id),
      ];

      const reordered = inserted.map((env, index) => ({
        id: env.id,
        name: env.name,
        order: index + 1,
      }));

      const environmentsToUpdateReordered = reordered.filter((env) => {
        const environmentUnderQuestion = groupEnvs.environments.find((sortedEnv) => sortedEnv.id === env.id);
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
    [batchUpdateEnvironment, groupedEnvironments]
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

      const group = groupedEnvironments.find((group) => group.projectId === sourceData.data.environment.projectId);

      if (!group) {
        console.error("Group environment not found", { locationData });
        return;
      }

      const targetOrder = globalEnvironments.find((env) => env.id === locationData.data.environment.id)?.order;

      if (targetOrder === undefined || targetOrder === undefined || !instruction) {
        console.error("Source, target or instruction not found", { targetOrder, instruction });
        return;
      }
      const dropOrder = instruction.operation === "reorder-before" ? targetOrder : targetOrder + 1;

      //delete grouped environment
      await deleteEnvironment({ id: sourceData.data.environment.id });

      //get reordered grouped environments after the deleted one
      const groupedEnvsToUpdate = group?.environments
        .filter((env) => env.order! > sourceData.data.environment.order!)
        .map((env) => ({
          id: env.id,
          order: env.order! - 1,
        }));

      //add new global environment
      const newGlobalEnv = await createEnvironment({
        name: sourceData.data.environment.name,
        order: dropOrder,
        variables: [],
      });

      //get reordered global environments after new global environment
      const globalEnvToUpdate =
        globalEnvironments
          .filter((env) => env.order! >= dropOrder && env.id !== newGlobalEnv.id)
          .map((env) => ({
            id: env.id,
            order: env.order! + 1,
          })) ?? [];

      //update global and grouped environments
      const envsToUpdate = [...groupedEnvsToUpdate, ...globalEnvToUpdate];
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
    [batchUpdateEnvironment, createEnvironment, deleteEnvironment, globalEnvironments, groupedEnvironments]
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

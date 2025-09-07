import { useCallback, useEffect } from "react";

import { useCreateEnvironment, useDeleteEnvironment, useStreamEnvironments } from "@/hooks";
import { useBatchUpdateEnvironment } from "@/hooks/workspace/environment/useBatchUpdateEnvironment";
import { monitorForElements } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";

import { GlobalEnvironmentItem, GroupedEnvironmentItem } from "../types";
import {
  getDropOperation,
  getLocationEnvironmentItemData,
  getLocationGlobalEnvironmentItemData,
  getLocationGroupedEnvironmentItemData,
  getSourceEnvironmentItem,
  getSourceGlobalEnvironmentItemData,
  isSourceEnvironmentItem,
} from "../utils";
import { useGroupedEnvironments } from "./useGroupedEnvironments";

export const useMonitorEnvironmentsLists = () => {
  const { globalEnvironments } = useStreamEnvironments();
  const { mutateAsync: batchUpdateEnvironment } = useBatchUpdateEnvironment();
  const { mutateAsync: deleteEnvironment } = useDeleteEnvironment();
  const { mutateAsync: createEnvironment } = useCreateEnvironment();
  const { groupedEnvironments } = useGroupedEnvironments();

  const handleReorderGlobal = useCallback(
    async (sourceData: GlobalEnvironmentItem, locationData: GlobalEnvironmentItem) => {
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

  const moveToGrouped = useCallback(
    async (sourceData: GlobalEnvironmentItem, locationData: GroupedEnvironmentItem) => {
      const instruction = locationData.instruction;

      const groupedEnvironment = groupedEnvironments.find(
        (group) => group.collectionId === locationData.data.environment.collectionId
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
      const globalEnvironmentsToUpdate = globalEnvironments
        .filter((env) => env.order! > sourceData.data.environment.order!)
        .map((env) => ({
          id: env.id,
          order: env.order! - 1,
        }));

      //add new grouped environment
      const newGroupedEnvironment = await createEnvironment({
        collectionId: locationData.data.environment.collectionId,
        name: sourceData.data.environment.name,
        order: dropOrder,
        variables: [],
      });

      console.log({ dropOrder });
      //update grouped environments after the target index
      const groupedEnvironmentsToUpdate =
        groupedEnvironment?.environments
          .filter((env) => env.order! >= dropOrder && env.id !== newGroupedEnvironment.id)
          .map((env) => ({
            id: env.id,
            order: env.order! + 1,
          })) ?? [];

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
    },
    [batchUpdateEnvironment, createEnvironment, deleteEnvironment, globalEnvironments, groupedEnvironments]
  );

  useEffect(() => {
    return monitorForElements({
      canMonitor({ source }) {
        return isSourceEnvironmentItem(source);
      },
      onDrop({ source, location }) {
        if (!isSourceEnvironmentItem(source)) return;

        const sourceData = getSourceEnvironmentItem(source);
        const locationData = getLocationEnvironmentItemData(location);

        if (!sourceData || !locationData || !locationData.instruction) return;

        const dropOperation = getDropOperation(sourceData, locationData, locationData.instruction);

        const sourceGlobalEnvironmentItemData = getSourceGlobalEnvironmentItemData(source);
        const locationGlobalEnvironmentItemData = getLocationGlobalEnvironmentItemData(location);

        const locationGroupedEnvironmentItemData = getLocationGroupedEnvironmentItemData(location);

        switch (dropOperation) {
          case "ReorderGlobal":
            if (!sourceGlobalEnvironmentItemData || !locationGlobalEnvironmentItemData) return;
            handleReorderGlobal(sourceGlobalEnvironmentItemData, locationGlobalEnvironmentItemData);
            break;
          case "ReorderGrouped":
            console.log("ReorderGrouped");
            break;
          case "MoveToGlobal":
            console.log("MoveToGlobal");
            break;
          case "MoveToGrouped":
            if (!sourceGlobalEnvironmentItemData || !locationGroupedEnvironmentItemData) return;
            moveToGrouped(sourceGlobalEnvironmentItemData, locationGroupedEnvironmentItemData);
            break;
          case "CombineGrouped":
            console.log("CombineGrouped");
            break;
          default:
            break;
        }
      },
    });
  }, [handleReorderGlobal, moveToGrouped]);
};

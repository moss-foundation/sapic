import { useCallback, useEffect } from "react";

import { useCreateEnvironment, useDeleteEnvironment, useStreamEnvironments } from "@/hooks";
import { useBatchUpdateEnvironment } from "@/hooks/workspace/environment/useBatchUpdateEnvironment";
import { DragLocationHistory } from "@atlaskit/pragmatic-drag-and-drop/dist/types/internal-types";
import { ElementDragPayload, monitorForElements } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";

import {
  getDropOperation,
  getLocationGlobalEnvironmentItemData,
  getLocationGroupedEnvironmentItemData,
  getLocationGroupedEnvironmentListData,
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

  const moveToGrouped = useCallback(
    async (source: ElementDragPayload, location: DragLocationHistory) => {
      const sourceData = getSourceGlobalEnvironmentItemData(source);
      const locationData = getLocationGroupedEnvironmentItemData(location);

      if (!sourceData || !locationData) return;

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

      //get reordered global environments after the deleted one
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
    },
    [batchUpdateEnvironment, createEnvironment, deleteEnvironment, globalEnvironments, groupedEnvironments]
  );

  const combineToGrouped = useCallback(
    async (source: ElementDragPayload, location: DragLocationHistory) => {
      const sourceData = getSourceEnvironmentItem(source);
      const locationData = getLocationGroupedEnvironmentListData(location);

      if (!sourceData || !locationData) return;

      const groupedEnvironments = locationData.data.groupWithEnvironments.environments;

      if (sourceData.type === "GlobalEnvironmentItem") {
        //delete global environment
        await deleteEnvironment({ id: sourceData.data.environment.id });

        //get reordered global environments after the deleted one
        const globalEnvironmentsToUpdate = groupedEnvironments
          .filter((env) => env.order! > sourceData.data.environment.order!)
          .map((env) => ({
            id: env.id,
            order: env.order! - 1,
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
          collectionId: locationData.data.groupWithEnvironments.collectionId,
          name: sourceData.data.environment.name,
          order: groupedEnvironments.length + 1,
          variables: [],
        });
      } else {
        console.log("CombineToGrouped", { sourceData, locationData });
      }
    },
    [batchUpdateEnvironment, createEnvironment, deleteEnvironment]
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
        console.log("onDrop", { dropOperation });
        switch (dropOperation) {
          case "ReorderGlobals":
            handleReorderGlobals(source, location);
            break;
          case "ReorderGrouped":
            console.log("ReorderGrouped");
            break;
          case "MoveToGlobal":
            console.log("MoveToGlobal");
            break;
          case "MoveToGrouped":
            moveToGrouped(source, location);
            break;
          case "CombineToGrouped":
            combineToGrouped(source, location);
            break;

          default:
            break;
        }
      },
    });
  }, [combineToGrouped, handleReorderGlobals, moveToGrouped]);
};

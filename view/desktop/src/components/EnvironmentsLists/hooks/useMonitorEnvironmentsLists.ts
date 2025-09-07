import { useCallback, useEffect } from "react";

import { useStreamEnvironments } from "@/hooks";
import { useBatchUpdateEnvironment } from "@/hooks/workspace/environment/useBatchUpdateEnvironment";
import { Instruction } from "@atlaskit/pragmatic-drag-and-drop-hitbox/list-item";
import { monitorForElements } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";

import { DropEnvironmentItem, GlobalEnvironmentItem } from "../types";
import {
  getDropOperation,
  getLocationEnvironmentItemData,
  getSourceEnvironmentItem,
  getSourceGlobalEnvironmentItemData,
  isSourceEnvironmentItem,
} from "../utils";

export const useMonitorEnvironmentsLists = () => {
  const { globalEnvironments } = useStreamEnvironments();
  const { mutateAsync: batchUpdateEnvironment } = useBatchUpdateEnvironment();

  const handleReorderGlobal = useCallback(
    async (sourceData: GlobalEnvironmentItem, locationData: DropEnvironmentItem, instruction: Instruction) => {
      const sourceIndex = globalEnvironments.findIndex((env) => env.id === sourceData.data.environment.id);
      const targetIndex = globalEnvironments.findIndex((env) => env.id === locationData.data.environment.id);

      if (sourceIndex === -1 || targetIndex === -1) {
        console.error("Source or target environment not found", { sourceIndex, targetIndex });
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
        switch (dropOperation) {
          case "ReorderGlobal":
            const sourceGlobalEnvironmentItemData = getSourceGlobalEnvironmentItemData(source);
            if (!sourceGlobalEnvironmentItemData) return;
            handleReorderGlobal(sourceGlobalEnvironmentItemData, locationData, locationData.instruction);
            break;
          case "ReorderGrouped":
            console.log("ReorderGrouped");
            break;
          case "MoveToGlobal":
            console.log("MoveToGlobal");
            break;
          case "MoveToGrouped":
            console.log("MoveToGrouped");
            break;
          case "CombineGrouped":
            console.log("CombineGrouped");
            break;
          default:
            break;
        }
      },
    });
  }, [handleReorderGlobal]);
};

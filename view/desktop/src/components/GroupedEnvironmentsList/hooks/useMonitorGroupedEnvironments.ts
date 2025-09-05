import { useEffect } from "react";

import { useBatchUpdateEnvironmentGroup } from "@/hooks/workspace/environment/useBatchUpdateEnvironmentGroup";
import { extractInstruction } from "@atlaskit/pragmatic-drag-and-drop-hitbox/list-item";
import { monitorForElements } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";

import {
  getLocationGroupedEnvironmentsListItem,
  getSourceGroupedEnvironmentsListItem,
  isSourceGroupedEnvironmentsListItem,
} from "../utils";
import { useGroupedWithEnvironments } from "./useGroupedWithEnvironments";

export const useMonitorGroupedEnvironments = () => {
  const { mutate: batchUpdateEnvironmentGroup } = useBatchUpdateEnvironmentGroup();
  const { groupedWithEnvironments } = useGroupedWithEnvironments();

  useEffect(() => {
    return monitorForElements({
      canMonitor({ source }) {
        return isSourceGroupedEnvironmentsListItem(source);
      },

      onDrop: async ({ source, location }) => {
        const sourceData = getSourceGroupedEnvironmentsListItem(source);
        const locationData = getLocationGroupedEnvironmentsListItem(location);
        const instruction = extractInstruction(location.current.dropTargets[0]?.data);

        if (!sourceData || !locationData || !instruction || !groupedWithEnvironments) return;

        if (sourceData.data.groupWithEnvironments.collectionId === locationData.data.groupWithEnvironments.collectionId)
          return;

        const dropOrder =
          instruction?.operation === "reorder-before"
            ? locationData.data.groupWithEnvironments.order! - 0.5
            : locationData.data.groupWithEnvironments.order! + 0.5;

        const dropTargetEnvironmentsWithNewOrders = [
          ...groupedWithEnvironments
            .slice(0, dropOrder)
            .filter((env) => env.collectionId !== sourceData.data.groupWithEnvironments.collectionId),
          sourceData.data.groupWithEnvironments,
          ...groupedWithEnvironments
            .slice(dropOrder)
            .filter((env) => env.collectionId !== sourceData.data.groupWithEnvironments.collectionId),
        ].map((entry, index) => ({ ...entry, order: index + 1 }));

        const environmentsToUpdate = dropTargetEnvironmentsWithNewOrders.filter((env) => {
          const oldEnv = groupedWithEnvironments.find((e) => e.collectionId === env.collectionId);
          return oldEnv?.order !== env.order;
        });

        batchUpdateEnvironmentGroup({
          items: environmentsToUpdate.map((env) => ({
            collectionId: env.collectionId,
            order: env.order,
          })),
        });
      },
    });
  }, [batchUpdateEnvironmentGroup, groupedWithEnvironments]);
};

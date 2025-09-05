import { useEffect } from "react";

import { useStreamEnvironments, useUpdateEnvironment } from "@/hooks";
import { extractInstruction } from "@atlaskit/pragmatic-drag-and-drop-hitbox/list-item";
import { monitorForElements } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";

import {
  getLocationGlobalEnvironmentsListItem,
  getSourceGlobalEnvironmentsListItem,
  isSourceGlobalEnvironmentsListItem,
} from "../GlobalEnvironmentsListItem/utils";

export const useMonitorGlobalEnvironmentsList = () => {
  const { globalEnvironments } = useStreamEnvironments();
  const { mutate: updateEnvironment } = useUpdateEnvironment();

  useEffect(() => {
    return monitorForElements({
      canMonitor({ source }) {
        return isSourceGlobalEnvironmentsListItem(source);
      },
      onDrop: async ({ source, location }) => {
        const sourceData = getSourceGlobalEnvironmentsListItem(source);
        const locationData = getLocationGlobalEnvironmentsListItem(location);
        const instruction = extractInstruction(location.current.dropTargets[0]?.data);

        if (!sourceData || !locationData || !globalEnvironments || !instruction) return;

        if (sourceData.data.environment.id === locationData.data.environment.id) return;

        const dropOrder =
          instruction?.operation === "reorder-before"
            ? locationData.data.environment.order! - 0.5
            : locationData.data.environment.order! + 0.5;

        const dropTargetEnvironmentsWithNewOrders = [
          ...globalEnvironments.slice(0, dropOrder).filter((env) => env.id !== sourceData.data.environment.id),
          sourceData.data.environment,
          ...globalEnvironments.slice(dropOrder).filter((env) => env.id !== sourceData.data.environment.id),
        ].map((entry, index) => ({ ...entry, order: index + 1 }));

        const environmentsToUpdate = dropTargetEnvironmentsWithNewOrders.filter((env) => {
          const oldEnv = globalEnvironments.find((e) => e.id === env.id);
          return oldEnv?.order !== env.order;
        });

        //TODO This should use batch update in the future, when it's supported by the backend
        const dropTargetEnvironmentsToUpdate = environmentsToUpdate.map((environment) => {
          updateEnvironment({
            id: environment.id,
            order: environment.order,
            varsToAdd: [],
            varsToUpdate: [],
            varsToDelete: [],
          });
        });

        await Promise.all(dropTargetEnvironmentsToUpdate);
      },
    });
  }, [globalEnvironments, updateEnvironment]);
};

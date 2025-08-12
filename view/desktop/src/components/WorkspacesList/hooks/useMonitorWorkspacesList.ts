import { useEffect } from "react";

import { useStreamEnvironments, useUpdateEnvironment } from "@/hooks/environment";
import { monitorForElements } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";

import {
  getLocationWorkspacesListItem,
  getSourceWorkspacesListItem,
  isSourceWorkspacesListItem,
} from "../WorkspacesListItem/utils";

export const useMonitorWorkspacesList = () => {
  const { environmentsSortedByOrder } = useStreamEnvironments();
  const { mutate: updateEnvironment } = useUpdateEnvironment();

  useEffect(() => {
    return monitorForElements({
      canMonitor({ source }) {
        return isSourceWorkspacesListItem(source);
      },
      onDrop: async ({ source, location }) => {
        const sourceData = getSourceWorkspacesListItem(source);
        const locationData = getLocationWorkspacesListItem(location);

        if (!sourceData || !locationData || !environmentsSortedByOrder) return;

        if (sourceData.data.environment.id === locationData.data.environment.id) return;

        const dropOrder =
          locationData.edge === "top"
            ? locationData.data.environment.order! - 0.5
            : locationData.data.environment.order! + 0.5;

        const dropTargetEnvironmentsWithNewOrders = [
          ...environmentsSortedByOrder.slice(0, dropOrder).filter((env) => env.id !== sourceData.data.environment.id),
          sourceData.data.environment,
          ...environmentsSortedByOrder.slice(dropOrder).filter((env) => env.id !== sourceData.data.environment.id),
        ].map((entry, index) => ({ ...entry, order: index + 1 }));

        const environmentsToUpdate = dropTargetEnvironmentsWithNewOrders.filter((env) => {
          const oldEnv = environmentsSortedByOrder.find((e) => e.id === env.id);

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
  }, [environmentsSortedByOrder, updateEnvironment]);
};

import { useEffect } from "react";

import { useAllStreamedProjectEnvironments } from "@/adapters/tanstackQuery/environment/derived/useAllStreamedProjectEnvironments";
import { useBatchUpdateEnvironmentGroup } from "@/adapters/tanstackQuery/environment/useBatchUpdateEnvironmentGroup";
import { monitorForElements } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";

import {
  getLocationGroupedEnvironmentListData,
  getSourceGroupedEnvironmentListData,
  isLocationGroupedEnvironmentList,
  isSourceGroupedEnvironmentList,
} from "../utils";

export const useMonitorEnvironmentsLists = () => {
  const { allProjectEnvironments } = useAllStreamedProjectEnvironments();
  const { mutate: batchUpdateEnvironmentGroup } = useBatchUpdateEnvironmentGroup();

  useEffect(() => {
    return monitorForElements({
      canMonitor({ source }) {
        return isSourceGroupedEnvironmentList(source);
      },
      onDrop({ source, location }) {
        if (!allProjectEnvironments) {
          console.warn("can't drop: no project environments", allProjectEnvironments);
          return;
        }

        if (!isSourceGroupedEnvironmentList(source) || !isLocationGroupedEnvironmentList(location)) {
          console.warn("can't drop: no source or location", source, location);
          return;
        }

        const sourceData = getSourceGroupedEnvironmentListData(source);
        const locationData = getLocationGroupedEnvironmentListData(location);
        const instruction = locationData?.instruction;

        if (!sourceData || !locationData) {
          console.warn("can't drop: no source or location", sourceData, locationData);
          return;
        }

        if (!instruction || instruction.blocked) {
          console.warn("can't drop: blocked", instruction);
          return;
        }

        const targetIndex = allProjectEnvironments.findIndex(
          (group) => group.projectId === locationData.data.groupWithEnvironments.projectId
        );

        const inserted = [
          ...allProjectEnvironments
            .slice(0, targetIndex)
            .filter((environment) => environment.projectId !== sourceData.data.groupWithEnvironments.projectId),
          sourceData.data.groupWithEnvironments,
          ...allProjectEnvironments
            .slice(targetIndex)
            .filter((environment) => environment.projectId !== sourceData.data.groupWithEnvironments.projectId),
        ].map((environment, index) => ({
          ...environment,
          order: index + 1,
        }));

        const groupsToUpdate = inserted.filter((group) => {
          const groupInLocation = allProjectEnvironments.find((g) => g.projectId === group.projectId);
          return groupInLocation?.order !== group.order;
        });

        batchUpdateEnvironmentGroup({
          items: groupsToUpdate.map((environment) => ({
            order: environment.order,
            projectId: environment.projectId ?? "",
          })),
        });
      },
    });
  }, [batchUpdateEnvironmentGroup, allProjectEnvironments]);
};

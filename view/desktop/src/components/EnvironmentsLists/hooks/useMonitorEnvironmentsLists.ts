import { useEffect } from "react";

import { useStreamEnvironments } from "@/hooks";
import { useBatchUpdateEnvironmentGroup } from "@/hooks/workspace/environment/useBatchUpdateEnvironmentGroup";
import { monitorForElements } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";

import {
  getLocationGroupedEnvironmentListData,
  getSourceGroupedEnvironmentListData,
  isLocationGroupedEnvironmentList,
  isSourceGroupedEnvironmentList,
} from "../utils";

export const useMonitorEnvironmentsLists = () => {
  const { groups } = useStreamEnvironments();
  const { mutate: batchUpdateEnvironmentGroup } = useBatchUpdateEnvironmentGroup();

  useEffect(() => {
    return monitorForElements({
      canMonitor({ source }) {
        return isSourceGroupedEnvironmentList(source);
      },
      onDrop({ source, location }) {
        if (!groups) {
          console.warn("can't drop: no groups", groups);
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

        const targetIndex = groups.findIndex(
          (group) => group.collectionId === locationData.data.groupWithEnvironments.collectionId
        );

        console.log("targetIndex", targetIndex);

        const inserted = [
          ...groups
            .slice(0, targetIndex)
            .filter((group) => group.collectionId !== sourceData.data.groupWithEnvironments.collectionId),
          sourceData.data.groupWithEnvironments,
          ...groups
            .slice(targetIndex)
            .filter((group) => group.collectionId !== sourceData.data.groupWithEnvironments.collectionId),
        ].map((group, index) => ({
          ...group,
          order: index + 1,
        }));

        const groupsToUpdate = inserted.filter((group) => {
          const groupInLocation = groups.find((g) => g.collectionId === group.collectionId);
          return groupInLocation?.order !== group.order;
        });

        batchUpdateEnvironmentGroup({
          items: groupsToUpdate.map((group) => ({
            order: group.order,
            collectionId: group.collectionId,
          })),
        });
      },
    });
  }, [batchUpdateEnvironmentGroup, groups]);
};

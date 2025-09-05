import { useMemo } from "react";

import { useStreamCollections, useStreamEnvironments } from "@/hooks";
import { sortObjectsByOrder } from "@/utils/sortObjectsByOrder";

import { GroupedWithEnvironment } from "../types";

export const useGroupedWithEnvironments = () => {
  const { data: collections } = useStreamCollections();
  const { groups, groupedEnvironments } = useStreamEnvironments();

  const groupedWithEnvironments: GroupedWithEnvironment[] = useMemo(() => {
    if (!collections || !groups || !groupedEnvironments) return [];

    const groupedWithEnvironments = groups.map((group) => {
      return {
        ...group,
        environments: groupedEnvironments.filter((environment) => environment.collectionId === group.collectionId),
      };
    });

    return sortObjectsByOrder(groupedWithEnvironments);
  }, [collections, groups, groupedEnvironments]);

  return { groupedWithEnvironments };
};

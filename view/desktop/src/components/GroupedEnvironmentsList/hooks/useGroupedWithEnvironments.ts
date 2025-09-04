import { useMemo } from "react";

import { useStreamCollections, useStreamEnvironments } from "@/hooks";

import { GroupedWithEnvironment } from "../types";

export const useGroupedWithEnvironments = () => {
  const { data: collections } = useStreamCollections();
  const { groups, groupedEnvironments } = useStreamEnvironments();

  const groupedWithEnvironments: GroupedWithEnvironment[] = useMemo(() => {
    if (!collections || !groups || !groupedEnvironments) return [];

    return groups.map((group) => {
      return {
        ...group,
        environments: groupedEnvironments.filter((environment) => environment.collectionId === group.collectionId),
      };
    });
  }, [collections, groups, groupedEnvironments]);

  return { groupedWithEnvironments };
};

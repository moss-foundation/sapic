import { useMemo } from "react";

import { useStreamEnvironments, useStreamProjects } from "@/hooks";
import { sortObjectsByOrder } from "@/utils/sortObjectsByOrder";

import { GroupedEnvironments } from "../types";

export const useGroupedEnvironments = () => {
  const { data: collections } = useStreamProjects();
  const { groups, collectionEnvironments } = useStreamEnvironments();

  const groupedEnvironments: GroupedEnvironments[] = useMemo(() => {
    if (!collections || !groups || !collectionEnvironments) return [];

    const groupedEnvironments = groups.map((group) => {
      return {
        ...group,
        environments: collectionEnvironments.filter((environment) => environment.collectionId === group.collectionId),
      };
    });

    return sortObjectsByOrder(groupedEnvironments);
  }, [collectionEnvironments, collections, groups]);

  return { groupedEnvironments };
};

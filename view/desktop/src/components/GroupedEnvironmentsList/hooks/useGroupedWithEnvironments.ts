import { useMemo } from "react";

import { useStreamCollections, useStreamEnvironments } from "@/hooks";

import { GroupedWithEnvironment } from "../types";

export const useGroupedWithEnvironments = () => {
  const { data: collections } = useStreamCollections();
  const { groupedEnvironments } = useStreamEnvironments();

  const groupedWithEnvironments: GroupedWithEnvironment[] = useMemo(() => {
    if (!collections || !groupedEnvironments) return [];

    return collections
      .map((collection) => {
        const collectionEnvironments = groupedEnvironments.filter(
          (environment) => environment.collectionId === collection.id
        );

        return {
          ...collection,
          environments: collectionEnvironments,
        };
      })
      .filter((collection) => collection.environments.length > 0);
  }, [collections, groupedEnvironments]);

  return { groupedWithEnvironments };
};

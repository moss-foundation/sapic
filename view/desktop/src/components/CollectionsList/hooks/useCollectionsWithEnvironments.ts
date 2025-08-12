import { useMemo } from "react";

import { useStreamedCollections } from "@/hooks/collection";
import { useStreamEnvironments } from "@/hooks/environment";

import { CollectionWithEnvironment } from "../types";

export const useCollectionsWithEnvironments = () => {
  const { data: collections } = useStreamedCollections();
  const { data: environments } = useStreamEnvironments();

  const collectionsWithEnvironments: CollectionWithEnvironment[] = useMemo(() => {
    if (!collections || !environments) return [];

    return collections
      .map((collection) => {
        const collectionEnvironments = environments.filter((environment) => environment.collectionId === collection.id);

        return {
          ...collection,
          environments: collectionEnvironments,
        };
      })
      .filter((collection) => collection.environments.length > 0);
  }, [collections, environments]);

  return { collectionsWithEnvironments };
};

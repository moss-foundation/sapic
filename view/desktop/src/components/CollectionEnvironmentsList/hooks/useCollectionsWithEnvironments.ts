import { useMemo } from "react";

import { useStreamedCollections } from "@/hooks/collection";
import { useStreamEnvironments } from "@/hooks/environment";

import { CollectionWithEnvironment } from "../types";

export const useCollectionsWithEnvironments = () => {
  const { data: collections } = useStreamedCollections();
  const { collectionsEnvironments } = useStreamEnvironments();

  const collectionsWithEnvironments: CollectionWithEnvironment[] = useMemo(() => {
    if (!collections || !collectionsEnvironments) return [];

    return collections
      .map((collection) => {
        const collectionEnvironments = collectionsEnvironments.filter(
          (environment) => environment.collectionId === collection.id
        );

        return {
          ...collection,
          environments: collectionEnvironments,
        };
      })
      .filter((collection) => collection.environments.length > 0);
  }, [collections, collectionsEnvironments]);

  return { collectionsWithEnvironments };
};

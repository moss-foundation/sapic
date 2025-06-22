import { CollectionTree } from "@/components/CollectionTreeV2/types";
import { CollectionInfo } from "@repo/moss-workspace";
import { useQuery } from "@tanstack/react-query";

export const USE_LIST_COLLECTIONS_QUERY_KEY = "listCollections";

const listCollectionsFn = async (workspaceId: string | null): Promise<CollectionInfo[]> => {
  if (!workspaceId) return [];

  // TODO: Implement proper collection listing once the Tauri command is available
  // For now, return mock collections from the store
  console.warn("Collection listing not yet implemented - returning mock data from store");

  // Import store data synchronously for the mock implementation
  const { useCollectionsStore } = await import("@/store/collections");
  const collections = useCollectionsStore.getState().collections;

  return collections.map((collection, index) => ({
    id: typeof collection.id === "string" ? index + 1 : (collection.id as number),
    displayName: typeof collection.id === "string" ? collection.id : `Collection ${collection.id}`,
    order: collection.order ?? undefined,
  }));
};

export const useListCollections = (workspaceId: string | null) => {
  return useQuery<CollectionInfo[], Error>({
    queryKey: [USE_LIST_COLLECTIONS_QUERY_KEY, workspaceId],
    queryFn: () => listCollectionsFn(workspaceId),
    enabled: !!workspaceId,
    // Provide initial data from store
    initialData: workspaceId ? [] : [],
  });
};

const getInitialData = (collections: CollectionTree[]) => {
  console.log("getInitialData", collections);
  return collections.map((collection, index) => ({
    id: typeof collection.id === "string" ? index + 1 : (collection.id as number),
    displayName: typeof collection.id === "string" ? collection.id : `Collection ${collection.id}`,
    order: collection.order ?? undefined,
  }));
};

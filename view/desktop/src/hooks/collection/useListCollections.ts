import { CollectionInfo } from "@repo/moss-workspace";
import { useQuery } from "@tanstack/react-query";

export const USE_LIST_COLLECTIONS_QUERY_KEY = "listCollections";

const listCollectionsFn = async (workspaceId: string | null): Promise<CollectionInfo[]> => {
  if (!workspaceId) return [];

  // TODO: Implement proper collection listing once the Tauri command is available
  // For now, return empty array to prevent errors
  console.warn("Collection listing not yet implemented - returning empty array");
  return [];
};

export const useListCollections = (workspaceId: string | null) => {
  return useQuery<CollectionInfo[], Error>({
    queryKey: [USE_LIST_COLLECTIONS_QUERY_KEY, workspaceId],
    queryFn: () => listCollectionsFn(workspaceId),
    enabled: !!workspaceId,
  });
};

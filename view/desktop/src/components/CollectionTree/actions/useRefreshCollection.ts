import { USE_STREAMED_COLLECTION_ENTRIES_QUERY_KEY } from "@/hooks";
import { useFetchEntriesForPath } from "@/hooks/collection/derivedHooks/useFetchEntriesForPath";
import { useQueryClient } from "@tanstack/react-query";

export const useRefreshCollection = (collectionId: string) => {
  const queryClient = useQueryClient();

  const { fetchEntriesForPath } = useFetchEntriesForPath();

  const refreshCollection = async () => {
    queryClient.invalidateQueries({
      queryKey: [USE_STREAMED_COLLECTION_ENTRIES_QUERY_KEY, collectionId],
    });
    queryClient.removeQueries({ queryKey: [USE_STREAMED_COLLECTION_ENTRIES_QUERY_KEY, collectionId] });

    await fetchEntriesForPath(collectionId, "");
  };

  return { refreshCollection };
};

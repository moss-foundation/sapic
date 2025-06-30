import { EntryInfo } from "@repo/moss-collection";
import { useQuery } from "@tanstack/react-query";

// Import the shared query function
import { fetchCollectionEntries } from "./queries/fetchCollectionEntries";

export const USE_STREAM_COLLECTION_ENTRIES_QUERY_KEY = "streamCollectionEntries";

export const useStreamCollectionEntries = (collectionId: string) => {
  return useQuery<EntryInfo[], Error>({
    queryKey: [USE_STREAM_COLLECTION_ENTRIES_QUERY_KEY, collectionId],
    queryFn: () => fetchCollectionEntries(collectionId),
    placeholderData: [],
  });
};

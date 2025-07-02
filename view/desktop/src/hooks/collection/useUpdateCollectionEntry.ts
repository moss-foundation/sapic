import { EntryInfo } from "@repo/moss-collection";
import { useQueryClient } from "@tanstack/react-query";
import { join } from "@tauri-apps/api/path";

import { USE_STREAM_COLLECTION_ENTRIES_QUERY_KEY } from "./useStreamCollectionEntries";

export interface UseUpdateCollectionEntryInput {
  collectionId: string;
  updatedEntry: EntryInfo;
}

export const useUpdateCollectionEntry = () => {
  const queryClient = useQueryClient();

  const placeholderFnForUpdateCollectionEntry = async ({
    collectionId,
    updatedEntry,
  }: UseUpdateCollectionEntryInput) => {
    const currentData = queryClient.getQueryData([USE_STREAM_COLLECTION_ENTRIES_QUERY_KEY, collectionId]) as
      | EntryInfo[]
      | undefined;

    if (!currentData) {
      return;
    }

    const entryBeforeUpdate = currentData.find((e) => e.id === updatedEntry.id);

    if (!entryBeforeUpdate) {
      return;
    }

    const newEntries = await Promise.all(
      currentData.map(async (oldEntry) => {
        if (oldEntry.id === updatedEntry.id) {
          return updatedEntry;
        }

        if (updatedEntry.kind === "Dir") {
          if (checkIfEntryIsInUpdatedEntry(oldEntry, entryBeforeUpdate)) {
            const newSegments = oldEntry.path.segments.map((segment) =>
              segment === entryBeforeUpdate.name ? updatedEntry.name : segment
            );

            const newPath = await join(...newSegments);

            return {
              ...oldEntry,
              path: {
                segments: newSegments,
                raw: newPath,
              },
            };
          }
        }

        return oldEntry;
      })
    );

    console.log("newEntries", newEntries);

    queryClient.setQueryData([USE_STREAM_COLLECTION_ENTRIES_QUERY_KEY, collectionId], newEntries);
  };

  return {
    placeholderFnForUpdateCollectionEntry,
  };
};

const checkIfEntryIsInUpdatedEntry = (oldEntry: EntryInfo, updatedEntry: EntryInfo) => {
  return updatedEntry.path.segments.every((p) => {
    return oldEntry.path.segments.includes(p);
  });
};

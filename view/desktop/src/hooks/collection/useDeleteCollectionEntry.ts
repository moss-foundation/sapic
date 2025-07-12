import { invokeTauriIpc } from "@/lib/backend/tauri";
import { DeleteEntryInput, DeleteEntryOutput, EntryInfo } from "@repo/moss-collection";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { useBatchUpdateCollectionEntry } from "./useBatchUpdateCollectionEntry";
import { USE_STREAMED_COLLECTION_ENTRIES_QUERY_KEY } from "./useStreamedCollectionEntries";

export interface UseDeleteCollectionEntryInput {
  collectionId: string;
  input: DeleteEntryInput;
}

const deleteCollectionEntry = async ({ collectionId, input }: UseDeleteCollectionEntryInput) => {
  const result = await invokeTauriIpc<DeleteEntryOutput>("delete_collection_entry", { collectionId, input });

  if (result.status === "error") {
    throw new Error(String(result.error));
  }

  return result.data;
};

export const useDeleteCollectionEntry = () => {
  const queryClient = useQueryClient();
  const { mutateAsync: batchUpdateCollectionEntry } = useBatchUpdateCollectionEntry();

  return useMutation<DeleteEntryOutput, Error, UseDeleteCollectionEntryInput>({
    mutationFn: deleteCollectionEntry,
    onSuccess: async (data, variables) => {
      // Store the deleted entry before filtering
      const allEntries =
        queryClient.getQueryData<EntryInfo[]>([USE_STREAMED_COLLECTION_ENTRIES_QUERY_KEY, variables.collectionId]) ||
        [];
      const deletedEntry = allEntries.find((entry) => entry.id === data.id);

      queryClient.setQueryData(
        [USE_STREAMED_COLLECTION_ENTRIES_QUERY_KEY, variables.collectionId],
        (old: EntryInfo[]) => {
          const deletedEntry = old.find((entry) => entry.id === data.id);
          if (!deletedEntry) {
            return old.filter((entry) => entry.id !== data.id);
          }

          return old.filter((entry) => {
            if (entry.id === data.id) {
              return false;
            }

            // Remove nested entries (entries that are children of the deleted entry)
            if (entry.path.segments.length > deletedEntry.path.segments.length) {
              const isNested = deletedEntry.path.segments.every(
                (segment, index) => entry.path.segments[index] === segment
              );

              if (isNested) {
                return false;
              }
            }

            return true;
          });
        }
      );

      // Find peer entries - entries at the same level with the same parent path
      if (deletedEntry) {
        // Get updated entries from query cache
        const updatedEntries =
          queryClient.getQueryData<EntryInfo[]>([USE_STREAMED_COLLECTION_ENTRIES_QUERY_KEY, variables.collectionId]) ||
          [];

        // Get the parent path (all segments except the last one which is the entry name)
        const parentPathSegments = deletedEntry.path.segments.slice(0, -1);

        // Find all peer entries (same parent path, same depth)
        const peerEntries = updatedEntries.filter((entry) => {
          // Check if entry is at the same level (same number of path segments)
          if (entry.path.segments.length !== deletedEntry.path.segments.length) return false;

          // Check if all parent path segments are the same
          const entryParentSegments = entry.path.segments.slice(0, -1);
          return (
            entryParentSegments.length === parentPathSegments.length &&
            entryParentSegments.every((segment, index) => segment === parentPathSegments[index])
          );
        });

        console.log({ peerEntries });

        // Update order for peer entries that come after the deleted entry
        const entriesToUpdate = peerEntries.filter((entry) => (entry.order ?? 0) > (deletedEntry.order ?? 0));

        if (entriesToUpdate.length > 0) {
          const updateEntries = await Promise.all(
            entriesToUpdate.map(async (entry) => {
              if (entry.kind === "Item") {
                return {
                  ITEM: {
                    id: entry.id,
                    order: Math.max(0, (entry.order ?? 0) - 1),
                  },
                };
              } else {
                return {
                  DIR: {
                    id: entry.id,
                    order: Math.max(0, (entry.order ?? 0) - 1),
                  },
                };
              }
            })
          );

          if (updateEntries.length > 0) {
            await batchUpdateCollectionEntry({
              collectionId: variables.collectionId,
              entries: {
                entries: updateEntries,
              },
            });
          }
        }
      }
    },
  });
};

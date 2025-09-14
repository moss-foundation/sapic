import { invokeTauriIpc } from "@/lib/backend/tauri";
import { useTabbedPaneStore } from "@/store/tabbedPane";
import { DeleteEntryInput, DeleteEntryOutput, StreamEntriesEvent } from "@repo/moss-project";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { USE_STREAM_COLLECTION_ENTRIES_QUERY_KEY } from "./useStreamCollectionEntries";

export interface UseDeleteCollectionEntryInput {
  collectionId: string;
  input: DeleteEntryInput;
}

const deleteCollectionEntry = async ({ collectionId, input }: UseDeleteCollectionEntryInput) => {
  const result = await invokeTauriIpc<DeleteEntryOutput>("delete_project_resource", { collectionId, input });

  if (result.status === "error") {
    throw new Error(String(result.error));
  }

  return result.data;
};

export const useDeleteCollectionEntry = () => {
  const queryClient = useQueryClient();
  const { api } = useTabbedPaneStore();

  return useMutation<DeleteEntryOutput, Error, UseDeleteCollectionEntryInput>({
    mutationFn: deleteCollectionEntry,
    onSuccess: async (data, variables) => {
      queryClient.setQueryData(
        [USE_STREAM_COLLECTION_ENTRIES_QUERY_KEY, variables.collectionId],
        (old: StreamEntriesEvent[]) => {
          const deletedEntry = old.find((entry) => entry.id === data.id);

          if (!deletedEntry) {
            return old;
          }

          return old.filter((entry) => {
            const panel = api?.getPanel(entry.id);

            if (entry.id === data.id) {
              if (panel) {
                api?.removePanel(panel);
              }
              return false;
            }

            if (entry.path.segments.length > deletedEntry.path.segments.length) {
              const isNested = deletedEntry.path.segments.every(
                (segment, index) => entry.path.segments[index] === segment
              );

              if (isNested) {
                if (panel) {
                  api?.removePanel(panel);
                }

                return false;
              }
            }

            return true;
          });
        }
      );
    },
  });
};

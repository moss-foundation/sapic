import { invokeTauriIpc } from "@/lib/backend/tauri";
import { StreamResourcesEvent, UpdateResourceInput, UpdateResourceOutput } from "@repo/moss-project";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { USE_DESCRIBE_PROJECT_ENTRY_QUERY_KEY } from "./useDescribeProjectEntry";
import { USE_STREAM_PROJECT_ENTRIES_QUERY_KEY } from "./useStreamProjectEntries";

export interface UseUpdateProjectEntryInput {
  projectId: string;
  updatedEntry: UpdateResourceInput;
}

const updateProjectEntry = async ({ projectId, updatedEntry }: UseUpdateProjectEntryInput) => {
  const result = await invokeTauriIpc<UpdateResourceOutput>("update_project_entry", {
    projectId: projectId,
    input: updatedEntry,
  });

  if (result.status === "error") {
    throw new Error(String(result.error));
  }

  return result.data;
};

export const useUpdateProjectEntry = () => {
  const queryClient = useQueryClient();

  return useMutation<UpdateResourceOutput, Error, UseUpdateProjectEntryInput>({
    mutationFn: updateProjectEntry,
    onSuccess: async (data, variables) => {
      queryClient.setQueryData(
        [USE_STREAM_PROJECT_ENTRIES_QUERY_KEY, variables.projectId],
        (old: StreamResourcesEvent[]) => {
          return old.map((oldEntry) => {
            const entryDataFromBackend = "ITEM" in data ? data.ITEM : data.DIR;
            const payloadEntryData =
              "ITEM" in variables.updatedEntry ? variables.updatedEntry.ITEM : variables.updatedEntry.DIR;

            if (oldEntry.id === entryDataFromBackend.id) {
              return {
                ...oldEntry,
                ...payloadEntryData,
                ...entryDataFromBackend,
              };
            }

            return oldEntry;
          });
        }
      );

      if ("ITEM" in data) {
        queryClient.invalidateQueries({
          queryKey: [USE_DESCRIBE_PROJECT_ENTRY_QUERY_KEY, variables.projectId, data.ITEM.id],
        });
      }

      if ("DIR" in data) {
        queryClient.invalidateQueries({
          queryKey: [USE_DESCRIBE_PROJECT_ENTRY_QUERY_KEY, variables.projectId, data.DIR.id],
        });
      }
    },
  });
};

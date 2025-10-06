import { invokeTauriIpc } from "@/lib/backend/tauri";
import { UpdateEntryInput, UpdateEntryOutput } from "@repo/moss-project";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { useFetchEntriesForPath } from "./derivedHooks/useFetchEntriesForPath";
import { USE_DESCRIBE_PROJECT_ENTRY_QUERY_KEY } from "./useDescribeProjectEntry";

export interface UseUpdateProjectEntryInput {
  projectId: string;
  updatedEntry: UpdateEntryInput;
}

const updateProjectEntry = async ({ projectId, updatedEntry }: UseUpdateProjectEntryInput) => {
  const result = await invokeTauriIpc<UpdateEntryOutput>("update_project_entry", {
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
  const { fetchEntriesForPath } = useFetchEntriesForPath();

  return useMutation<UpdateEntryOutput, Error, UseUpdateProjectEntryInput>({
    mutationFn: updateProjectEntry,
    onSuccess: async (data, variables) => {
      if ("ITEM" in data) {
        fetchEntriesForPath(variables.projectId, data.ITEM.path.raw);
        queryClient.invalidateQueries({
          queryKey: [USE_DESCRIBE_PROJECT_ENTRY_QUERY_KEY, variables.projectId, data.ITEM.id],
        });
      }

      if ("DIR" in data) {
        fetchEntriesForPath(variables.projectId, data.DIR.path.raw);
        queryClient.invalidateQueries({
          queryKey: [USE_DESCRIBE_PROJECT_ENTRY_QUERY_KEY, variables.projectId, data.DIR.id],
        });
      }
    },
  });
};

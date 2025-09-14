import { invokeTauriIpc } from "@/lib/backend/tauri";
import { CreateEntryInput, CreateEntryOutput, StreamEntriesEvent } from "@repo/moss-project";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { USE_STREAM_PROJECT_ENTRIES_QUERY_KEY } from "./useStreamProjectEntries";
import { createProjectEntryForCache } from "./utils";

export interface UseCreateProjectEntryInputProps {
  projectId: string;
  input: CreateEntryInput;
}

const createProjectEntry = async ({ projectId, input }: UseCreateProjectEntryInputProps) => {
  const result = await invokeTauriIpc<CreateEntryOutput>("create_project_entry", {
    collectionId: projectId,
    input,
  });

  if (result.status === "error") {
    throw new Error(String(result.error));
  }

  return result.data;
};

export const useCreateProjectEntry = () => {
  const queryClient = useQueryClient();

  return useMutation<CreateEntryOutput, Error, UseCreateProjectEntryInputProps>({
    mutationFn: createProjectEntry,
    onSuccess: async (data, variables) => {
      const newEntry = await createProjectEntryForCache(data.id, variables.input);

      queryClient.setQueryData(
        [USE_STREAM_PROJECT_ENTRIES_QUERY_KEY, variables.projectId],
        (old: StreamEntriesEvent[]) => {
          return [...old, newEntry];
        }
      );
    },
  });
};

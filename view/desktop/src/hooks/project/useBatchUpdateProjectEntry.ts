import { invokeTauriIpc, IpcResult } from "@/lib/backend/tauri";
import { BatchUpdateEntryInput, BatchUpdateEntryOutput, BatchUpdateEntryOutputKind } from "@repo/moss-project";
import { useMutation } from "@tanstack/react-query";
import { Channel } from "@tauri-apps/api/core";

export interface UseBatchUpdateProjectEntryInput {
  projectId: string;
  entries: BatchUpdateEntryInput;
}

const batchUpdateProjectEntry = async ({ projectId, entries }: UseBatchUpdateProjectEntryInput) => {
  const onProjectEvent = new Channel<BatchUpdateEntryOutputKind>();

  const result = await invokeTauriIpc<BatchUpdateEntryOutput>("batch_update_project_entry", {
    channel: onProjectEvent,
    collectionId: projectId,
    input: {
      entries: entries.entries,
    },
  });

  return result;
};

export const useBatchUpdateProjectEntry = () => {
  return useMutation<IpcResult<BatchUpdateEntryOutput, unknown>, Error, UseBatchUpdateProjectEntryInput>({
    mutationFn: batchUpdateProjectEntry,
  });
};

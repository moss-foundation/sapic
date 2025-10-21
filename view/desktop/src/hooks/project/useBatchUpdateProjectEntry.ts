import { invokeTauriIpc, IpcResult } from "@/lib/backend/tauri";
import { BatchUpdateResourceInput, BatchUpdateResourceOutput, BatchUpdateResourceOutputKind } from "@repo/moss-project";
import { useMutation } from "@tanstack/react-query";
import { Channel } from "@tauri-apps/api/core";

export interface UseBatchUpdateProjectEntryInput {
  projectId: string;
  entries: BatchUpdateResourceInput;
}

const batchUpdateProjectEntry = async ({ projectId, entries }: UseBatchUpdateProjectEntryInput) => {
  const onProjectEvent = new Channel<BatchUpdateResourceOutputKind>();

  const result = await invokeTauriIpc<BatchUpdateResourceOutput>("batch_update_project_entry", {
    channel: onProjectEvent,
    projectId: projectId,
    input: {
      entries: entries.resources,
    },
  });

  return result;
};

export const useBatchUpdateProjectEntry = () => {
  return useMutation<IpcResult<BatchUpdateResourceOutput, unknown>, Error, UseBatchUpdateProjectEntryInput>({
    mutationFn: batchUpdateProjectEntry,
  });
};

import { invokeTauriIpc, IpcResult } from "@/lib/backend/tauri";
import { BatchUpdateResourceInput, BatchUpdateResourceOutput, BatchUpdateResourceOutputKind } from "@repo/moss-project";
import { useMutation } from "@tanstack/react-query";
import { Channel } from "@tauri-apps/api/core";

export interface UseBatchUpdateProjectEntryInput {
  projectId: string;
  resources: BatchUpdateResourceInput;
}

const batchUpdateProjectEntry = async ({ projectId, resources }: UseBatchUpdateProjectEntryInput) => {
  const onProjectEvent = new Channel<BatchUpdateResourceOutputKind>();

  const result = await invokeTauriIpc<BatchUpdateResourceOutput>("batch_update_project_entry", {
    channel: onProjectEvent,
    projectId: projectId,
    input: {
      resources: resources.resources,
    },
  });

  return result;
};

export const useBatchUpdateProjectEntry = () => {
  return useMutation<IpcResult<BatchUpdateResourceOutput, unknown>, Error, UseBatchUpdateProjectEntryInput>({
    mutationFn: batchUpdateProjectEntry,
  });
};

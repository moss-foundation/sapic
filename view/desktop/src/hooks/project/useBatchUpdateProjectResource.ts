import { invokeTauriIpc, IpcResult } from "@/infra/ipc/tauri";
import { BatchUpdateResourceInput, BatchUpdateResourceOutput, BatchUpdateResourceOutputKind } from "@repo/moss-project";
import { useMutation } from "@tanstack/react-query";
import { Channel } from "@tauri-apps/api/core";

export interface UseBatchUpdateProjectResourceInput {
  projectId: string;
  resources: BatchUpdateResourceInput;
}

const batchUpdateProjectResource = async ({ projectId, resources }: UseBatchUpdateProjectResourceInput) => {
  const onProjectEvent = new Channel<BatchUpdateResourceOutputKind>();

  const result = await invokeTauriIpc<BatchUpdateResourceOutput>("batch_update_project_resource", {
    channel: onProjectEvent,
    projectId: projectId,
    input: {
      resources: resources.resources,
    },
  });

  return result;
};

export const useBatchUpdateProjectResource = () => {
  return useMutation<IpcResult<BatchUpdateResourceOutput, unknown>, Error, UseBatchUpdateProjectResourceInput>({
    mutationFn: batchUpdateProjectResource,
  });
};

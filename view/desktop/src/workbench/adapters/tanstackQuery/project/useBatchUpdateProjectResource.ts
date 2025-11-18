import { projectIpc } from "@/infra/ipc/project";
import { BatchUpdateResourceEvent, BatchUpdateResourceInput, BatchUpdateResourceOutput } from "@repo/moss-project";
import { useMutation } from "@tanstack/react-query";
import { Channel } from "@tauri-apps/api/core";

export interface UseBatchUpdateProjectResourceInput {
  projectId: string;
  resources: BatchUpdateResourceInput;
}

const batchUpdateProjectResource = async ({ projectId, resources }: UseBatchUpdateProjectResourceInput) => {
  const channelEvent = new Channel<BatchUpdateResourceEvent>();
  return await projectIpc.batchUpdateProjectResource(projectId, resources, channelEvent);
};

export const useBatchUpdateProjectResource = () => {
  return useMutation<BatchUpdateResourceOutput, Error, UseBatchUpdateProjectResourceInput>({
    mutationFn: batchUpdateProjectResource,
  });
};

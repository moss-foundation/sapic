import { invokeTauriIpc } from "@/infra/ipc/tauri";
import { BatchCreateResourceInput, BatchCreateResourceOutput } from "@repo/moss-project";
import { useMutation } from "@tanstack/react-query";

interface UseBatchCreateProjectResourceInput {
  projectId: string;
  input: BatchCreateResourceInput;
}

const batchCreateProjectResource = async ({ projectId, input }: UseBatchCreateProjectResourceInput) => {
  const result = await invokeTauriIpc<BatchCreateResourceOutput>("batch_create_project_resource", {
    projectId,
    input,
  });

  if (result.status === "error") {
    throw new Error(String(result.error));
  }

  return result.data;
};

export const useBatchCreateProjectResource = () => {
  return useMutation<BatchCreateResourceOutput, Error, UseBatchCreateProjectResourceInput>({
    mutationFn: batchCreateProjectResource,
  });
};

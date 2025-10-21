import { invokeTauriIpc } from "@/lib/backend/tauri";
import { BatchCreateResourceInput, BatchCreateResourceOutput } from "@repo/moss-project";
import { useMutation } from "@tanstack/react-query";

interface UseBatchCreateProjectEntryInput {
  projectId: string;
  input: BatchCreateResourceInput;
}

const batchCreateProjectEntry = async ({ projectId, input }: UseBatchCreateProjectEntryInput) => {
  const result = await invokeTauriIpc<BatchCreateResourceOutput>("batch_create_project_entry", {
    projectId,
    input,
  });

  if (result.status === "error") {
    throw new Error(String(result.error));
  }

  return result.data;
};

export const useBatchCreateProjectEntry = () => {
  return useMutation<BatchCreateResourceOutput, Error, UseBatchCreateProjectEntryInput>({
    mutationFn: batchCreateProjectEntry,
  });
};

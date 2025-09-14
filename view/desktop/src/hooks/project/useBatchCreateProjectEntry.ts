import { invokeTauriIpc } from "@/lib/backend/tauri";
import { BatchCreateEntryInput, BatchCreateEntryOutput } from "@repo/moss-project";
import { useMutation } from "@tanstack/react-query";

interface UseBatchCreateProjectEntryInput {
  projectId: string;
  input: BatchCreateEntryInput;
}

const batchCreateProjectEntry = async ({ projectId, input }: UseBatchCreateProjectEntryInput) => {
  const result = await invokeTauriIpc<BatchCreateEntryOutput>("batch_create_project_entry", {
    projectId,
    input,
  });

  if (result.status === "error") {
    throw new Error(String(result.error));
  }

  return result.data;
};

export const useBatchCreateProjectEntry = () => {
  return useMutation<BatchCreateEntryOutput, Error, UseBatchCreateProjectEntryInput>({
    mutationFn: batchCreateProjectEntry,
  });
};

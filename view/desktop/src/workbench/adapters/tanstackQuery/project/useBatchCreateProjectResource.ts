import { projectIpc } from "@/infra/ipc/project";
import { BatchCreateResourceInput, BatchCreateResourceOutput } from "@repo/moss-project";
import { useMutation } from "@tanstack/react-query";

interface UseBatchCreateProjectResourceInput {
  projectId: string;
  input: BatchCreateResourceInput;
}

export const useBatchCreateProjectResource = () => {
  return useMutation<BatchCreateResourceOutput, Error, UseBatchCreateProjectResourceInput>({
    mutationFn: ({ projectId, input }) => projectIpc.batchCreateProjectResource(projectId, input),
  });
};

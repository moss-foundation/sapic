import { projectService } from "@/domains/project/projectService";
import { BatchCreateResourceInput, BatchCreateResourceOutput } from "@repo/moss-project";
import { useMutation } from "@tanstack/react-query";

interface UseBatchCreateProjectResourceInput {
  projectId: string;
  input: BatchCreateResourceInput;
}

export const useBatchCreateProjectResource = () => {
  return useMutation<BatchCreateResourceOutput, Error, UseBatchCreateProjectResourceInput>({
    mutationFn: ({ projectId, input }) => projectService.batchCreateProjectResource(projectId, input),
  });
};

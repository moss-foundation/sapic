import { resourceService } from "@/domains/resource/resourceService";
import { CreateResourceInput, CreateResourceOutput } from "@repo/moss-project";
import { useMutation } from "@tanstack/react-query";

export interface UseCreateProjectResourceInputProps {
  projectId: string;
  input: CreateResourceInput;
}

export const useCreateProjectResource = () => {
  return useMutation<CreateResourceOutput, Error, UseCreateProjectResourceInputProps>({
    mutationFn: ({ projectId, input }) => resourceService.create(projectId, input),
  });
};

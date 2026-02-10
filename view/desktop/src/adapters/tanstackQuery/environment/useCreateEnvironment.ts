import { environmentService } from "@/domains/environment/environmentService";
import { CreateEnvironmentInput, CreateEnvironmentOutput } from "@repo/ipc";
import { useMutation } from "@tanstack/react-query";

export const useCreateEnvironment = () => {
  return useMutation<CreateEnvironmentOutput, Error, CreateEnvironmentInput>({
    mutationFn: (input) => environmentService.createEnvironment(input),
  });
};

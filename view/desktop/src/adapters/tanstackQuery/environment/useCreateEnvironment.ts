import { CreateEnvironmentParams, environmentService } from "@/domains/environment/environmentService";
import { CreateEnvironmentOutput } from "@repo/ipc";
import { useMutation } from "@tanstack/react-query";

export const useCreateEnvironment = () => {
  return useMutation<CreateEnvironmentOutput, Error, CreateEnvironmentParams>({
    mutationFn: (input) => environmentService.createEnvironment(input),
  });
};

import { environmentService } from "@/domains/environment/environmentService";
import { ActivateEnvironmentInput, ActivateEnvironmentOutput } from "@repo/ipc";
import { useMutation } from "@tanstack/react-query";

const ACTIVATE_ENVIRONMENT_QUERY_KEY = "activateEnvironment" as const;

export const useActivateEnvironment = () => {
  return useMutation<ActivateEnvironmentOutput, Error, ActivateEnvironmentInput>({
    mutationKey: [ACTIVATE_ENVIRONMENT_QUERY_KEY],
    mutationFn: (input) => environmentService.activateEnvironment(input),
  });
};

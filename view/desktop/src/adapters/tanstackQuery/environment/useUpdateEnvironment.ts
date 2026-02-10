import { environmentService } from "@/domains/environment/environmentService";
import { UpdateEnvironmentInput, UpdateEnvironmentOutput } from "@repo/ipc";
import { useMutation } from "@tanstack/react-query";

const UPDATE_ENVIRONMENT_QUERY_KEY = "updateEnvironment";

export const useUpdateEnvironment = () => {
  return useMutation<UpdateEnvironmentOutput, Error, UpdateEnvironmentInput>({
    mutationKey: [UPDATE_ENVIRONMENT_QUERY_KEY],
    mutationFn: (input) => environmentService.updateEnvironment(input),
  });
};

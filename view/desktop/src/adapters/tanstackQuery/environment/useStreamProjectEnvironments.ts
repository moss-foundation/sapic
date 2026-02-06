import { environmentService } from "@/domains/environment/environmentService";
import { StreamProjectEnvironmentsInput } from "@repo/ipc";
import { useQuery } from "@tanstack/react-query";

export const USE_STREAMED_PROJECT_ENVIRONMENTS_QUERY_KEY = "streamedProjectEnvironments";

export const useStreamProjectEnvironments = (input: StreamProjectEnvironmentsInput) => {
  return useQuery({
    queryKey: [USE_STREAMED_PROJECT_ENVIRONMENTS_QUERY_KEY, input.projectId],
    queryFn: () => environmentService.streamProjectEnvironments(input),
    placeholderData: [],
  });
};

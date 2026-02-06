import { environmentService } from "@/domains/environment/environmentService";
import { useQuery } from "@tanstack/react-query";

export const USE_STREAMED_ENVIRONMENTS_QUERY_KEY = "streamedEnvironments";

export const useStreamEnvironments = () => {
  return useQuery({
    queryKey: [USE_STREAMED_ENVIRONMENTS_QUERY_KEY],
    queryFn: environmentService.streamEnvironments,
  });
};

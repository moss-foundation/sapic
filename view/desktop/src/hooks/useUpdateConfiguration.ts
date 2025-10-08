import { AppService } from "@/lib/services/app";
import { UpdateConfigurationInput } from "@repo/moss-app";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { USE_DESCRIBE_APP_QUERY_KEY } from "./app/useDescribeApp";

export const useUpdateConfiguration = () => {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (configuration: UpdateConfigurationInput) => {
      return AppService.updateConfiguration(configuration);
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: [USE_DESCRIBE_APP_QUERY_KEY] });
    },
  });
};

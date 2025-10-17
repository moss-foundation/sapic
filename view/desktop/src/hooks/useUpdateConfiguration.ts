import { AppService } from "@/lib/services";
import { DescribeAppOutput, UpdateConfigurationInput } from "@repo/moss-app";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { USE_DESCRIBE_APP_QUERY_KEY } from "./app/useDescribeApp";

export const useUpdateConfiguration = () => {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (configuration: UpdateConfigurationInput) => {
      return AppService.updateConfiguration(configuration);
    },
    onSuccess: (_, variables) => {
      queryClient.setQueryData<DescribeAppOutput>([USE_DESCRIBE_APP_QUERY_KEY], (old) => {
        const newKeys = old?.configuration.keys ?? [];

        if (!newKeys.includes(variables.key)) {
          newKeys.push(variables.key);
        }

        return {
          ...old,
          configuration: {
            ...old?.configuration,
            keys: newKeys,
            contents: {
              ...old?.configuration.contents,
              [variables.key]: variables.value,
            },
          },
        };
      });
    },
  });
};

import { DEBOUNCE_TIME } from "@/constants/tanstackConfig";
import { USE_DESCRIBE_WORKSPACE_STATE_QUERY_KEY } from "@/hooks/workspace/useDescribeWorkspaceState";
import { AppService } from "@/lib/services/app";
import { ActivitybarPartStateInfo, DescribeStateOutput } from "@repo/moss-workspace";
import { asyncDebounce } from "@tanstack/react-pacer/async-debouncer";
import { useMutation, useQueryClient } from "@tanstack/react-query";

export const USE_UPDATE_ACTIVITYBAR_PART_STATE_MUTATION_KEY = "updateActivitybarPartState";

const debouncedSetActivitybarPartState = asyncDebounce(
  async (activitybar: ActivitybarPartStateInfo) => {
    await AppService.updateActivitybarPartState(activitybar);
  },
  { wait: DEBOUNCE_TIME }
);

export const useUpdateActivitybarPartState = () => {
  const queryClient = useQueryClient();

  return useMutation<void, Error, ActivitybarPartStateInfo>({
    mutationKey: [USE_UPDATE_ACTIVITYBAR_PART_STATE_MUTATION_KEY],
    mutationFn: async (activitybar: ActivitybarPartStateInfo): Promise<void> => {
      await debouncedSetActivitybarPartState(activitybar);
    },
    onSuccess: (_, variables) => {
      queryClient.setQueryData<DescribeStateOutput>([USE_DESCRIBE_WORKSPACE_STATE_QUERY_KEY], (old) => {
        return {
          ...old,
          activitybar: variables,
        };
      });
    },
  });
};

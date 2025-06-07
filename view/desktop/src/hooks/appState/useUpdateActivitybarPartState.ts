import { DEBOUNCE_TIME } from "@/constants/tanstackConfig";
import { invokeTauriIpc } from "@/lib/backend/tauri";
import { ActivitybarPartStateInfo } from "@repo/moss-workspace";
import { asyncDebounce } from "@tanstack/react-pacer/async-debouncer";
import { useMutation, useQueryClient } from "@tanstack/react-query";
import { USE_DESCRIBE_WORKSPACE_STATE_QUERY_KEY } from "@/hooks/workspace/useDescribeWorkspaceState";

export const USE_UPDATE_ACTIVITYBAR_PART_STATE_MUTATION_KEY = "updateActivitybarPartState";

const debouncedSetActivitybarPartState = asyncDebounce(
  async (activitybar: ActivitybarPartStateInfo) => {
    await invokeTauriIpc("update_workspace_state", {
      input: { "updateActivitybarPartState": activitybar },
    });
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
    onSuccess: () => {
      // Invalidate workspace state to refetch latest data
      queryClient.invalidateQueries({ queryKey: [USE_DESCRIBE_WORKSPACE_STATE_QUERY_KEY] });
    },
  });
};

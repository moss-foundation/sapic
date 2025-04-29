import { DEBOUNCE_TIME } from "@/constants/tanstackConfig";
import { invokeTauriIpc } from "@/lib/backend/tauri";
import { SidebarPartState } from "@repo/moss-workspace";
import { asyncDebounce } from "@tanstack/react-pacer/async-debouncer";
import { useMutation } from "@tanstack/react-query";

export const USE_UPDATE_SIDEBAR_PART_STATE_MUTATION_KEY = "updateSidebarPartState";

const debouncedSetSidebarPartState = asyncDebounce(
  async (sidebar: SidebarPartState) => {
    await invokeTauriIpc("update_workspace_state", {
      input: { "updateSidebarPartState": sidebar },
    });
  },
  { wait: DEBOUNCE_TIME }
);

export const useUpdateSidebarPartState = () => {
  return useMutation<void, Error, SidebarPartState>({
    mutationKey: [USE_UPDATE_SIDEBAR_PART_STATE_MUTATION_KEY],
    mutationFn: debouncedSetSidebarPartState,
  });
};

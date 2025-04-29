import { invokeTauriIpc } from "@/lib/backend/tauri";
import { SidebarPartState } from "@repo/moss-workspace";
import { debounce } from "@tanstack/react-pacer/debouncer";
import { useMutation } from "@tanstack/react-query";

export const USE_UPDATE_SIDEBAR_PART_STATE_MUTATION_KEY = "updateSidebarPartState";

const debouncedSetSidebarPartState = debounce(
  async (sidebar: SidebarPartState) => {
    await invokeTauriIpc("update_workspace_state", {
      input: { "updateSidebarPartState": sidebar },
    });
  },
  { wait: 2000 }
);

export const setDebouncedSidebarPartState = async (sidebar: SidebarPartState) => {
  debouncedSetSidebarPartState(sidebar);
};

export const useUpdateSidebarPartState = () => {
  return useMutation<void, Error, SidebarPartState>({
    mutationKey: [USE_UPDATE_SIDEBAR_PART_STATE_MUTATION_KEY],
    mutationFn: setDebouncedSidebarPartState,
  });
};

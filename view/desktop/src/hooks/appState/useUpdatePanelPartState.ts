import { DEBOUNCE_TIME } from "@/constants/tanstackConfig";
import { invokeTauriIpc } from "@/lib/backend/tauri";
import { PanelPartState } from "@repo/moss-workspace";
import { asyncDebounce } from "@tanstack/react-pacer/async-debouncer";
import { useMutation } from "@tanstack/react-query";

export const USE_UPDATE_PANEL_PART_STATE_MUTATION_KEY = "updatePanelPartState";

const debouncedSetPanelPartState = asyncDebounce(
  async (panel: PanelPartState) => {
    await invokeTauriIpc("update_workspace_state", {
      input: { "updatePanelPartState": panel },
    });
  },
  { wait: DEBOUNCE_TIME }
);

export const useUpdatePanelPartState = () => {
  return useMutation<void, Error, PanelPartState>({
    mutationKey: [USE_UPDATE_PANEL_PART_STATE_MUTATION_KEY],
    mutationFn: debouncedSetPanelPartState,
  });
};

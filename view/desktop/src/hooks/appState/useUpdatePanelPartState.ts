import { DEBOUNCE_TIME } from "@/constants/tanstackConfig";
import { invokeTauriIpc } from "@/lib/backend/tauri";
import { PanelPartState } from "@repo/moss-workspace";
import { debounce } from "@tanstack/react-pacer/debouncer";
import { useMutation } from "@tanstack/react-query";

export const USE_UPDATE_PANEL_PART_STATE_MUTATION_KEY = "updatePanelPartState";

const debouncedSetPanelPartState = debounce(
  async (panel: PanelPartState) => {
    await invokeTauriIpc("update_workspace_state", {
      input: { "updatePanelPartState": panel },
    });
  },
  { wait: DEBOUNCE_TIME }
);

const setPanelPartStateWithDebounce = async (panel: PanelPartState) => {
  debouncedSetPanelPartState(panel);
};

export const useUpdatePanelPartState = () => {
  return useMutation<void, Error, PanelPartState>({
    mutationKey: [USE_UPDATE_PANEL_PART_STATE_MUTATION_KEY],
    mutationFn: setPanelPartStateWithDebounce,
  });
};

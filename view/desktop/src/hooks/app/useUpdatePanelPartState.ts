import { DEBOUNCE_TIME } from "@/constants/tanstackConfig";
import { AppService } from "@/lib/services";
import { PanelPartStateInfo } from "@repo/moss-workspace";
import { asyncDebounce } from "@tanstack/react-pacer/async-debouncer";
import { useMutation } from "@tanstack/react-query";

export const USE_UPDATE_PANEL_PART_STATE_MUTATION_KEY = "updatePanelPartState";

const debouncedSetPanelPartState = asyncDebounce(
  async (panel: PanelPartStateInfo) => {
    await AppService.updatePanelPartState(panel);
  },
  { wait: DEBOUNCE_TIME }
);

export const useUpdatePanelPartState = () => {
  return useMutation<void, Error, PanelPartStateInfo>({
    mutationKey: [USE_UPDATE_PANEL_PART_STATE_MUTATION_KEY],
    mutationFn: async (panel: PanelPartStateInfo): Promise<void> => {
      await debouncedSetPanelPartState(panel);
    },
  });
};

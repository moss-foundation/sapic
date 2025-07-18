import { DEBOUNCE_TIME } from "@/constants/tanstackConfig";
import { invokeTauriIpc } from "@/lib/backend/tauri";
import { SerializedDockview } from "@/lib/moss-tabs/src";
import { asyncDebounce } from "@tanstack/react-pacer/async-debouncer";
import { useMutation } from "@tanstack/react-query";

import { mapSerializedDockviewToEditorPartState } from "./utils";

export const USE_UPDATE_EDITOR_PART_STATE_MUTATION_KEY = "updateEditorPartState";

const debouncedSetEditorPartState = asyncDebounce(
  async (editor: SerializedDockview) => {
    await invokeTauriIpc("update_workspace_state", {
      input: { "updateEditorPartState": mapSerializedDockviewToEditorPartState(editor) },
    });
  },
  { wait: DEBOUNCE_TIME }
);

export const useUpdateEditorPartState = () => {
  return useMutation<void, Error, SerializedDockview>({
    mutationKey: [USE_UPDATE_EDITOR_PART_STATE_MUTATION_KEY],
    mutationFn: async (editor: SerializedDockview): Promise<void> => {
      await debouncedSetEditorPartState(editor);
    },
  });
};

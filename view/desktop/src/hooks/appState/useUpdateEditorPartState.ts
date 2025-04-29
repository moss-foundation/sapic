import { DEBOUNCE_TIME } from "@/constants/tanstackConfig";
import { invokeTauriIpc } from "@/lib/backend/tauri";
import { SerializedDockview } from "@/lib/moss-tabs/src";
import { EditorPartState } from "@repo/moss-workspace";
import { debounce } from "@tanstack/react-pacer/debouncer";
import { useMutation } from "@tanstack/react-query";

import { mapSerializedDockviewToEditorPartState } from "./utils";

export const USE_UPDATE_EDITOR_PART_STATE_MUTATION_KEY = "updateEditorPartState";

const debouncedSetEditorPartState = debounce(
  async (editor: EditorPartState) => {
    await invokeTauriIpc("update_workspace_state", {
      input: { "updateEditorPartState": editor },
    });
  },
  { wait: DEBOUNCE_TIME }
);

const setEditorPartStateWithDebounce = async (editor: SerializedDockview) => {
  debouncedSetEditorPartState(mapSerializedDockviewToEditorPartState(editor));
};

export const useUpdateEditorPartState = () => {
  return useMutation<void, Error, SerializedDockview>({
    mutationKey: [USE_UPDATE_EDITOR_PART_STATE_MUTATION_KEY],
    mutationFn: setEditorPartStateWithDebounce,
  });
};

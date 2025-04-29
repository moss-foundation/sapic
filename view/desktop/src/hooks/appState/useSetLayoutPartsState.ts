import { invokeTauriIpc } from "@/lib/backend/tauri";
import { SerializedDockview } from "@/lib/moss-tabs/src";
import { PanelPartState, SetLayoutPartsStateInput, SidebarPartState } from "@repo/moss-workspace";
import { debounce } from "@tanstack/react-pacer/debouncer";
import { useMutation } from "@tanstack/react-query";

import { mapEditorPartStateToSerializedDockview, mapSerializedDockviewToEditorPartState } from "./utils";

export const USE_SET_LAYOUT_PARTS_STATE_MUTATION_KEY = "setLayoutPartsState";

const debouncedSetLayoutPartsState = debounce(
  async (latestInput: SetLayoutPartsStateInput) => {
    await invokeTauriIpc("set_layout_parts_state", {
      input: {
        ...latestInput,
        editor: latestInput?.editor ? mapEditorPartStateToSerializedDockview(latestInput.editor) : undefined,
      },
    });
  },
  { wait: 2000 }
);

interface SetLayoutPartsStateProps {
  input: {
    editor?: SerializedDockview;
    sidebar?: SidebarPartState;
    panel?: PanelPartState;
  };
}

export const setDebouncedLayoutPartsState = async ({ input }: SetLayoutPartsStateProps) => {
  debouncedSetLayoutPartsState({
    editor: input.editor ? mapSerializedDockviewToEditorPartState(input.editor) : undefined,
    sidebar: input.sidebar,
    panel: input.panel,
  });
};

export const useSetLayoutPartsState = () => {
  return useMutation<
    void,
    Error,
    {
      input: {
        editor?: SerializedDockview;
        sidebar?: SidebarPartState;
        panel?: PanelPartState;
      };
    }
  >({
    mutationKey: [USE_SET_LAYOUT_PARTS_STATE_MUTATION_KEY],
    mutationFn: setDebouncedLayoutPartsState,
  });
};

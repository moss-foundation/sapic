import { invokeTauriIpc } from "@/lib/backend/tauri";
import { SerializedDockview } from "@/lib/moss-tabs/src";
import { SetLayoutPartsStateParams } from "@repo/moss-workspace";

interface SetLayoutPartsStateProps {
  input: {
    editor?: SerializedDockview;
    sidebar?: {
      preferredSize: number;
      isVisible: boolean;
    };
    panel?: {
      preferredSize: number;
      isVisible: boolean;
    };
  };
  params?: SetLayoutPartsStateParams;
}

export const setLayoutPartsState = ({ input, params }: SetLayoutPartsStateProps) => {
  console.log("setLayoutPartsState", input, params);
  invokeTauriIpc("set_layout_parts_state", {
    input,
    params: params ?? { isOnExit: false },
  });
};

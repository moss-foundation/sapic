import { invokeTauriIpc } from "@/lib/backend/tauri";
import { SerializedDockview } from "@/lib/moss-tabs/src";
import {
  DescribeLayoutPartsStateOutput,
  OpenWorkspaceInput,
  OpenWorkspaceOutput,
  SetLayoutPartsStateParams,
} from "@repo/moss-workspace";

export const describeLayoutPartsState = async () => {
  return await invokeTauriIpc<DescribeLayoutPartsStateOutput>("describe_layout_parts_state");
};

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
  invokeTauriIpc("set_layout_parts_state", {
    input,
    params: params ?? { isOnExit: false },
  });
};

export const openWorkspace = async (name: string) => {
  return await invokeTauriIpc<OpenWorkspaceInput, OpenWorkspaceOutput>("open_workspace", {
    input: { name },
  });
};

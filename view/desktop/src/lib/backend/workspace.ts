import { invokeTauriIpc } from "@/lib/backend/tauri";
import { SerializedDockview } from "@/lib/moss-tabs/src";
import { DescribeLayoutPartsStateOutput, OpenWorkspaceInput, OpenWorkspaceOutput } from "@repo/moss-workspace";

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
}

export const setLayoutPartsState = ({ input }: SetLayoutPartsStateProps) => {
  invokeTauriIpc("set_layout_parts_state", {
    input,
  });
};

export const openWorkspace = async (name: string) => {
  return await invokeTauriIpc<OpenWorkspaceInput, OpenWorkspaceOutput>("open_workspace", {
    input: { name },
  });
};

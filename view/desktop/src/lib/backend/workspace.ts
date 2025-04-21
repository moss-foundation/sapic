import { invokeTauriIpc } from "@/lib/backend/tauri";
import {
  DescribeLayoutPartsStateOutput,
  OpenWorkspaceInput,
  OpenWorkspaceOutput,
  SetLayoutPartsStateInput,
} from "@repo/moss-workspace";

import { mapEditorPartStateToSerializedDockview } from "./utils";

export const describeLayoutPartsState = async () => {
  const res = await invokeTauriIpc<DescribeLayoutPartsStateOutput>("describe_layout_parts_state");

  if (res.status !== "ok") {
    console.error("Failed to describe layout parts state", res);
    return undefined;
  }

  return {
    editor: res.data?.editor ? mapEditorPartStateToSerializedDockview(res.data.editor) : undefined,
    sidebar: res.data?.sidebar,
    panel: res.data?.panel,
  };
};

export const setLayoutPartsState = ({ input }: { input: SetLayoutPartsStateInput }) => {
  invokeTauriIpc("set_layout_parts_state", {
    input: {
      ...input,
      editor: input?.editor ? mapEditorPartStateToSerializedDockview(input.editor) : undefined,
    },
  });
};

export const openWorkspace = async (name: string) => {
  return await invokeTauriIpc<OpenWorkspaceInput, OpenWorkspaceOutput>("open_workspace", {
    input: { name },
  });
};

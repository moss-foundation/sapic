import { invokeTauriIpc } from "@/lib/backend/tauri";
import { DescribeStateOutput } from "@repo/moss-workspace";
import { useQuery } from "@tanstack/react-query";

import { mapEditorPartStateToSerializedDockview } from "./utils";

export const USE_DESCRIBE_LAYOUT_PARTS_STATE_QUERY_KEY = "describeLayoutPartsState";

export const describeLayoutPartsState = async () => {
  const res = await invokeTauriIpc<DescribeStateOutput>("describe_workspace_state");

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

export const useDescribeLayoutPartsState = () => {
  return useQuery({
    queryKey: [USE_DESCRIBE_LAYOUT_PARTS_STATE_QUERY_KEY],
    queryFn: describeLayoutPartsState,
  });
};

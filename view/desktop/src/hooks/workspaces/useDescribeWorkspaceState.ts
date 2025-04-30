import { invokeTauriIpc } from "@/lib/backend/tauri";
import { useAllowDescribeWorkspaceStore } from "@/store/allowDescribeWorkspace";
import { DescribeStateOutput } from "@repo/moss-workspace";
import { useQuery } from "@tanstack/react-query";

import { mapEditorPartStateToSerializedDockview } from "../appState/utils";

export const USE_DESCRIBE_WORKSPACE_STATE_QUERY_KEY = "describeLayoutPartsState";

export const describeWorkspaceState = async () => {
  const res = await invokeTauriIpc<DescribeStateOutput>("describe_workspace_state");

  if (res.status !== "ok") {
    console.warn("Failed to describe layout parts state", res);
    return null;
  }

  return {
    editor: res.data?.editor ? mapEditorPartStateToSerializedDockview(res.data.editor) : undefined,
    sidebar: res.data?.sidebar,
    panel: res.data?.panel,
  };
};

export const useDescribeWorkspaceState = () => {
  const { allow } = useAllowDescribeWorkspaceStore();

  return useQuery({
    queryKey: [USE_DESCRIBE_WORKSPACE_STATE_QUERY_KEY],
    queryFn: describeWorkspaceState,
    enabled: allow,
  });
};

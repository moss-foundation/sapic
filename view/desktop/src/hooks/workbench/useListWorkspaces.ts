import { invokeTauriIpc } from "@/lib/backend/tauri";
import { ListWorkspacesOutput } from "@repo/moss-app";
import { useQuery } from "@tanstack/react-query";

export const USE_LIST_WORKSPACES_QUERY_KEY = "listWorkspaces";

const listWorkspacesFn = async (): Promise<ListWorkspacesOutput> => {
  const result = await invokeTauriIpc<ListWorkspacesOutput>("list_workspaces");

  if (result.status === "error") {
    throw new Error(String(result.error));
  }

  return result.data;
};

export const useListWorkspaces = () => {
  return useQuery<ListWorkspacesOutput, Error>({
    queryKey: [USE_LIST_WORKSPACES_QUERY_KEY],
    queryFn: listWorkspacesFn,
    placeholderData: [],
    staleTime: 10 * 60 * 1000, // 10 minutes
    gcTime: 30 * 60 * 1000, // 30 minutes
  });
};

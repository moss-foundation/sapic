import { invokeTauriIpc } from "@/lib/backend/tauri";
import { ListWorkspacesOutput } from "@repo/moss-workbench";
import { useQuery } from "@tanstack/react-query";

export const USE_GET_WORKSPACE_QUERY_KEY = "getWorkspace";

const getWorkspaceFn = async (): Promise<ListWorkspacesOutput> => {
  const result = await invokeTauriIpc<ListWorkspacesOutput>("list_workspaces");

  if (result.status === "error") {
    throw new Error(String(result.error));
  }

  return result.data;
};

export const useGetWorkspaces = () => {
  return useQuery<ListWorkspacesOutput, Error>({
    queryKey: [USE_GET_WORKSPACE_QUERY_KEY],
    queryFn: getWorkspaceFn,
  });
};

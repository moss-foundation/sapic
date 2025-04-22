import { invokeTauriIpc } from "@/lib/backend/tauri";
import { OpenWorkspaceInput, OpenWorkspaceOutput } from "@repo/moss-workspace";
import { useQuery } from "@tanstack/react-query";

export const USE_OPEN_WORKSPACE_QUERY_KEY = "openWorkspace";

const openWorkspaceFn = async (name: string): Promise<OpenWorkspaceOutput> => {
  const result = await invokeTauriIpc<OpenWorkspaceOutput, OpenWorkspaceInput>("open_workspace", {
    input: { name },
  });
  if (result.status === "error") {
    throw new Error(String(result.error));
  }
  return result.data;
};

export const useOpenWorkspace = (name: string) => {
  return useQuery<OpenWorkspaceOutput, Error>({
    queryKey: [USE_OPEN_WORKSPACE_QUERY_KEY, name],
    queryFn: () => openWorkspaceFn(name),
  });
};

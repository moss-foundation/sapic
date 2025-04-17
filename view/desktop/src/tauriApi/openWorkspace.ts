import { invokeTauriIpc } from "@/lib/backend/tauri";
import { OpenWorkspaceInput, OpenWorkspaceOutput } from "@repo/moss-workspace";

export const openWorkspace = async (name: string) => {
  return await invokeTauriIpc<OpenWorkspaceInput, OpenWorkspaceOutput>("open_workspace", {
    input: { name },
  });
};

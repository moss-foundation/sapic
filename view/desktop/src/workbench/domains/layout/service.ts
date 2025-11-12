import { sharedStorageIpc } from "@/infra/ipc/sharedStorage";
import { UpdateLayoutInput } from "@repo/moss-workspace";

const SHARED_STORAGE_LAYOUT_KEY = "workbench.layout" as const;

export const layoutService = {
  getLayout(workspaceId: string) {
    return sharedStorageIpc.getItem(SHARED_STORAGE_LAYOUT_KEY, {
      workspace: workspaceId,
    });
  },
  updateLayout(input: UpdateLayoutInput, workspaceId: string) {
    return sharedStorageIpc.putItem(SHARED_STORAGE_LAYOUT_KEY, input, {
      workspace: workspaceId,
    });
  },
  removeLayout(workspaceId: string) {
    return sharedStorageIpc.removeItem(SHARED_STORAGE_LAYOUT_KEY, {
      workspace: workspaceId,
    });
  },
};

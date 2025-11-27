import { ISharedStorageIpc } from "@/shared/sharedStorage/ipc";
import { invoke } from "@tauri-apps/api/core";

export const sharedStorageIpc: ISharedStorageIpc = {
  getItem: async (key, scope) => {
    return await invoke("plugin:shared-storage|get_item", {
      input: {
        key,
        scope,
      },
    });
  },
  putItem: async (key, value, scope) => {
    return await invoke("plugin:shared-storage|put_item", {
      input: {
        key,
        scope,
        value,
      },
    });
  },
  removeItem: async (key, scope) => {
    return await invoke("plugin:shared-storage|remove_item", {
      input: {
        key,
        scope,
      },
    });
  },
};

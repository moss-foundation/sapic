import { ISharedStorageIpc } from "@/shared/sharedStorage/ipc";
import { invoke } from "@tauri-apps/api/core";

export const sharedStorageIpc: ISharedStorageIpc = {
  getItem: async (key, scope) => {
    return await invoke("plugin:shared-storage|get_item", {
      input: { key, scope },
    });
  },
  putItem: async (key, value, scope) => {
    return await invoke("plugin:shared-storage|put_item", {
      input: { key, value, scope },
    });
  },
  removeItem: async (key, scope) => {
    return await invoke("plugin:shared-storage|remove_item", {
      input: { key, scope },
    });
  },

  batchPutItem: async (items, scope) => {
    return await invoke("plugin:shared-storage|batch_put_item", {
      input: { items, scope },
    });
  },
  batchRemoveItem: async (keys, scope) => {
    return await invoke("plugin:shared-storage|batch_remove_item", {
      input: { keys, scope },
    });
  },
  batchGetItem: async (keys, scope) => {
    return await invoke("plugin:shared-storage|batch_get_item", {
      input: { keys, scope },
    });
  },
  batchGetItemByPrefix: async (prefix, scope) => {
    return await invoke("plugin:shared-storage|batch_get_item_by_prefix", {
      input: { prefix, scope },
    });
  },
  batchRemoveItemByPrefix: async (prefix, scope) => {
    return await invoke("plugin:shared-storage|batch_remove_item_by_prefix", {
      input: { prefix, scope },
    });
  },
};

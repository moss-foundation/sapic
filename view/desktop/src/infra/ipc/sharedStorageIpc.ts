import { ISharedStorageIpc } from "@/shared/sharedStorage/ipc";

import { invokeTauriServiceIpc } from "./tauri";

export const sharedStorageIpc: ISharedStorageIpc = {
  getItem: async (key, scope) => {
    return await invokeTauriServiceIpc("plugin:shared-storage|get_item", {
      input: { key, scope },
    });
  },
  putItem: async (key, value, scope) => {
    return await invokeTauriServiceIpc("plugin:shared-storage|put_item", {
      input: { key, value, scope },
    });
  },
  removeItem: async (key, scope) => {
    return await invokeTauriServiceIpc("plugin:shared-storage|remove_item", {
      input: { key, scope },
    });
  },

  batchPutItem: async (items, scope) => {
    return await invokeTauriServiceIpc("plugin:shared-storage|batch_put_item", {
      input: { items, scope },
    });
  },
  batchRemoveItem: async (keys, scope) => {
    return await invokeTauriServiceIpc("plugin:shared-storage|batch_remove_item", {
      input: { keys, scope },
    });
  },
  batchGetItem: async (keys, scope) => {
    return await invokeTauriServiceIpc("plugin:shared-storage|batch_get_item", {
      input: { keys, scope },
    });
  },
  batchGetItemByPrefix: async (prefix, scope) => {
    return await invokeTauriServiceIpc("plugin:shared-storage|batch_get_item_by_prefix", {
      input: { prefix, scope },
    });
  },
  batchRemoveItemByPrefix: async (prefix, scope) => {
    return await invokeTauriServiceIpc("plugin:shared-storage|batch_remove_item_by_prefix", {
      input: { prefix, scope },
    });
  },
};

import { ISettingsStorageIpc } from "@/shared/settingsStorage/ipc";
import { invoke } from "@tauri-apps/api/core";

export const settingsStorageIpc: ISettingsStorageIpc = {
  getValue: async (key, scope) => {
    return await invoke("plugin:settings-storage|get_value", {
      input: {
        key,
        scope,
      },
    });
  },
  batchGetValue: async (keys, scope) => {
    return await invoke("plugin:settings-storage|batch_get_value", {
      input: {
        keys,
        scope,
      },
    });
  },
  updateValue: async (key, value, scope) => {
    return await invoke("plugin:settings-storage|update_value", {
      input: {
        key,
        value,
        scope,
      },
    });
  },
  removeValue: async (key, scope) => {
    return await invoke("plugin:settings-storage|remove_value", {
      input: {
        key,
        scope,
      },
    });
  },
};

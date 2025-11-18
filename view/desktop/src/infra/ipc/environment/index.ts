import { IEnvironmentIpc } from "@/domains/environment";
import { StreamEnvironmentsEvent } from "@repo/moss-workspace";
import { Channel, invoke } from "@tauri-apps/api/core";

export const environmentIpc: IEnvironmentIpc = {
  activateEnvironment: async (input) => {
    return await invoke("activate_environment", { input });
  },

  batchUpdateEnvironment: async (input) => {
    return await invoke("batch_update_environment", { input });
  },

  batchUpdateEnvironmentGroup: async (input) => {
    return await invoke("batch_update_environment_group", { input });
  },

  createEnvironment: async (input) => {
    return await invoke("create_environment", { input });
  },

  deleteEnvironment: async (input) => {
    return await invoke("delete_environment", { input });
  },

  streamEnvironments: async (channelEvent: Channel<StreamEnvironmentsEvent>) => {
    return await invoke("stream_environments", { channel: channelEvent });
  },

  updateEnvironment: async (input) => {
    return await invoke("update_environment", { input });
  },

  updateEnvironmentGroup: async (input) => {
    return await invoke("update_environment_group", { input });
  },
};

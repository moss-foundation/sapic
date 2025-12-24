import { IResourceIpc } from "@/domains/resource";
import { invoke } from "@tauri-apps/api/core";

export const resourceIpc: IResourceIpc = {
  batchCreate: async (projectId, input) => {
    return await invoke("batch_create_project_resource", { projectId, input });
  },

  batchUpdate: async (projectId, input, channelEvent) => {
    return await invoke("batch_update_project_resource", { projectId, input, channel: channelEvent });
  },

  create: async (projectId, input) => {
    return await invoke("create_project_resource", { projectId, input });
  },

  delete: async (projectId, input) => {
    return await invoke("delete_project_resource", { projectId, input });
  },

  describe: async (projectId, resourceId) => {
    return await invoke("describe_project_resource", { projectId, resourceId });
  },

  stream: async (projectId, channelEvent, path) => {
    return await invoke("stream_project_resources", {
      projectId,
      channel: channelEvent,
      input: path ? { "RELOAD_PATH": path } : "LOAD_ROOT",
    });
  },

  update: async (projectId, input) => {
    return await invoke("update_project_resource", { projectId, input });
  },
};

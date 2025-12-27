import { IProjectIpc } from "@/domains/project";
import { invoke } from "@tauri-apps/api/core";

export const projectIpc: IProjectIpc = {
  batchUpdateProject: async (input) => {
    return await invoke("batch_update_project", { input });
  },

  createProject: async (input) => {
    return await invoke("create_project", { input });
  },

  deleteProject: async (input) => {
    return await invoke("delete_project", { input });
  },

  importProject: async (input) => {
    return await invoke("import_project", { input });
  },

  streamProjects: async (channelEvent) => {
    return await invoke("stream_projects", {
      channel: channelEvent,
    });
  },

  updateProject: async (input) => {
    return await invoke("update_project", { input });
  },
};

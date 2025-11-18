import { IProjectIpc } from "@/domains/project/ipc";
import { StreamResourcesEvent } from "@repo/moss-project";
import { StreamProjectsEvent } from "@repo/moss-workspace";
import { Channel, invoke } from "@tauri-apps/api/core";

export const projectIpc: IProjectIpc = {
  batchCreateProjectResource: async (projectId, input) => {
    return await invoke("batch_create_project_resource", { projectId, input });
  },

  batchUpdateProject: async (input) => {
    return await invoke("batch_update_project", { input });
  },

  batchUpdateProjectResource: async (projectId, input, channelEvent) => {
    return await invoke("batch_update_project_resource", { projectId, input, channel: channelEvent });
  },

  createProject: async (input) => {
    return await invoke("create_project", { input });
  },

  createProjectResource: async (projectId, input) => {
    return await invoke("create_project_resource", { projectId, input });
  },

  deleteProject: async (input) => {
    return await invoke("delete_project", { input });
  },

  deleteProjectResource: async (projectId, input) => {
    return await invoke("delete_project_resource", { projectId, input });
  },

  describeProjectResource: async (projectId, resourceId) => {
    return await invoke("describe_project_resource", { projectId, resourceId });
  },

  importProject: async (input) => {
    return await invoke("import_project", { input });
  },

  streamProjects: async (channelEvent: Channel<StreamProjectsEvent>) => {
    return await invoke("stream_projects", {
      channel: channelEvent,
    });
  },

  streamProjectResources: async (projectId, channelEvent: Channel<StreamResourcesEvent>, path?: string) => {
    return await invoke("stream_project_resources", {
      projectId,
      channel: channelEvent,
      input: path ? { "RELOAD_PATH": path } : "LOAD_ROOT",
    });
  },

  updateProject: async (input) => {
    return await invoke("update_project", { input });
  },

  updateProjectResource: async (projectId, input) => {
    return await invoke("update_project_resource", { projectId, input });
  },
};

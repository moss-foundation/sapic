import { IResourceIpc } from "@/domains/resource";

import { invokeTauriIpc } from "./tauri";

export const resourceIpc: IResourceIpc = {
  batchCreate: async (projectId, input) => {
    return await invokeTauriIpc("batch_create_project_resource", { projectId, input });
  },

  batchUpdate: async (projectId, input, channelEvent) => {
    return await invokeTauriIpc("batch_update_project_resource", { projectId, input, channel: channelEvent });
  },

  create: async (projectId, input) => {
    return await invokeTauriIpc("create_project_resource", { projectId, input });
  },

  delete: async (projectId, input) => {
    return await invokeTauriIpc("delete_project_resource", { projectId, input });
  },

  describe: async (projectId, resourceId) => {
    return await invokeTauriIpc("describe_project_resource", { projectId, resourceId });
  },

  list: async (input) => {
    return await invokeTauriIpc("main__list_project_resources", { input });
  },

  update: async (projectId, input) => {
    return await invokeTauriIpc("update_project_resource", { projectId, input });
  },
};

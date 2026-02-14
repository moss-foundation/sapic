import { IResourceIpc } from "@/domains/resource";

import { invokeTauriServiceIpc } from "./tauri";

export const resourceIpc: IResourceIpc = {
  batchCreate: async (projectId, input) => {
    return await invokeTauriServiceIpc("batch_create_project_resource", { projectId, input });
  },

  batchUpdate: async (projectId, input, channelEvent) => {
    return await invokeTauriServiceIpc("batch_update_project_resource", { projectId, input, channel: channelEvent });
  },

  create: async (projectId, input) => {
    return await invokeTauriServiceIpc("create_project_resource", { projectId, input });
  },

  delete: async (projectId, input) => {
    return await invokeTauriServiceIpc("delete_project_resource", { projectId, input });
  },

  describe: async (projectId, resourceId) => {
    return await invokeTauriServiceIpc("describe_project_resource", { projectId, resourceId });
  },

  list: async (input) => {
    return await invokeTauriServiceIpc("main__list_project_resources", { input });
  },

  update: async (projectId, input) => {
    return await invokeTauriServiceIpc("update_project_resource", { projectId, input });
  },
};

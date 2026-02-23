import { IProjectIpc } from "@/domains/project";

import { invokeTauriIpc } from "./tauri";

export const projectIpc: IProjectIpc = {
  batchUpdateProject: async (input) => {
    return await invokeTauriIpc("batch_update_project", { input });
  },

  createProject: async (input) => {
    return await invokeTauriIpc("create_project", { input });
  },

  deleteProject: async (input) => {
    return await invokeTauriIpc("delete_project", { input });
  },

  importProject: async (input) => {
    return await invokeTauriIpc("import_project", { input });
  },

  listProjects: async () => {
    return await invokeTauriIpc("main__list_projects");
  },

  updateProject: async (input) => {
    return await invokeTauriIpc("update_project", { input });
  },
};

import { IProjectIpc } from "@/domains/project";

import { invokeTauriServiceIpc } from "./tauri";

export const projectIpc: IProjectIpc = {
  batchUpdateProject: async (input) => {
    return await invokeTauriServiceIpc("batch_update_project", { input });
  },

  createProject: async (input) => {
    return await invokeTauriServiceIpc("create_project", { input });
  },

  deleteProject: async (input) => {
    return await invokeTauriServiceIpc("delete_project", { input });
  },

  importProject: async (input) => {
    return await invokeTauriServiceIpc("import_project", { input });
  },

  listProjects: async () => {
    return await invokeTauriServiceIpc("main__list_projects");
  },

  updateProject: async (input) => {
    return await invokeTauriServiceIpc("update_project", { input });
  },
};

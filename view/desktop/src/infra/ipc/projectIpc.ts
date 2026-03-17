import { IProjectIpc } from "@/domains/project";

import { invokeTauriIpc } from "./tauri";

export const projectIpc: IProjectIpc = {
  batchUpdate: async (input) => {
    return await invokeTauriIpc("batch_update_project", { input });
  },

  create: async (input) => {
    return await invokeTauriIpc("create_project", { input });
  },

  delete: async (input) => {
    return await invokeTauriIpc("delete_project", { input });
  },

  import: async (input) => {
    return await invokeTauriIpc("import_project", { input });
  },

  list: async () => {
    return await invokeTauriIpc("main__list_projects");
  },

  update: async (input) => {
    return await invokeTauriIpc("update_project", { input });
  },
};

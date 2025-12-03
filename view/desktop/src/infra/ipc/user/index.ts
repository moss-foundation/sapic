import { IUserIpc } from "@/domains/user";
import { invoke } from "@tauri-apps/api/core";

export const userIpc: IUserIpc = {
  // DEPRECATED
  updateProfile: async (input) => {
    return await invoke("update_profile", { input });
  },

  listUserAccounts: async () => {
    return await invoke("list_user_accounts");
  },

  addUserAccount: async (input) => {
    return await invoke("add_user_account", { input });
  },

  updateUserAccount: async (input) => {
    return await invoke("update_user_account", { input });
  },

  removeUserAccount: async (input) => {
    return await invoke("remove_user_account", { input });
  },
};

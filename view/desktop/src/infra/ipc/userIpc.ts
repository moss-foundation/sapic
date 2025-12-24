import { IUserIpc } from "@/domains/user";
import { invoke } from "@tauri-apps/api/core";

export const userIpc: IUserIpc = {
  listUserAccounts: async () => await invoke("list_user_accounts"),
  addUserAccount: async (input) => await invoke("add_user_account", { input }),
  updateUserAccount: async (input) => await invoke("update_user_account", { input }),
  removeUserAccount: async (input) => await invoke("remove_user_account", { input }),
};

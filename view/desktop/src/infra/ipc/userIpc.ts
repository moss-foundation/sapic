import { IUserIpc } from "@/domains/user";

import { invokeTauriServiceIpc } from "./tauri";

export const userIpc: IUserIpc = {
  listUserAccounts: async () => await invokeTauriServiceIpc("list_user_accounts"),
  addUserAccount: async (input) => await invokeTauriServiceIpc("add_user_account", { input }),
  updateUserAccount: async (input) => await invokeTauriServiceIpc("update_user_account", { input }),
  removeUserAccount: async (input) => await invokeTauriServiceIpc("remove_user_account", { input }),
};

import { IUserIpc } from "@/domains/user";

import { invokeTauriIpc } from "./tauri";

export const userIpc: IUserIpc = {
  listUserAccounts: async () => await invokeTauriIpc("list_user_accounts"),
  addUserAccount: async (input) => await invokeTauriIpc("add_user_account", { input }),
  updateUserAccount: async (input) => await invokeTauriIpc("update_user_account", { input }),
  removeUserAccount: async (input) => await invokeTauriIpc("remove_user_account", { input }),
};

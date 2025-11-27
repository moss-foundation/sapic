import { IUserIpc } from "@/domains/user";
import { invoke } from "@tauri-apps/api/core";

export const userIpc: IUserIpc = {
  updateProfile: async (input) => {
    return await invoke("update_profile", { input });
  },
};

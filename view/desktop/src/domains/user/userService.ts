import { userIpc } from "@/infra/ipc/user";
import { UpdateProfileInput } from "@repo/window";

export const userService = {
  updateProfile: async (input: UpdateProfileInput) => {
    return await userIpc.updateProfile(input);
  },
};

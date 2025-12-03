import { userIpc } from "@/infra/ipc/user";
import { AddUserAccountInput, ListUserAccountsOutput, RemoveUserAccountInput, UpdateUserAccountInput } from "@repo/ipc";
import { UpdateProfileInput, UpdateProfileOutput } from "@repo/window";

export interface IUserService {
  // DEPRECATED
  updateProfile: (input: UpdateProfileInput) => Promise<UpdateProfileOutput>;

  listUserAccounts: () => Promise<ListUserAccountsOutput>;
  addUserAccount: (input: AddUserAccountInput) => Promise<void>;
  updateUserAccount: (input: UpdateUserAccountInput) => Promise<void>;
  removeUserAccount: (input: RemoveUserAccountInput) => Promise<void>;
}

export const userService: IUserService = {
  updateProfile: async (input: UpdateProfileInput) => {
    return await userIpc.updateProfile(input);
  },

  listUserAccounts: async () => {
    return await userIpc.listUserAccounts();
  },
  addUserAccount: async (input: AddUserAccountInput) => {
    return await userIpc.addUserAccount(input);
  },
  updateUserAccount: async (input: UpdateUserAccountInput) => {
    return await userIpc.updateUserAccount(input);
  },
  removeUserAccount: async (input: RemoveUserAccountInput) => {
    return await userIpc.removeUserAccount(input);
  },
};

import { userIpc } from "@/infra/ipc/user";
import { AddUserAccountInput, ListUserAccountsOutput, RemoveUserAccountInput, UpdateUserAccountInput } from "@repo/ipc";

export interface IUserService {
  listUserAccounts: () => Promise<ListUserAccountsOutput>;
  addUserAccount: (input: AddUserAccountInput) => Promise<void>;
  updateUserAccount: (input: UpdateUserAccountInput) => Promise<void>;
  removeUserAccount: (input: RemoveUserAccountInput) => Promise<void>;
}

export const userService: IUserService = {
  listUserAccounts: async () => await userIpc.listUserAccounts(),
  addUserAccount: async (input: AddUserAccountInput) => await userIpc.addUserAccount(input),
  updateUserAccount: async (input: UpdateUserAccountInput) => await userIpc.updateUserAccount(input),
  removeUserAccount: async (input: RemoveUserAccountInput) => await userIpc.removeUserAccount(input),
};

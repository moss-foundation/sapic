import { AddUserAccountInput, ListUserAccountsOutput, RemoveUserAccountInput, UpdateUserAccountInput } from "@repo/ipc";

export interface IUserIpc {
  listUserAccounts: () => Promise<ListUserAccountsOutput>;
  addUserAccount: (input: AddUserAccountInput) => Promise<void>;
  updateUserAccount: (input: UpdateUserAccountInput) => Promise<void>;
  removeUserAccount: (input: RemoveUserAccountInput) => Promise<void>;
}

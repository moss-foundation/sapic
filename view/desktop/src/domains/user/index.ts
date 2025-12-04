import { AddUserAccountInput, ListUserAccountsOutput, RemoveUserAccountInput, UpdateUserAccountInput } from "@repo/ipc";
import { UpdateProfileInput, UpdateProfileOutput } from "@repo/window";

export interface IUserIpc {
  // DEPRECATED
  updateProfile: (input: UpdateProfileInput) => Promise<UpdateProfileOutput>;

  listUserAccounts: () => Promise<ListUserAccountsOutput>;
  addUserAccount: (input: AddUserAccountInput) => Promise<void>;
  updateUserAccount: (input: UpdateUserAccountInput) => Promise<void>;
  removeUserAccount: (input: RemoveUserAccountInput) => Promise<void>;
}

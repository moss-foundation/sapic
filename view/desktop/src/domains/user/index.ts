import { UpdateProfileInput, UpdateProfileOutput } from "@repo/window";

export interface IUserIpc {
  updateProfile: (input: UpdateProfileInput) => Promise<UpdateProfileOutput>;
}

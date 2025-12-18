import { userService } from "@/domains/user/userService";
import { UpdateUserAccountInput } from "@repo/ipc";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { USE_LIST_USER_ACCOUNTS_QUERY_KEY } from "./useListUserAccounts";

export const USE_UPDATE_USER_ACCOUNT_QUERY_KEY = "updateUserAccount";

export const useUpdateUserAccount = () => {
  const queryClient = useQueryClient();
  return useMutation<void, Error, UpdateUserAccountInput>({
    mutationFn: userService.updateUserAccount,
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: [USE_LIST_USER_ACCOUNTS_QUERY_KEY] });
    },
  });
};

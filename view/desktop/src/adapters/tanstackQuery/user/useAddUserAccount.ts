import { userService } from "@/domains/user/userService";
import { AddUserAccountInput } from "@repo/ipc";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { USE_LIST_USER_ACCOUNTS_QUERY_KEY } from "./useListUserAccounts";

export const USE_ADD_USER_ACCOUNT_QUERY_KEY = "addUserAccount";

export const useAddUserAccount = () => {
  const queryClient = useQueryClient();
  return useMutation<void, Error, AddUserAccountInput>({
    mutationFn: userService.addUserAccount,
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: [USE_LIST_USER_ACCOUNTS_QUERY_KEY] });
    },
  });
};

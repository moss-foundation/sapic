import { userService } from "@/domains/user/userService";
import { ListUserAccountsOutput, RemoveUserAccountInput } from "@repo/ipc";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { USE_LIST_USER_ACCOUNTS_QUERY_KEY } from "./useListUserAccounts";

export const useRemoveUserAccount = () => {
  const queryClient = useQueryClient();
  return useMutation<void, Error, RemoveUserAccountInput>({
    mutationFn: userService.removeUserAccount,
    onSuccess: (_data, variables) => {
      queryClient.setQueryData([USE_LIST_USER_ACCOUNTS_QUERY_KEY], (old: ListUserAccountsOutput) => ({
        ...old,
        accounts: old.accounts.filter((account) => account.id !== variables.id),
      }));
    },
  });
};

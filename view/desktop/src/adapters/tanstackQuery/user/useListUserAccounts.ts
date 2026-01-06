import { userService } from "@/domains/user/userService";
import { ListUserAccountsOutput } from "@repo/ipc";
import { useQuery } from "@tanstack/react-query";

export const USE_LIST_USER_ACCOUNTS_QUERY_KEY = "listUserAccounts";

export const useListUserAccounts = () => {
  return useQuery<ListUserAccountsOutput, Error>({
    queryKey: [USE_LIST_USER_ACCOUNTS_QUERY_KEY],
    queryFn: userService.listUserAccounts,
  });
};

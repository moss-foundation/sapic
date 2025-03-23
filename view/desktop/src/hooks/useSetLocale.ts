import { invokeTauriIpc } from "@/lib/backend/tauri";
import { SetLocaleInput } from "@repo/moss-state";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { USE_DESCRIBE_APP_STATE_QUERY_KEY } from "./useDescribeAppState";

export const USE_SET_LOCALE_MUTATION_KEY = "setLocale";

const setLocaleFn = async (input: SetLocaleInput): Promise<void> => {
  const result = await invokeTauriIpc("set_locale", {
    input: input,
  });
  if (result.status === "error") {
    throw new Error(String(result.error));
  }
};

export const useSetLocale = () => {
  const queryClient = useQueryClient();
  return useMutation<void, Error, SetLocaleInput>({
    mutationKey: [USE_SET_LOCALE_MUTATION_KEY],
    mutationFn: setLocaleFn,
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: [USE_DESCRIBE_APP_STATE_QUERY_KEY] });
    },
  });
};

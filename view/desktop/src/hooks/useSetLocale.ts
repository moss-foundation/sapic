import { invokeTauriIpc } from "@/lib/backend/tauri";
import { SetLocaleInput } from "@repo/moss-state";
import { useMutation, useQueryClient } from "@tanstack/react-query";

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
    mutationKey: ["setLocale"],
    mutationFn: setLocaleFn,
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["getState"] });
    },
  });
};

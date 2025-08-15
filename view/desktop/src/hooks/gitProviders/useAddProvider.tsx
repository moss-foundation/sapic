import { invokeTauriIpc } from "@/lib/backend/tauri";
import { useGitProviderStore } from "@/store/gitProvider";
import { AddAccountInput, AddAccountOutput } from "@repo/moss-app";
import { useMutation } from "@tanstack/react-query";

export const ADD_PROVIDER_QUERY_KEY = "addProvider";

const addProvider = async (input: AddAccountInput) => {
  const result = await invokeTauriIpc<AddAccountOutput>("add_account", {
    input,
  });

  if (result.status === "error") {
    throw new Error(String(result.error));
  }

  return result.data;
};

export const useAddProvider = () => {
  const { setGitProvider } = useGitProviderStore();

  return useMutation({
    mutationKey: [ADD_PROVIDER_QUERY_KEY],
    mutationFn: addProvider,
    onSuccess: (data) => {
      setGitProvider(data);
    },
  });
};

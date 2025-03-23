import { invokeTauriIpc } from "@/lib/backend/tauri";
import { ListLocalesOutput } from "@repo/moss-nls";
import { useQuery } from "@tanstack/react-query";

const listLocalesFn = async (): Promise<ListLocalesOutput> => {
  const result = await invokeTauriIpc<ListLocalesOutput>("list_locales");
  if (result.status === "error") {
    throw new Error(String(result.error));
  }

  return result.data;
};

export const useListLocales = () => {
  return useQuery<ListLocalesOutput, Error>({
    queryKey: ["listLocales"],
    queryFn: listLocalesFn,
  });
};

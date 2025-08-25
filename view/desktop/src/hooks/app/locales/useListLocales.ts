import { invokeTauriIpc } from "@/lib/backend/tauri";
import { ListLocalesOutput } from "@repo/moss-app";
import { useQuery } from "@tanstack/react-query";

export const USE_LIST_LOCALES_QUERY_KEY = "listLocales";

const listLocalesFn = async (): Promise<ListLocalesOutput> => {
  const result = await invokeTauriIpc<ListLocalesOutput>("list_locales");
  if (result.status === "error") {
    throw new Error(String(result.error));
  }

  return result.data;
};

export const useListLocales = () => {
  return useQuery<ListLocalesOutput, Error>({
    queryKey: [USE_LIST_LOCALES_QUERY_KEY],
    queryFn: listLocalesFn,
  });
};

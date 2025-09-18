import { invokeTauriIpc } from "@/lib/backend/tauri";
import { GetLocaleInput, GetLocaleOutput } from "@repo/moss-app";
import { useQuery, UseQueryOptions } from "@tanstack/react-query";

export const USE_GET_LOCALE_QUERY_KEY = "getLocale";

export const getLocaleFn = async (identifier: string): Promise<GetLocaleOutput> => {
  const result = await invokeTauriIpc<GetLocaleOutput>("get_locale", { input: { identifier } });

  if (result.status === "error") {
    throw new Error(String(result.error));
  }

  return result.data;
};

interface UseGetLocaleOptions extends GetLocaleInput {
  identifier: string;
  options?: Omit<UseQueryOptions<GetLocaleOutput, Error>, "queryKey" | "queryFn">;
}

export const useGetLocale = ({ identifier, options }: UseGetLocaleOptions) => {
  return useQuery<GetLocaleOutput, Error>({
    queryKey: [USE_GET_LOCALE_QUERY_KEY],
    queryFn: () => getLocaleFn(identifier),
    ...options,
  });
};

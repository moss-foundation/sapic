import { invokeTauriIpc } from "@/lib/backend/tauri";
import { GetLocaleInput, GetLocaleOutput } from "@repo/moss-app";
import { useQuery } from "@tanstack/react-query";

export const USE_GET_LOCALE_QUERY_KEY = "getLocale";

export const getLocaleFn = async (identifier: string): Promise<GetLocaleOutput> => {
  const result = await invokeTauriIpc<GetLocaleOutput>("get_locale", { input: { identifier } });

  if (result.status === "error") {
    throw new Error(String(result.error));
  }

  return result.data;
};

export const useGetLocale = ({ identifier }: GetLocaleInput) => {
  const query = useQuery<GetLocaleOutput, Error>({
    queryKey: [USE_GET_LOCALE_QUERY_KEY],
    queryFn: () => getLocaleFn(identifier),
  });

  const getLocaleById = async (id: string) => {
    const result = await getLocaleFn(id);
    return result;
  };

  return { ...query, getLocaleById };
};

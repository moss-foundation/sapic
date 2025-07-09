import { invokeTauriIpc } from "@/lib/backend/tauri";
import { GetTranslationsInput, GetTranslationsOutput } from "@repo/moss-app";
import { useQuery } from "@tanstack/react-query";

export const USE_GET_TRANSLATIONS_QUERY_KEY = "getTranslations";

const getTranslationsFn = async (input: GetTranslationsInput): Promise<GetTranslationsOutput> => {
  const result = await invokeTauriIpc<GetTranslationsOutput>("get_translations", {
    input: input,
  });
  if (result.status === "error") {
    throw new Error(String(result.error));
  }

  return result.data;
};

export interface UseGetTranslationsParams {
  language: string;
  namespace: string;
  enabled?: boolean;
}

export const useGetTranslations = ({ language, namespace, enabled = true }: UseGetTranslationsParams) => {
  return useQuery<GetTranslationsOutput, Error>({
    queryKey: [USE_GET_TRANSLATIONS_QUERY_KEY, language, namespace],
    queryFn: () => getTranslationsFn({ language, namespace }),
    enabled: enabled && !!language && !!namespace,
  });
};

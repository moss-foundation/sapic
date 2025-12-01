import { AppService } from "@/lib/services";
import { ListLanguagesOutput } from "@repo/ipc";
import { useQuery } from "@tanstack/react-query";

export const USE_LIST_LANGUAGES_QUERY_KEY = "listLanguages";

const listLanguagesFn = async (): Promise<ListLanguagesOutput> => {
  const result = await AppService.listLanguages();

  if (result.status === "error") {
    throw new Error(String(result.error));
  }

  return result.data;
};

export const useListLanguages = () => {
  return useQuery<ListLanguagesOutput, Error>({
    queryKey: [USE_LIST_LANGUAGES_QUERY_KEY],
    queryFn: listLanguagesFn,
  });
};

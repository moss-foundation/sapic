import { languageService } from "@/domains/language/languageService";
import { ListLanguagesOutput } from "@repo/ipc";
import { useQuery } from "@tanstack/react-query";

export const USE_LIST_LANGUAGES_QUERY_KEY = "listLanguages";

export const useListLanguages = () => {
  return useQuery<ListLanguagesOutput, Error>({
    queryKey: [USE_LIST_LANGUAGES_QUERY_KEY],
    queryFn: languageService.listLanguages,
  });
};

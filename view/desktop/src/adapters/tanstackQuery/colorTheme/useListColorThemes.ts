import { themeService } from "@/domains/theme/themeService";
import { ListColorThemesOutput } from "@repo/ipc";
import { useQuery } from "@tanstack/react-query";

export const USE_LIST_COLOR_THEMES_QUERY_KEY = "listColorThemes";

export const useListColorThemes = () => {
  return useQuery<ListColorThemesOutput, Error>({
    queryKey: [USE_LIST_COLOR_THEMES_QUERY_KEY],
    queryFn: themeService.listColorThemes,
  });
};

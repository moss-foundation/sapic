import { AppService } from "@/lib/services";
import { sortObjectsByOrder } from "@/utils/sortObjectsByOrder";
import { ListColorThemesOutput } from "@repo/ipc";
import { useQuery } from "@tanstack/react-query";

export const USE_LIST_COLOR_THEMES_QUERY_KEY = "listColorThemes";

const listColorThemesFn = async (): Promise<ListColorThemesOutput> => {
  const result = await AppService.listColorThemes();

  if (result.status === "error") {
    throw new Error(String(result.error));
  }

  return result.data;
};

export const useListColorThemes = () => {
  return useQuery<ListColorThemesOutput, Error>({
    queryKey: [USE_LIST_COLOR_THEMES_QUERY_KEY],
    queryFn: async () => {
      const data = await listColorThemesFn();
      return sortObjectsByOrder(data);
    },
  });
};

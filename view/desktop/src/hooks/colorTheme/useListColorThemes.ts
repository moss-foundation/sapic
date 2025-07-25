import { invokeTauriIpc } from "@/lib/backend/tauri";
import { ListColorThemesOutput } from "@repo/moss-app";
import { useQuery } from "@tanstack/react-query";

export const USE_LIST_COLOR_THEMES_QUERY_KEY = "listColorThemes";

const listColorThemesFn = async (): Promise<ListColorThemesOutput> => {
  const result = await invokeTauriIpc<ListColorThemesOutput>("list_color_themes");

  if (result.status === "error") {
    throw new Error(String(result.error));
  }

  return result.data;
};

export const useListColorThemes = () => {
  return useQuery<ListColorThemesOutput, Error>({
    queryKey: [USE_LIST_COLOR_THEMES_QUERY_KEY],
    queryFn: listColorThemesFn,
    staleTime: 30 * 60 * 1000, // 30 minutes
    gcTime: 60 * 60 * 1000, // 1 hour
  });
};

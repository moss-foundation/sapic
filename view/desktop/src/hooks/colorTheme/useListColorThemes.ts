import { invokeTauriIpc } from "@/lib/backend/tauri";
import { ListColorThemesOutput } from "@repo/moss-app";
import { useQuery } from "@tanstack/react-query";

export const USE_LIST_COLOR_THEMES_QUERY_KEY = "listColorThemes";

const listThemesFn = async (): Promise<ListColorThemesOutput> => {
  const result = await invokeTauriIpc<ListColorThemesOutput>("list_color_themes");
  if (result.status === "error") {
    throw new Error(String(result.error));
  }

  return result.data;
};

export const useListColorThemes = () => {
  return useQuery<ListColorThemesOutput, Error>({
    queryKey: [USE_LIST_COLOR_THEMES_QUERY_KEY],
    queryFn: listThemesFn,
  });
};

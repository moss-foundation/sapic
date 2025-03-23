import { invokeTauriIpc } from "@/lib/backend/tauri";
import { ListColorThemesOutput } from "@repo/moss-theme";
import { useQuery } from "@tanstack/react-query";

const listThemesFn = async (): Promise<ListColorThemesOutput> => {
  const result = await invokeTauriIpc<ListColorThemesOutput>("list_color_themes");
  if (result.status === "error") {
    throw new Error(String(result.error));
  }

  return result.data;
};

export const useListColorThemes = () => {
  return useQuery<ListColorThemesOutput, Error>({
    queryKey: ["listColorThemes"],
    queryFn: listThemesFn,
  });
};

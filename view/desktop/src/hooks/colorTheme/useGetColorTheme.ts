import { invokeTauriIpc } from "@/lib/backend/tauri";
import { GetColorThemeInput, GetColorThemeOutput } from "@repo/moss-app";
import { useQuery } from "@tanstack/react-query";

export const USE_GET_COLOR_THEME_QUERY_KEY = "getColorTheme";

const getColorThemeFn = async (input: GetColorThemeInput): Promise<GetColorThemeOutput> => {
  const result = await invokeTauriIpc<GetColorThemeOutput>("get_color_theme", {
    input: input,
  });
  if (result.status === "error") {
    throw new Error(String(result.error));
  }

  return result.data;
};

export interface UseGetColorThemeParams {
  themeId: string;
  enabled?: boolean;
}

export const useGetColorTheme = ({ themeId, enabled = true }: UseGetColorThemeParams) => {
  return useQuery<GetColorThemeOutput, Error>({
    queryKey: [USE_GET_COLOR_THEME_QUERY_KEY, themeId],
    queryFn: () => getColorThemeFn({ id: themeId }),
    enabled: enabled && !!themeId,
    staleTime: 15 * 60 * 1000,
    gcTime: 30 * 60 * 1000,
  });
};

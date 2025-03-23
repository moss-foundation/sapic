import { listThemes } from "@/api/appearance";
import { invokeTauriIpc } from "@/lib/backend/tauri";
import { ColorThemeDescriptor, ListColorThemesOutput } from "@repo/moss-theme";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";

export const useListColorThemes = () => {
  return useQuery<ListColorThemesOutput, Error>({
    queryKey: ["listColorThemes"],
    queryFn: listThemes,
  });
};

export const changeColorTheme = async (descriptor: ColorThemeDescriptor): Promise<void> => {
  await invokeTauriIpc("change_color_theme", {
    descriptor: descriptor,
  });

  document.documentElement.setAttribute("data-theme", descriptor.mode);
};

export const useChangeColorTheme = () => {
  const queryClient = useQueryClient();
  return useMutation<void, Error, ColorThemeDescriptor>({
    mutationKey: ["changeColorTheme"],
    mutationFn: changeColorTheme,
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["getState"] });
    },
  });
};

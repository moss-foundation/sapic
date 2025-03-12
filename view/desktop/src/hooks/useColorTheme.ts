import { listThemes } from "@/api/appearance";
import { invokeTauriIpc } from "@/lib/backend/tauri";
import { ListThemesOutput, ThemeDescriptor } from "@repo/moss-theme";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";

export const useGetColorThemes = () => {
  return useQuery<ListThemesOutput, Error>({
    queryKey: ["getColorTheme"],
    queryFn: listThemes,
  });
};

export const changeColorTheme = async (descriptor: ThemeDescriptor): Promise<void> => {
  await invokeTauriIpc("change_color_theme", {
    descriptor: descriptor,
  });

  document.documentElement.setAttribute("data-theme", descriptor.mode);
};

export const useChangeColorTheme = () => {
  const queryClient = useQueryClient();
  return useMutation<void, Error, ThemeDescriptor>({
    mutationKey: ["changeColorTheme"],
    mutationFn: changeColorTheme,
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["getState"] });
    },
  });
};

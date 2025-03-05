import { getColorThemes } from "@/api/appearance";
import { invokeTauriIpc, IpcResult } from "@/lib/backend/tauri";
import { ListThemesOutput, ThemeDescriptor } from "@repo/moss-theme";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";

export const useGetColorThemes = () => {
  return useQuery<ListThemesOutput, Error>({
    queryKey: ["getColorTheme"],
    queryFn: getColorThemes,
  });
};

export const changeTheme = async (descriptor: ThemeDescriptor): Promise<void> => {
  await invokeTauriIpc("change_color_theme", {
    descriptor: descriptor,
  });
};

export const useChangeColorTheme = () => {
  const queryClient = useQueryClient();
  return useMutation<void, Error, ThemeDescriptor>({
    mutationKey: ["changeColorTheme"],
    mutationFn: changeTheme,
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["getState"] });
    },
  });
};

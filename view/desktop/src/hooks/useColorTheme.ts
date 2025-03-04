import { getColorThemes } from "@/api/appearance";
import { invokeMossCommand } from "@/lib/backend/platfrom";
import { invokeTauriIpc, IpcResult } from "@/lib/backend/tauri";
import { ListThemesOutput, ThemeDescriptor } from "@repo/moss-theme";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";

export const useGetColorThemes = () => {
  return useQuery<ListThemesOutput, Error>({
    queryKey: ["getColorTheme"],
    queryFn: getColorThemes,
  });
};

// const changeTheme = async (themeDescriptor: ThemeDescriptor): Promise<void> => {
//   await invokeMossCommand("workbench.changeColorTheme", {
//     themeDescriptor,
//   });
// };

export const changeTheme = async (id: string): Promise<void> => {
  await invokeTauriIpc("change_color_theme", {
    id: id,
  });
};

export const useChangeColorTheme = () => {
  const queryClient = useQueryClient();
  return useMutation<void, Error, string>({
    mutationKey: ["changeColorTheme"],
    mutationFn: changeTheme,
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["getState"] });
    },
  });
};

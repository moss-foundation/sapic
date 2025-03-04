import { getColorThemes } from "@/api/appearance";
import { invokeMossCommand } from "@/lib/backend/platfrom";
import { invokeTauriIpc, IpcResult } from "@/lib/backend/tauri";
import { ThemeDescriptor } from "@repo/moss-theme";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";

export const useGetColorThemes = () => {
  return useQuery<ThemeDescriptor[], Error>({
    queryKey: ["getColorTheme"],
    queryFn: getColorThemes,
  });
};

// const changeTheme = async (themeDescriptor: ThemeDescriptor): Promise<void> => {
//   await invokeMossCommand("workbench.changeColorTheme", {
//     themeDescriptor,
//   });
// };

export const changeTheme = async (id: string): Promise<IpcResult<void, string>> => {
  return await invokeTauriIpc("change_color_theme", {
    id: id,
  });
};

export const useChangeColorTheme = () => {
  const queryClient = useQueryClient();

  return useMutation<void, Error, ThemeDescriptor>({
    mutationKey: ["changeColorTheme"],
    mutationFn: async (id: string) => {
      const result = await changeTheme(id);
      if (result.status === "ok") {
        return result.data;
      } else if (result.status === "error") {
        throw result.error;
      }
      throw new Error("Unexpected response status");
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["getState"] });
    },
  });
};

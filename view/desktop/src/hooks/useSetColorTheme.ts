import { invokeTauriIpc } from "@/lib/backend/tauri";
import { SetColorThemeInput } from "@repo/moss-state";
import { useMutation, useQueryClient } from "@tanstack/react-query";

const setColorThemeFn = async (input: SetColorThemeInput): Promise<void> => {
  const result = await invokeTauriIpc("set_color_theme", {
    input: input,
  });
  if (result.status === "error") {
    throw new Error(String(result.error));
  }

  document.documentElement.setAttribute("data-theme", input.themeInfo.mode);
};

export const useSetColorTheme = () => {
  const queryClient = useQueryClient();
  return useMutation<void, Error, SetColorThemeInput>({
    mutationKey: ["setColorTheme"],
    mutationFn: setColorThemeFn,
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["getState"] });
    },
  });
};

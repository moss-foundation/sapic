import { USE_DESCRIBE_APP_QUERY_KEY } from "@/hooks/useDescribeApp";
import { invokeTauriIpc } from "@/lib/backend/tauri";
import { SetColorThemeInput } from "@repo/moss-app";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { USE_DESCRIBE_APP_STATE_QUERY_KEY } from "../useDescribeAppState";

export const USE_SET_COLOR_THEME_MUTATION_KEY = "setColorTheme";

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
    mutationKey: [USE_SET_COLOR_THEME_MUTATION_KEY],
    mutationFn: setColorThemeFn,
    onSuccess: () => {
      //TODO this should update cache of USE_DESCRIBE_APP
      queryClient.invalidateQueries({ queryKey: [USE_DESCRIBE_APP_STATE_QUERY_KEY] });
      queryClient.invalidateQueries({ queryKey: [USE_DESCRIBE_APP_QUERY_KEY] });
    },
  });
};

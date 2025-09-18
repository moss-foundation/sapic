import { USE_DESCRIBE_APP_QUERY_KEY } from "@/hooks/app/useDescribeApp";
import { invokeTauriIpc } from "@/lib/backend/tauri";
import { DescribeAppOutput, SetColorThemeInput } from "@repo/moss-app";
import { useMutation, useQueryClient } from "@tanstack/react-query";

export const USE_SET_COLOR_THEME_MUTATION_KEY = "setColorTheme";

const setColorThemeFn = async (input: SetColorThemeInput): Promise<void> => {
  const result = await invokeTauriIpc("set_color_theme", {
    input,
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
    onSuccess: (_, input) => {
      queryClient.setQueryData([USE_DESCRIBE_APP_QUERY_KEY], (old: DescribeAppOutput) => {
        return {
          ...old,
          configuration: {
            ...old.configuration,
            contents: {
              ...old.configuration.contents,
              colorTheme: input.themeInfo.identifier,
            },
          },
        };
      });
    },
  });
};

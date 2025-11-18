import { AppService } from "@/lib/services";
import { GetColorThemeInput, GetColorThemeOutput } from "@repo/ipc";
import { UndefinedInitialDataOptions, useQuery } from "@tanstack/react-query";

export const USE_DESCRIBE_COLOR_THEME_QUERY_KEY = "describeColorTheme";

const describeColorThemeFn = async (input: GetColorThemeInput): Promise<GetColorThemeOutput> => {
  const result = await AppService.describeColorTheme(input.id);

  if (result.status === "error") {
    throw new Error(String(result.error));
  }

  return result.data;
};

export interface UseGetColorThemeParams
  extends Omit<
    UndefinedInitialDataOptions<GetColorThemeOutput, Error, GetColorThemeOutput, readonly unknown[]>,
    "queryKey" | "queryFn"
  > {
  themeId: string;
}

export const useDescribeColorTheme = ({ themeId, enabled = true, ...options }: UseGetColorThemeParams) => {
  return useQuery<GetColorThemeOutput, Error>({
    queryKey: [USE_DESCRIBE_COLOR_THEME_QUERY_KEY, themeId],
    queryFn: () => describeColorThemeFn({ id: themeId }),
    enabled: enabled && !!themeId,
    ...options,
  });
};

import { themeService } from "@/domains/theme/themeService";
import { GetColorThemeOutput } from "@repo/ipc";
import { UndefinedInitialDataOptions, useQuery } from "@tanstack/react-query";

export const USE_DESCRIBE_COLOR_THEME_QUERY_KEY = "describeColorTheme";

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
    queryFn: () => themeService.describeColorTheme(themeId),
    enabled: enabled && !!themeId,
    ...options,
  });
};

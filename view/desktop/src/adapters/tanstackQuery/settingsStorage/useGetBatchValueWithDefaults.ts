import { settingsStorageService } from "@/app/services/settingsStorage";
import { useQuery } from "@tanstack/react-query";

export const USE_GET_BATCH_SETTINGS_VALUE_WITH_DEFAULTS_QUERY_KEY = "application.settings" as const;

export const useGetBatchValueWithDefaults = <T extends Record<string, object | boolean | number | string | null>>(
  keys: Array<keyof T>,
  defaults: { [K in keyof T]: T[K] }
) => {
  return useQuery({
    queryKey: [USE_GET_BATCH_SETTINGS_VALUE_WITH_DEFAULTS_QUERY_KEY, keys],
    queryFn: () => settingsStorageService.batchGetValue<T>(keys, defaults),
  });
};

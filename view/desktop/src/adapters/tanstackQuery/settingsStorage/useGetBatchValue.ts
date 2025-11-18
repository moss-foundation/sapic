import { settingsStorageService } from "@/app/services/settingsStorage";
import { useQuery } from "@tanstack/react-query";

export const USE_GET_BATCH_VALUE_QUERY_KEY = "application.settings" as const;

export const useGetBatchValue = <T extends Record<string, object | boolean | number | string | null>>(
  keys: Array<keyof T>
) => {
  return useQuery({
    queryKey: [USE_GET_BATCH_VALUE_QUERY_KEY, keys],
    queryFn: () => settingsStorageService.batchGetValue<T>(keys),
  });
};

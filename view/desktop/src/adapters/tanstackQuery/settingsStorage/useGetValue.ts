import { settingsStorageService } from "@/app/services/settingsStorage";
import { useQuery } from "@tanstack/react-query";

import { USE_GET_BATCH_VALUE_QUERY_KEY } from "./useGetBatchValue";

export const useGetValue = <T extends object | boolean | number | string | null>(key: string) => {
  return useQuery<T | null, Error>({
    queryKey: [USE_GET_BATCH_VALUE_QUERY_KEY, key],
    queryFn: () => settingsStorageService.getValue<T>(key),
  });
};

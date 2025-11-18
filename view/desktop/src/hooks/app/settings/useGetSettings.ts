import { settingsStorageService } from "@/app/services/settingsStorage";
import { useQuery } from "@tanstack/react-query";

import { SETTINGS_QUERY_KEY } from "../useSyncSettings";

export const useGetSettings = <T extends Record<string, object | boolean | number | string | null>>(
  keys: Array<keyof T>
) => {
  return useQuery({
    queryKey: [SETTINGS_QUERY_KEY],
    queryFn: async () => {
      return await settingsStorageService.batchGetValue<T>(keys);
    },
  });
};

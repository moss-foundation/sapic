import { settingsStorageService } from "@/app/services/settingsStorage";
import { UpdateValueOutput } from "@repo/settings-storage";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { SETTINGS_QUERY_KEY } from "../useSyncSettings";

export const useUpdateSettingsValue = <T extends object | boolean | number | string | null>() => {
  const queryClient = useQueryClient();

  return useMutation<UpdateValueOutput, Error, { key: string; value: T }>({
    mutationFn: async ({ key, value }: { key: string; value: T }) => {
      return await settingsStorageService.updateValue(
        key,
        value as unknown as object | boolean | number | string | null
      );
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: [SETTINGS_QUERY_KEY] });
    },
  });
};

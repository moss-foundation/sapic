import { settingsStorageService } from "@/app/services/settingsStorage";
import { TypedValue } from "@/app/services/types";
import { UpdateValueOutput } from "@repo/settings-storage";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { USE_GET_BATCH_SETTINGS_VALUE_WITH_DEFAULTS_QUERY_KEY } from "./useGetBatchSettingsValueWithDefaults";

export const useUpdateSettingsValue = <T extends object | boolean | number | string | null>() => {
  const queryClient = useQueryClient();

  return useMutation<UpdateValueOutput, Error, { key: string; value: TypedValue<T> }>({
    mutationFn: async ({ key, value }) => {
      return await settingsStorageService.updateValue(key, value);
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: [USE_GET_BATCH_SETTINGS_VALUE_WITH_DEFAULTS_QUERY_KEY] });
    },
  });
};

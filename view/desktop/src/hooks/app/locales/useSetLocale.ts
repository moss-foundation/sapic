import i18next from "@/app/i18n";
import { invokeTauriIpc } from "@/lib/backend/tauri";
import { DescribeAppStateOutput, SetLocaleInput } from "@repo/moss-app";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { USE_DESCRIBE_APP_STATE_QUERY_KEY } from "../useDescribeAppState";
import { getLocaleFn } from "./useGetLocale";

export const USE_SET_LOCALE_MUTATION_KEY = "setLocale";

const setLocaleFn = async (input: SetLocaleInput): Promise<void> => {
  const result = await invokeTauriIpc("set_locale", {
    input,
  });

  if (result.status === "error") {
    throw new Error(String(result.error));
  }
};

export const useSetLocale = () => {
  const queryClient = useQueryClient();
  const mutation = useMutation<void, Error, SetLocaleInput>({
    mutationKey: [USE_SET_LOCALE_MUTATION_KEY],
    mutationFn: setLocaleFn,
    onSuccess: async (_, input) => {
      queryClient.setQueryData([USE_DESCRIBE_APP_STATE_QUERY_KEY], (old: DescribeAppStateOutput) => {
        return {
          ...old,
          preferences: {
            ...old.preferences,
            locale: input.localeInfo,
          },
        };
      });

      await i18next.changeLanguage(input.localeInfo.code).catch(console.error);
    },
  });

  const applyLocaleById = async (id: string) => {
    const result = await getLocaleFn(id);
    i18next.changeLanguage(result.code);
  };

  return { ...mutation, applyLocaleById };
};

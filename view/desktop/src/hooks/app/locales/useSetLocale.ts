import i18next from "@/app/i18n";
import { USE_DESCRIBE_APP_QUERY_KEY } from "@/hooks/app/useDescribeApp";
import { invokeTauriIpc } from "@/lib/backend/tauri";
import { DescribeAppOutput, SetLocaleInput } from "@repo/moss-app";
import { useMutation, useQueryClient } from "@tanstack/react-query";

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
      queryClient.setQueryData([USE_DESCRIBE_APP_QUERY_KEY], (old: DescribeAppOutput) => {
        return {
          ...old,
          configuration: {
            ...old.configuration,
            contents: {
              ...old.configuration.contents,
              locale: input.localeInfo.identifier,
            },
          },
        };
      });

      await i18next.changeLanguage(input.localeInfo.code).catch(console.error);
    },
  });

  const setLocaleLocally = async (input: SetLocaleInput) => {
    queryClient.setQueryData([USE_DESCRIBE_APP_QUERY_KEY], (old: DescribeAppOutput) => {
      return {
        ...old,
        configuration: {
          ...old.configuration,
          contents: {
            ...old.configuration.contents,
            locale: input.localeInfo.identifier,
          },
        },
      };
    });
    await i18next.changeLanguage(input.localeInfo.code).catch(console.error);
  };

  return { ...mutation, setLocaleLocally };
};

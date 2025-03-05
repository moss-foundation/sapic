import { getLanguagePacks } from "@/api/appearance";
import { invokeTauriIpc } from "@/lib/backend/tauri";
import { ListLocalesOutput, LocaleDescriptor } from "@repo/moss-nls";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";

export const useGetLanguagePacks = () => {
  return useQuery<ListLocalesOutput, Error>({
    queryKey: ["getLanguagePacks"],
    queryFn: getLanguagePacks,
  });
};

export const changeLanguagePack = async (descriptor: LocaleDescriptor): Promise<void> => {
  await invokeTauriIpc("change_language_pack", {
    descriptor: descriptor,
  });
};

export const useChangeLanguagePack = () => {
  const queryClient = useQueryClient();
  return useMutation<void, Error, LocaleDescriptor>({
    mutationKey: ["changeLanguagePack"],
    mutationFn: changeLanguagePack,
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["getState"] });
    },
  });
};

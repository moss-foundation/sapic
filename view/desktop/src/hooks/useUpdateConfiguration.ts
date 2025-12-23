import { AppService } from "@/lib/services";
import { UpdateConfigurationInput } from "@repo/window";
import { useMutation } from "@tanstack/react-query";

export const useUpdateConfiguration = () => {
  return useMutation({
    mutationFn: (configuration: UpdateConfigurationInput) => {
      return AppService.updateConfiguration(configuration);
    },
  });
};

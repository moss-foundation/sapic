import { userService } from "@/domains/user/userService";
import { USE_DESCRIBE_APP_QUERY_KEY } from "@/hooks";
import { UpdateProfileInput, UpdateProfileOutput } from "@repo/window";
import { useMutation, useQueryClient } from "@tanstack/react-query";

export const useUpdateProfile = () => {
  const queryClient = useQueryClient();
  return useMutation<UpdateProfileOutput, Error, UpdateProfileInput>({
    mutationFn: userService.updateProfile,
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: [USE_DESCRIBE_APP_QUERY_KEY] });
    },
  });
};

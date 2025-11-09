import { sharedStorageService } from "@/lib/services/sharedStorage";
import { useMutation } from "@tanstack/react-query";

export const USE_REMOVE_LAYOUT_MUTATION_KEY = "removeLayout";

interface UseRemoveLayoutProps {
  workspaceId?: string;
}

const mutationFn = async ({ workspaceId }: UseRemoveLayoutProps) => {
  return await sharedStorageService.removeItem("layout", workspaceId);
};

export const useRemoveLayout = () => {
  return useMutation({
    mutationKey: [USE_REMOVE_LAYOUT_MUTATION_KEY],
    mutationFn: ({ workspaceId }: UseRemoveLayoutProps) => mutationFn({ workspaceId }),
  });
};

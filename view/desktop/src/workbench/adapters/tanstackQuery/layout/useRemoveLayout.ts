import { layoutService } from "@/workbench/domains/layout/service";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { USE_GET_LAYOUT_QUERY_KEY } from "./useGetLayout";

export const USE_REMOVE_LAYOUT_MUTATION_KEY = "removeLayout";

export const useRemoveLayout = () => {
  const queryClient = useQueryClient();

  return useMutation<void, Error, string>({
    mutationKey: [USE_REMOVE_LAYOUT_MUTATION_KEY],
    mutationFn: (workspaceId: string) => layoutService.removeLayout(workspaceId),
    onSuccess: (_, workspaceId: string) => {
      queryClient.removeQueries({ queryKey: [USE_GET_LAYOUT_QUERY_KEY, workspaceId] });
    },
  });
};

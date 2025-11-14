import { layoutService, LayoutStateInput } from "@/workbench/domains/layout/service";
import { useMutation } from "@tanstack/react-query";

export const USE_UPDATE_LAYOUT_MUTATION_KEY = "updateLayout";

export const useUpdateLayout = () => {
  return useMutation({
    mutationKey: [USE_UPDATE_LAYOUT_MUTATION_KEY],
    mutationFn: async ({ input, workspaceId }: { input: LayoutStateInput; workspaceId: string }) => {
      return await layoutService.updateLayout(input, workspaceId);
    },
    onSuccess: () => {
      console.log("useUpdateLayout success");
    },
    onError: (error) => {
      console.error("useUpdateLayout error: ", error);
    },
  });
};

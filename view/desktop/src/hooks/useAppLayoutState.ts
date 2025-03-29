import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";

export interface AppLayoutState {
  activeSidebar: "left" | "right" | "none";
}

let AppLayoutState = {
  activeSidebar: "left",
};

const getAppLayoutState = async () => {
  await new Promise((resolve) => setTimeout(resolve, 0));
  return AppLayoutState as AppLayoutState;
};

export const useGetAppLayoutState = () => {
  return useQuery<AppLayoutState, Error>({
    queryKey: ["getAppLayoutState"],
    queryFn: getAppLayoutState,
  });
};

export const useChangeAppLayoutState = () => {
  const queryClient = useQueryClient();

  return useMutation<AppLayoutState, Error, AppLayoutState>({
    mutationFn: async (newLayout) => {
      await new Promise((resolve) => setTimeout(resolve, 50));

      AppLayoutState = newLayout;
      return newLayout;
    },
    onSuccess() {
      queryClient.invalidateQueries({ queryKey: ["getAppLayoutState"] });
    },
  });
};

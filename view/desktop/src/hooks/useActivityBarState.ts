import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";

export type ActivityBarPosition = "top" | "bottom" | "hidden" | "default";

export interface ActivityBarState {
  position: ActivityBarPosition;
  groupOrder: string[];
}

let ActivityBarState: ActivityBarState = {
  position: "default",
  groupOrder: [],
};

const getActivityBarState = async () => {
  await new Promise((resolve) => setTimeout(resolve, 0));
  return { ...ActivityBarState };
};

export const useGetActivityBarState = () => {
  return useQuery<ActivityBarState, Error>({
    queryKey: ["getActivityBar"],
    queryFn: getActivityBarState,
  });
};

export const useChangeActivityBarState = () => {
  const queryClient = useQueryClient();
  return useMutation<ActivityBarState, Error, Partial<ActivityBarState>>({
    mutationFn: async (newState) => {
      await new Promise((resolve) => setTimeout(resolve, 50));

      ActivityBarState = {
        ...ActivityBarState,
        ...newState,
      };

      return { ...ActivityBarState };
    },
    onSuccess() {
      queryClient.invalidateQueries({ queryKey: ["getActivityBar"] });
    },
  });
};

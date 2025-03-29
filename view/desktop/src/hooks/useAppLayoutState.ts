import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";

// FIXME: remove mock data
export interface AppLayoutState {
  activeSidebar: "left" | "right" | "none";
}

export const USE_APP_LAYOUT_STATE_QUERY_KEY = "appLayoutState";
export const USE_CHANGE_APP_LAYOUT_STATE_MUTATION_KEY = "changeAppLayoutState";

let AppLayoutState = {
  activeSidebar: "left",
};

const getAppLayoutStateFn = async (): Promise<AppLayoutState> => {
  await new Promise((resolve) => setTimeout(resolve, 0));
  return AppLayoutState as AppLayoutState;
};

const changeAppLayoutStateFn = async (newLayout: AppLayoutState): Promise<AppLayoutState> => {
  await new Promise((resolve) => setTimeout(resolve, 50));

  AppLayoutState = newLayout;
  return newLayout;
};

export const useGetAppLayoutState = () => {
  return useQuery<AppLayoutState, Error>({
    queryKey: [USE_APP_LAYOUT_STATE_QUERY_KEY],
    queryFn: getAppLayoutStateFn,
  });
};

export const useChangeAppLayoutState = () => {
  const queryClient = useQueryClient();

  return useMutation<AppLayoutState, Error, AppLayoutState>({
    mutationKey: [USE_CHANGE_APP_LAYOUT_STATE_MUTATION_KEY],
    mutationFn: changeAppLayoutStateFn,
    onSuccess() {
      queryClient.invalidateQueries({ queryKey: [USE_APP_LAYOUT_STATE_QUERY_KEY] });
    },
  });
};
